use distributed_rate_limiter::{RateLimiter, RateLimitConfig, AlgorithmType};
use distributed_rate_limiter::algorithms::*;
use distributed_rate_limiter::redis_limiter::RedisRateLimiter;
use std::time::Duration;

#[tokio::main]
async fn main() {
    println!("🚀 Distributed Rate Limiter - Phase 3");
    println!("======================================\n");
    
    let config = RateLimitConfig::per_second(3);
    
    println!("Testing all 4 rate limiting algorithms");
    println!("Config: 3 requests per second\n");
    
    // Test 1: Token Bucket
    println!("1️⃣  TOKEN BUCKET");
    println!("   (Tokens refill over time)");
    println!("   ----------------------------");
    test_algorithm("Token Bucket", TokenBucket::new(config.clone())).await;
    
    println!("\n");
    
    // Test 2: Leaky Bucket
    println!("2️⃣  LEAKY BUCKET");
    println!("   (Queue that leaks at constant rate)");
    println!("   ----------------------------");
    test_algorithm("Leaky Bucket", LeakyBucket::new(config.clone())).await;
    
    println!("\n");
    
    // Test 3: Fixed Window
    println!("3️⃣  FIXED WINDOW");
    println!("   (Counter resets every window)");
    println!("   ----------------------------");
    test_algorithm("Fixed Window", FixedWindow::new(config.clone())).await;
    
    println!("\n");
    
    // Test 4: Sliding Window
    println!("4️⃣  SLIDING WINDOW");
    println!("   (Tracks exact timestamps)");
    println!("   ----------------------------");
    test_algorithm("Sliding Window", SlidingWindow::new(config.clone())).await;
    
    println!("\n");
    
    // Test 5: Redis Distributed
    println!("5️⃣  REDIS DISTRIBUTED");
    println!("   (Token Bucket with Redis backend)");
    println!("   ----------------------------");
    test_redis().await;
    
    println!("\n");
    println!("✨ Phase 3 Complete! All 4 algorithms implemented!");
    println!("\n📊 Algorithm Comparison:");
    println!("   • Token Bucket:    Best for smooth traffic, allows bursts");
    println!("   • Leaky Bucket:    Smooths out bursts, constant output rate");
    println!("   • Fixed Window:    Simple, but burst at window boundaries");
    println!("   • Sliding Window:  Most accurate, higher memory usage");
}

async fn test_algorithm<T: RateLimiter>(name: &str, mut limiter: T) {
    for i in 1..=5 {
        let allowed = limiter.allow_request("user_test").unwrap();
        println!("   Request {}: {}", i, if allowed { "✅ ALLOWED" } else { "❌ BLOCKED" });
    }
    
    println!("   ⏳ Waiting 1 second...");
    tokio::time::sleep(Duration::from_secs(1)).await;
    
    for i in 1..=2 {
        let allowed = limiter.allow_request("user_test").unwrap();
        println!("   Request {}: {}", i, if allowed { "✅ ALLOWED" } else { "❌ BLOCKED" });
    }
}

async fn test_redis() {
    let redis_url = "redis://127.0.0.1/";
    let config = RateLimitConfig::per_second(3);
    
    match RedisRateLimiter::new(redis_url, config) {
        Ok(mut limiter) => {
            println!("   ✅ Connected to Redis\n");
            limiter.reset("user_redis");
            
            for i in 1..=5 {
                match limiter.allow_request("user_redis") {
                    Ok(allowed) => {
                        println!("   Request {}: {}", i, if allowed { "✅ ALLOWED" } else { "❌ BLOCKED" });
                    }
                    Err(_) => {
                        println!("   ⚠️  Redis connection lost during test");
                        break;
                    }
                }
            }
            
            limiter.reset("user_redis");
        }
        Err(_) => {
            println!("   ⚠️  Redis not available (start Redis server to test distributed mode)");
            println!("   💡 Run: redis-server.exe");
        }
    }
}