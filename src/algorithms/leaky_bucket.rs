use crate::{RateLimiter, RateLimitConfig, Result};
use std::collections::HashMap;
use std::time::Instant;

#[derive(Debug)]
struct BucketState {
    water_level: f64, // Current "water" in bucket
    last_update: Instant,
}

/// Leaky Bucket algorithm implementation
/// Water (requests) fills a bucket that leaks at a constant rate
/// If bucket overflows, requests are denied
pub struct LeakyBucket {
    config: RateLimitConfig,
    buckets: HashMap<String, BucketState>,
    leak_rate: f64, // water units leaked per second
    max_capacity: f64,
}

impl LeakyBucket {
    pub fn new(config: RateLimitConfig) -> Self {
        let leak_rate = config.max_requests as f64 / config.window.as_secs_f64();
        let max_capacity = config.max_requests as f64;
        
        Self {
            config,
            buckets: HashMap::new(),
            leak_rate,
            max_capacity,
        }
    }
    
    fn update_bucket(&mut self, key: &str) -> &mut BucketState {
        let now = Instant::now();
        
        let bucket = self.buckets.entry(key.to_string()).or_insert(BucketState {
            water_level: 0.0, // Start empty
            last_update: now,
        });
        
        // Calculate how much has leaked since last check
        let elapsed = now.duration_since(bucket.last_update).as_secs_f64();
        let leaked = elapsed * self.leak_rate;
        
        // Reduce water level by leaked amount (but not below 0)
        bucket.water_level = (bucket.water_level - leaked).max(0.0);
        bucket.last_update = now;
        
        bucket
    }
}

impl RateLimiter for LeakyBucket {
    fn allow_request(&mut self, key: &str) -> Result<bool> {
        let max_capacity = self.max_capacity;
        let bucket = self.update_bucket(key);
        
        // Check if adding 1 unit of water would overflow
        if bucket.water_level + 1.0 <= max_capacity {
            bucket.water_level += 1.0;
            Ok(true)
        } else {
            Ok(false) // Bucket would overflow
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
    use std::time::Duration;
    
    #[test]
    fn test_leaky_bucket_allows_requests() {
        let config = RateLimitConfig::per_second(3);
        let mut limiter = LeakyBucket::new(config);
        
        // Bucket starts empty, can accept 3 requests
        assert!(limiter.allow_request("user1").unwrap(), "Request 1 should be allowed");
        assert!(limiter.allow_request("user1").unwrap(), "Request 2 should be allowed");
        assert!(limiter.allow_request("user1").unwrap(), "Request 3 should be allowed");
        
        // 4th request overflows the bucket
        assert!(!limiter.allow_request("user1").unwrap(), "4th request should be denied");
    }
    
    #[test]
    fn test_leaky_bucket_leaks() {
        let config = RateLimitConfig::per_second(2);
        let mut limiter = LeakyBucket::new(config);
        
        // Fill bucket to capacity
        assert!(limiter.allow_request("user1").unwrap());
        assert!(limiter.allow_request("user1").unwrap());
        
        // Bucket is full now
        assert!(!limiter.allow_request("user1").unwrap());
        
        // Wait for bucket to leak (1.1 seconds = ~2.2 units leaked)
        sleep(Duration::from_millis(1100));
        
        // Bucket should be empty now, can accept requests again
        assert!(limiter.allow_request("user1").unwrap());
        assert!(limiter.allow_request("user1").unwrap());
    }
}