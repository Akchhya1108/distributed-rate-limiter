use crate::{RateLimiter, RateLimitConfig, Result};
use redis::{Client, Commands, Script};
use std::time::SystemTime;

/// Redis-backed distributed rate limiter using Lua scripts for atomicity
pub struct RedisRateLimiter {
    client: Client,
    config: RateLimitConfig,
    lua_script: Script,
}

impl RedisRateLimiter {
    /// Create a new Redis rate limiter
    pub fn new(redis_url: &str, config: RateLimitConfig) -> anyhow::Result<Self> {
        let client = Client::open(redis_url)?;
        
        // Lua script for atomic token bucket check
        // This ensures race conditions don't occur in distributed systems
        let lua_script = Script::new(
            r#"
            local key = KEYS[1]
            local max_tokens = tonumber(ARGV[1])
            local refill_rate = tonumber(ARGV[2])
            local now = tonumber(ARGV[3])
            
            -- Get current state
            local state = redis.call('HMGET', key, 'tokens', 'last_refill')
            local tokens = tonumber(state[1]) or max_tokens
            local last_refill = tonumber(state[2]) or now
            
            -- Calculate refill
            local elapsed = now - last_refill
            local tokens_to_add = elapsed * refill_rate
            tokens = math.min(tokens + tokens_to_add, max_tokens)
            
            -- Check if request allowed
            local allowed = 0
            if tokens >= 1.0 then
                tokens = tokens - 1.0
                allowed = 1
            end
            
            -- Save state with expiration (2x window duration)
            redis.call('HMSET', key, 'tokens', tokens, 'last_refill', now)
            redis.call('EXPIRE', key, ARGV[4])
            
            return allowed
            "#,
        );
        
        Ok(Self {
            client,
            config,
            lua_script,
        })
    }
    
    /// Check with circuit breaker pattern
    pub fn check_with_fallback(&mut self, key: &str) -> Result<bool> {
        match self.allow_request(key) {
            Ok(result) => Ok(result),
            Err(_) => {
                // Fallback: allow request but log error
                eprintln!("⚠️  Redis connection failed, allowing request (circuit breaker open)");
                Ok(true)
            }
        }
    }
}

impl RateLimiter for RedisRateLimiter {
    fn allow_request(&mut self, key: &str) -> Result<bool> {
        let mut conn = self.client.get_connection()
            .map_err(|e| crate::RateLimitError::ConfigError(format!("Redis connection failed: {}", e)))?;
        
        let redis_key = format!("rate_limit:{}", key);
        let max_tokens = self.config.max_requests as f64;
        let refill_rate = max_tokens / self.config.window.as_secs_f64();
        let now = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs_f64();
        let ttl = (self.config.window.as_secs() * 2) as usize;
        
        let allowed: i32 = self.lua_script
            .key(&redis_key)
            .arg(max_tokens)
            .arg(refill_rate)
            .arg(now)
            .arg(ttl)
            .invoke(&mut conn)
            .map_err(|e| crate::RateLimitError::ConfigError(format!("Lua script failed: {}", e)))?;
        
        Ok(allowed == 1)
    }
    
    fn reset(&mut self, key: &str) {
        if let Ok(mut conn) = self.client.get_connection() {
            let redis_key = format!("rate_limit:{}", key);
            let res: redis::RedisResult<i32> = conn.del(&redis_key);
            let _ = res;



        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;
    
    #[test]
    fn test_redis_rate_limiter() {
        // Skip if Redis not available
        let config = RateLimitConfig::per_second(5);
        if let Ok(mut limiter) = RedisRateLimiter::new("redis://127.0.0.1/", config) {
            // Clean slate
            limiter.reset("test_user");
            
            // Should allow 5 requests
            for _ in 0..5 {
                assert!(limiter.allow_request("test_user").unwrap());
            }
            
            // 6th should be denied
            assert!(!limiter.allow_request("test_user").unwrap());
            
            // Cleanup
            limiter.reset("test_user");
        }
    }
}