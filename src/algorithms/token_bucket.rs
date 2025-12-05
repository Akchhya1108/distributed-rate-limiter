use crate::{RateLimiter, RateLimitConfig, RateLimitError, Result};
use std::collections::HashMap;
use std::time::{Duration, Instant};

#[derive(Debug)]
struct BucketState {
    tokens: f64,
    last_refill: Instant,
}

/// Token Bucket algorithm implementation
/// Tokens are added at a constant rate, requests consume tokens
pub struct TokenBucket {
    config: RateLimitConfig,
    buckets: HashMap<String, BucketState>,
    refill_rate: f64, // tokens per second
}

impl TokenBucket {
    pub fn new(config: RateLimitConfig) -> Self {
        let refill_rate = config.max_requests as f64 / config.window.as_secs_f64();
        
        Self {
            config,
            buckets: HashMap::new(),
            refill_rate,
        }
    }
    
    fn refill_tokens(&mut self, key: &str) -> &mut BucketState {
        let now = Instant::now();
        let max_tokens = self.config.max_requests as f64;
        
        let bucket = self.buckets.entry(key.to_string()).or_insert(BucketState {
            tokens: max_tokens,
            last_refill: now,
        });
        
        // Calculate tokens to add based on time elapsed
        let elapsed = now.duration_since(bucket.last_refill).as_secs_f64();
        let tokens_to_add = elapsed * self.refill_rate;
        
        // Add tokens but don't exceed max capacity
        bucket.tokens = (bucket.tokens + tokens_to_add).min(max_tokens);
        bucket.last_refill = now;
        
        bucket
    }
}

impl RateLimiter for TokenBucket {
    fn allow_request(&mut self, key: &str) -> Result<bool> {
        let bucket = self.refill_tokens(key);
        
        if bucket.tokens >= 1.0 {
            bucket.tokens -= 1.0;
            Ok(true)
        } else {
            Ok(false)
        }
    }
    
    fn reset(&mut self, key: &str) {
        self.buckets.remove(key);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread::sleep;
    
    #[test]
    fn test_token_bucket_allows_requests() {
        let config = RateLimitConfig::per_second(5);
        let mut limiter = TokenBucket::new(config);
        
        // Should allow 5 requests immediately
        for _ in 0..5 {
            assert!(limiter.allow_request("user1").unwrap());
        }
        
        // 6th request should be denied
        assert!(!limiter.allow_request("user1").unwrap());
    }
    
    #[test]
    fn test_token_bucket_refills() {
        let config = RateLimitConfig::per_second(2);
        let mut limiter = TokenBucket::new(config);
        
        // Use up tokens
        assert!(limiter.allow_request("user1").unwrap());
        assert!(limiter.allow_request("user1").unwrap());
        assert!(!limiter.allow_request("user1").unwrap());
        
        // Wait for refill
        sleep(Duration::from_secs(1));
        
        // Should have new tokens
        assert!(limiter.allow_request("user1").unwrap());
    }
    
    #[test]
    fn test_token_bucket_different_keys() {
        let config = RateLimitConfig::per_second(2);
        let mut limiter = TokenBucket::new(config);
        
        // Different keys have independent limits
        assert!(limiter.allow_request("user1").unwrap());
        assert!(limiter.allow_request("user2").unwrap());
        assert!(limiter.allow_request("user1").unwrap());
        assert!(limiter.allow_request("user2").unwrap());
    }
}