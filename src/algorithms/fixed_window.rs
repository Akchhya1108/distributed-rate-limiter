use crate::{RateLimiter, RateLimitConfig, Result};
use std::collections::HashMap;
use std::time::{Duration, Instant};

#[derive(Debug)]
struct WindowState {
    count: u64,
    window_start: Instant,
}

/// Fixed Window algorithm implementation
/// Counts requests in fixed time windows
/// Simple but can have burst issues at window boundaries
pub struct FixedWindow {
    config: RateLimitConfig,
    windows: HashMap<String, WindowState>,
}

impl FixedWindow {
    pub fn new(config: RateLimitConfig) -> Self {
        Self {
            config,
            windows: HashMap::new(),
        }
    }
    
    fn check_window(&mut self, key: &str) -> &mut WindowState {
        let now = Instant::now();
        
        let window = self.windows.entry(key.to_string()).or_insert(WindowState {
            count: 0,
            window_start: now,
        });
        
        // Check if window has expired
        let elapsed = now.duration_since(window.window_start);
        if elapsed >= self.config.window {
            // Reset window
            window.count = 0;
            window.window_start = now;
        }
        
        window
    }
}

impl RateLimiter for FixedWindow {
    fn allow_request(&mut self, key: &str) -> Result<bool> {
        let max_requests = self.config.max_requests;
        let window = self.check_window(key);
        
        if window.count < max_requests {
            window.count += 1;
            Ok(true)
        } else {
            Ok(false)
        }
    }
    
    fn reset(&mut self, key: &str) {
        self.windows.remove(key);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread::sleep;
    
    #[test]
    fn test_fixed_window_allows_requests() {
        let config = RateLimitConfig::per_second(5);
        let mut limiter = FixedWindow::new(config);
        
        // Should allow 5 requests
        for _ in 0..5 {
            assert!(limiter.allow_request("user1").unwrap());
        }
        
        // 6th should be denied
        assert!(!limiter.allow_request("user1").unwrap());
    }
    
    #[test]
    fn test_fixed_window_resets() {
        let config = RateLimitConfig::new(3, Duration::from_millis(500));
        let mut limiter = FixedWindow::new(config);
        
        // Use up window
        for _ in 0..3 {
            assert!(limiter.allow_request("user1").unwrap());
        }
        assert!(!limiter.allow_request("user1").unwrap());
        
        // Wait for window to expire
        sleep(Duration::from_millis(600));
        
        // Should have new window
        assert!(limiter.allow_request("user1").unwrap());
    }
}