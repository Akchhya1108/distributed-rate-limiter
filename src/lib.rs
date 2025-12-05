pub mod algorithms;
pub mod redis_limiter;
pub mod metrics; 

use std::time::Duration;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum RateLimitError {
    #[error("Rate limit exceeded")]
    LimitExceeded,
    
    #[error("Configuration error: {0}")]
    ConfigError(String),
}

pub type Result<T> = std::result::Result<T, RateLimitError>;

/// Configuration for rate limiter
#[derive(Debug, Clone)]
pub struct RateLimitConfig {
    pub max_requests: u64,
    pub window: Duration,
}

impl RateLimitConfig {
    pub fn new(max_requests: u64, window: Duration) -> Self {
        Self {
            max_requests,
            window,
        }
    }
    
    pub fn per_second(max_requests: u64) -> Self {
        Self::new(max_requests, Duration::from_secs(1))
    }
    
    pub fn per_minute(max_requests: u64) -> Self {
        Self::new(max_requests, Duration::from_secs(60))
    }
}

/// Trait that all rate limiting algorithms must implement
pub trait RateLimiter: Send + Sync {
    /// Check if a request is allowed
    fn allow_request(&mut self, key: &str) -> Result<bool>;
    
    /// Reset the rate limiter for a specific key
    fn reset(&mut self, key: &str);
}

/// Enum for selecting rate limiting algorithm
#[derive(Debug, Clone, Copy)]
pub enum AlgorithmType {
    TokenBucket,
    LeakyBucket,
    FixedWindow,
    SlidingWindow,
}