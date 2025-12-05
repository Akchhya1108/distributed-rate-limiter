use distributed_rate_limiter::{RateLimiter, RateLimitConfig};
use distributed_rate_limiter::algorithms::TokenBucket;
use distributed_rate_limiter::redis_limiter::RedisRateLimiter;
use std::time::Duration;

#[tokio::main]
async fn main() {
    println!("🚀 Distributed Rate Limiter - Phase 2");
    println!("======================================\n");
    
    // Test 1: In-Memory Token Bucket
    println!("📦 Test 1: In-Memory Token Bucket");
    println!("----------------------------------");
    test_in_memory().await;
    
    println!("\n");
    
    // Test 2: Redis-backed Distributed Limiter
    println!("🔴 Test 2: Redis Distributed Limiter");
    println!("-------------------------------------");
    test_redis().await;
    
    println!("\n✨ Phase 2 Complete! Redis integration working!");
}

async fn test_in_memory() {
    let config = RateLimitConfig::per_second(3);
    let mut limiter = TokenBucket::new(config);
    
    println!("Config: 3 requests per second (in-memory)\n");
    
    for i in 1..=5 {
        let allowed = limiter.allow_request("user_local").unwrap();
        println!("  Request {}: {}", i, if allowed { "✅ ALLOWED" } else { "❌ BLOCKED" });
    }
}

async fn test_redis() {
    // Try to connect to Redis
    let redis_url = "redis://127.0.0.1/";
    let config = RateLimitConfig::per_second(3);
    
    match RedisRateLimiter::new(redis_url, config) {
        Ok(mut limiter) => {
            println!("✅ Connected to Redis at {}\n", redis_url);
            println!("Config: 3 requests per second (distributed)\n");
            
            // Clean slate
            limiter.reset("user_distributed");
            
            for i in 1..=5 {
                let allowed = limiter.allow_request("user_distributed").unwrap();
                println!("  Request {}: {}", i, if allowed { "✅ ALLOWED" } else { "❌ BLOCKED" });
            }
            
            println!("\n⏳ Waiting 1 second for refill...\n");
            tokio::time::sleep(Duration::from_secs(1)).await;
            
            for i in 1..=2 {
                let allowed = limiter.allow_request("user_distributed").unwrap();
                println!("  Request {}: {}", i, if allowed { "✅ ALLOWED" } else { "❌ BLOCKED" });
            }
            
            // Test circuit breaker
            println!("\n🔧 Testing circuit breaker pattern:");
            let result = limiter.check_with_fallback("user_distributed");
            println!("  Fallback test: {:?}", result);
            
            // Cleanup
            limiter.reset("user_distributed");
        }
        Err(e) => {
            println!("❌ Could not connect to Redis: {}", e);
            println!("   Make sure Redis is running!");
            println!("   Start it with: redis-server.exe");
        }
    }
}