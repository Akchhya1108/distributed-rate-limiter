use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use distributed_rate_limiter::{RateLimiter, RateLimitConfig};
use distributed_rate_limiter::algorithms::*;
use std::time::Duration;

fn benchmark_token_bucket(c: &mut Criterion) {
    let mut group = c.benchmark_group("token_bucket");
    
    for rate in [100, 1000, 10000, 100000].iter() {
        let config = RateLimitConfig::per_second(*rate);
        let mut limiter = TokenBucket::new(config);
        
        group.bench_with_input(BenchmarkId::from_parameter(rate), rate, |b, _| {
            b.iter(|| {
                let _ = limiter.allow_request(black_box("user1"));
            });
        });
    }
    group.finish();
}

fn benchmark_leaky_bucket(c: &mut Criterion) {
    let mut group = c.benchmark_group("leaky_bucket");
    
    for rate in [100, 1000, 10000, 100000].iter() {
        let config = RateLimitConfig::per_second(*rate);
        let mut limiter = LeakyBucket::new(config);
        
        group.bench_with_input(BenchmarkId::from_parameter(rate), rate, |b, _| {
            b.iter(|| {
                let _ = limiter.allow_request(black_box("user1"));
            });
        });
    }
    group.finish();
}

fn benchmark_fixed_window(c: &mut Criterion) {
    let mut group = c.benchmark_group("fixed_window");
    
    for rate in [100, 1000, 10000, 100000].iter() {
        let config = RateLimitConfig::per_second(*rate);
        let mut limiter = FixedWindow::new(config);
        
        group.bench_with_input(BenchmarkId::from_parameter(rate), rate, |b, _| {
            b.iter(|| {
                let _ = limiter.allow_request(black_box("user1"));
            });
        });
    }
    group.finish();
}

fn benchmark_sliding_window(c: &mut Criterion) {
    let mut group = c.benchmark_group("sliding_window");
    
    for rate in [100, 1000, 10000].iter() {
        let config = RateLimitConfig::per_second(*rate);
        let mut limiter = SlidingWindow::new(config);
        
        group.bench_with_input(BenchmarkId::from_parameter(rate), rate, |b, _| {
            b.iter(|| {
                let _ = limiter.allow_request(black_box("user1"));
            });
        });
    }
    group.finish();
}

fn benchmark_comparison(c: &mut Criterion) {
    let mut group = c.benchmark_group("algorithm_comparison");
    let config = RateLimitConfig::per_second(1000);
    
    group.bench_function("token_bucket", |b| {
        let mut limiter = TokenBucket::new(config.clone());
        b.iter(|| {
            let _ = limiter.allow_request(black_box("user1"));
        });
    });
    
    group.bench_function("leaky_bucket", |b| {
        let mut limiter = LeakyBucket::new(config.clone());
        b.iter(|| {
            let _ = limiter.allow_request(black_box("user1"));
        });
    });
    
    group.bench_function("fixed_window", |b| {
        let mut limiter = FixedWindow::new(config.clone());
        b.iter(|| {
            let _ = limiter.allow_request(black_box("user1"));
        });
    });
    
    group.bench_function("sliding_window", |b| {
        let mut limiter = SlidingWindow::new(config.clone());
        b.iter(|| {
            let _ = limiter.allow_request(black_box("user1"));
        });
    });
    
    group.finish();
}

criterion_group!(
    benches,
    benchmark_token_bucket,
    benchmark_leaky_bucket,
    benchmark_fixed_window,
    benchmark_sliding_window,
    benchmark_comparison
);
criterion_main!(benches);