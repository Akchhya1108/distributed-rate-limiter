use crate::{RateLimiter, RateLimitConfig, Result};
use std::collections::{HashMap, VecDeque};
use std::time::{Duration, Instant};

#[derive(Debug)]
struct RequestLog {
    timestamps: VecDeque<Instant>,
}

/// Sliding Window algorithm implementation
/// Tracks individual request timestamps in a sliding window
/// Most accurate but uses more memory
pub struct SlidingWindow {
    config: RateLimitConfig,
    logs: HashMap<String, RequestLog>,
}

impl SlidingWindow {
    pub fn new(config: RateLimitConfig) -> Self {
        Self {
            config,
            logs: HashMap::new(),
        }
    }
    
    fn clean_old_requests(&mut self, key: &str) {
        let now = Instant::now();
        
        if let Some(log) = self.logs.get_mut(key) {
            // Remove requests outside the sliding window
            while let Some(&timestamp) = log.timestamps.front() {
                if now.duration_since(timestamp) > self.config.window {
                    log.timestamps.pop_front();
                } else {
                    break;
                }
            }
        }
    }
}

impl RateLimiter for SlidingWindow {
    fn allow_request(&mut self, key: &str) -> Result<bool> {
        self.clean_old_requests(key);
        
        let log = self.logs.entry(key.to_string()).or_insert(RequestLog {
            timestamps: VecDeque::new(),
        });
        
        if log.timestamps.len() < self.config.max_requests as usize {
            log.timestamps.push_back(Instant::now());
            Ok(true)
        } else {
            Ok(false)
        }
    }
    
    fn reset(&mut self, key: &str) {
        self.logs.remove(key);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread::sleep;
    
    #[test]
    fn test_sliding_window_allows_requests() {
        let config = RateLimitConfig::per_second(5);
        let mut limiter = SlidingWindow::new(config);
        
        // Should allow 5 requests
        for _ in 0..5 {
            assert!(limiter.allow_request("user1").unwrap());
        }
        
        // 6th should be denied
        assert!(!limiter.allow_request("user1").unwrap());
    }
    
    #[test]
    fn test_sliding_window_slides() {
        let config = RateLimitConfig::new(3, Duration::from_millis(500));
        let mut limiter = SlidingWindow::new(config);
        
        // Use up limit
        for _ in 0..3 {
            assert!(limiter.allow_request("user1").unwrap());
        }
        assert!(!limiter.allow_request("user1").unwrap());
        
        // Wait for oldest request to slide out
        sleep(Duration::from_millis(600));
        
        // Should allow new requests as old ones slide out
        assert!(limiter.allow_request("user1").unwrap());
    }
}