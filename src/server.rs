use actix_web::{get, post, web, App, HttpResponse, HttpServer, Result};
use actix_cors::Cors;
use actix_files as fs;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use std::time::Instant;

use distributed_rate_limiter::{RateLimiter, RateLimitConfig, AlgorithmType};
use distributed_rate_limiter::algorithms::*;
use distributed_rate_limiter::metrics;

#[derive(Serialize)]
struct MetricsResponse {
    total: u64,
    allowed: u64,
    blocked: u64,
    allow_rate: f64,
}

#[derive(Deserialize)]
struct TestRequest {
    algorithm: String,
    max_requests: u64,
    window_seconds: u64,
    num_requests: u32,
}

#[derive(Serialize)]
struct TestResponse {
    allowed: u32,
    blocked: u32,
    duration_ms: f64,
    requests_per_sec: f64,
    results: Vec<bool>,
}

#[get("/")]
async fn index() -> Result<fs::NamedFile> {
    Ok(fs::NamedFile::open("web/index.html")?)
}

#[get("/api/metrics")]
async fn get_metrics() -> Result<HttpResponse> {
    let total = metrics::REQUESTS_TOTAL.get();
    let allowed = metrics::REQUESTS_ALLOWED.get();
    let blocked = metrics::REQUESTS_BLOCKED.get();
    
    let allow_rate = if total > 0 {
        (allowed as f64 / total as f64) * 100.0
    } else {
        0.0
    };
    
    Ok(HttpResponse::Ok().json(MetricsResponse {
        total,
        allowed,
        blocked,
        allow_rate,
    }))
}

#[post("/api/test")]
async fn test_rate_limiter(req: web::Json<TestRequest>) -> Result<HttpResponse> {
    let config = RateLimitConfig::new(
        req.max_requests,
        std::time::Duration::from_secs(req.window_seconds),
    );
    
    let start = Instant::now();
    let mut allowed_count = 0;
    let mut blocked_count = 0;
    let mut results = Vec::new();
    
    match req.algorithm.as_str() {
        "token_bucket" => {
            let mut limiter = TokenBucket::new(config);
            for _ in 0..req.num_requests {
                match limiter.allow_request("test_user") {
                    Ok(true) => {
                        allowed_count += 1;
                        results.push(true);
                    }
                    Ok(false) => {
                        blocked_count += 1;
                        results.push(false);
                    }
                    Err(_) => {}
                }
            }
        }
        "leaky_bucket" => {
            let mut limiter = LeakyBucket::new(config);
            for _ in 0..req.num_requests {
                match limiter.allow_request("test_user") {
                    Ok(true) => {
                        allowed_count += 1;
                        results.push(true);
                    }
                    Ok(false) => {
                        blocked_count += 1;
                        results.push(false);
                    }
                    Err(_) => {}
                }
            }
        }
        "fixed_window" => {
            let mut limiter = FixedWindow::new(config);
            for _ in 0..req.num_requests {
                match limiter.allow_request("test_user") {
                    Ok(true) => {
                        allowed_count += 1;
                        results.push(true);
                    }
                    Ok(false) => {
                        blocked_count += 1;
                        results.push(false);
                    }
                    Err(_) => {}
                }
            }
        }
        "sliding_window" => {
            let mut limiter = SlidingWindow::new(config);
            for _ in 0..req.num_requests {
                match limiter.allow_request("test_user") {
                    Ok(true) => {
                        allowed_count += 1;
                        results.push(true);
                    }
                    Ok(false) => {
                        blocked_count += 1;
                        results.push(false);
                    }
                    Err(_) => {}
                }
            }
        }
        _ => {}
    }
    
    let duration = start.elapsed();
    let duration_ms = duration.as_secs_f64() * 1000.0;
    let requests_per_sec = req.num_requests as f64 / duration.as_secs_f64();
    
    Ok(HttpResponse::Ok().json(TestResponse {
        allowed: allowed_count,
        blocked: blocked_count,
        duration_ms,
        requests_per_sec,
        results,
    }))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize metrics
    metrics::init_metrics();
    
    println!("🚀 Starting Rate Limiter Web Dashboard");
    println!("📊 Dashboard: http://localhost:8080");
    println!("📈 Metrics API: http://localhost:8080/api/metrics");
    println!("\nPress Ctrl+C to stop\n");
    
    HttpServer::new(|| {
        let cors = Cors::permissive();
        
        App::new()
            .wrap(cors)
            .service(index)
            .service(get_metrics)
            .service(test_rate_limiter)
            .service(fs::Files::new("/", "web").index_file("index.html"))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}