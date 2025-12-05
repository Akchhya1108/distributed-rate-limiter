use distributed_rate_limiter::{RateLimiter, RateLimitConfig};
use distributed_rate_limiter::algorithms::TokenBucket;
use std::time::Duration;

#[tokio::main]
async fn main() {
    println!("🚀 Distributed Rate Limiter - Phase 1");
    println!("=====================================\n");
    
    // Create a rate limiter: 5 requests per second
    let config = RateLimitConfig::per_second(5);
    let mut limiter = TokenBucket::new(config);
    
    println!("Testing Token Bucket Algorithm");
    println!("Config: 5 requests per second\n");
    
    // Simulate requests
    for i in 1..=7 {
        let allowed = limiter.allow_request("user123").unwrap();
        println!("Request {}: {}", i, if allowed { "✅ ALLOWED" } else { "❌ BLOCKED" });
    }
    
    println!("\n⏳ Waiting 1 second for token refill...\n");
    tokio::time::sleep(Duration::from_secs(1)).await;
    
    // Try again after refill
    for i in 1..=3 {
        let allowed = limiter.allow_request("user123").unwrap();
        println!("Request {}: {}", i, if allowed { "✅ ALLOWED" } else { "❌ BLOCKED" });
    }
    
    println!("\n✨ Phase 1 Complete! Token Bucket algorithm working!");
}