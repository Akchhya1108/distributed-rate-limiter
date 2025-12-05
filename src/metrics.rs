use prometheus::{Counter, Histogram, IntCounter, Registry, Encoder, TextEncoder};
use lazy_static::lazy_static;
use std::time::Instant;

lazy_static! {
    /// Global metrics registry
    pub static ref REGISTRY: Registry = Registry::new();
    
    /// Total requests processed
    pub static ref REQUESTS_TOTAL: IntCounter = IntCounter::new(
        "rate_limiter_requests_total",
        "Total number of requests processed"
    ).expect("metric can be created");
    
    /// Requests allowed
    pub static ref REQUESTS_ALLOWED: IntCounter = IntCounter::new(
        "rate_limiter_requests_allowed",
        "Number of requests allowed"
    ).expect("metric can be created");
    
    /// Requests blocked
    pub static ref REQUESTS_BLOCKED: IntCounter = IntCounter::new(
        "rate_limiter_requests_blocked",
        "Number of requests blocked"
    ).expect("metric can be created");
    
    /// Request processing latency
    pub static ref REQUEST_LATENCY: Histogram = Histogram::with_opts(
        prometheus::HistogramOpts::new(
            "rate_limiter_request_duration_seconds",
            "Request processing latency in seconds"
        ).buckets(vec![0.0001, 0.0005, 0.001, 0.005, 0.01, 0.05, 0.1])
    ).expect("metric can be created");
}

/// Initialize metrics registry
pub fn init_metrics() {
    REGISTRY.register(Box::new(REQUESTS_TOTAL.clone()))
        .expect("collector can be registered");
    REGISTRY.register(Box::new(REQUESTS_ALLOWED.clone()))
        .expect("collector can be registered");
    REGISTRY.register(Box::new(REQUESTS_BLOCKED.clone()))
        .expect("collector can be registered");
    REGISTRY.register(Box::new(REQUEST_LATENCY.clone()))
        .expect("collector can be registered");
}

/// Record a rate limit check
pub fn record_request(allowed: bool, start: Instant) {
    let duration = start.elapsed().as_secs_f64();
    
    REQUESTS_TOTAL.inc();
    if allowed {
        REQUESTS_ALLOWED.inc();
    } else {
        REQUESTS_BLOCKED.inc();
    }
    REQUEST_LATENCY.observe(duration);
}

/// Get metrics in Prometheus format
pub fn get_metrics() -> String {
    let encoder = TextEncoder::new();
    let metric_families = REGISTRY.gather();
    
    let mut buffer = Vec::new();
    encoder.encode(&metric_families, &mut buffer).unwrap();
    String::from_utf8(buffer).unwrap()
}

/// Print metrics summary to console
pub fn print_metrics_summary() {
    println!("\n📊 METRICS SUMMARY");
    println!("==================");
    println!("Total Requests:   {}", REQUESTS_TOTAL.get());
    println!("Allowed:          {} ✅", REQUESTS_ALLOWED.get());
    println!("Blocked:          {} ❌", REQUESTS_BLOCKED.get());
    
    let allow_rate = if REQUESTS_TOTAL.get() > 0 {
        (REQUESTS_ALLOWED.get() as f64 / REQUESTS_TOTAL.get() as f64) * 100.0
    } else {
        0.0
    };
    println!("Allow Rate:       {:.1}%", allow_rate);
}