# 🚀 Distributed Rate Limiter

<div align="center">

![Rust](https://img.shields.io/badge/Rust-1.75+-orange?logo=rust)
![License](https://img.shields.io/badge/License-MIT-blue)
![Stars](https://img.shields.io/github/stars/Akchhya1108/distributed-rate-limiter?style=social)

**High-performance distributed rate limiter built in Rust**

[Features](#-features) • [Demo](#-demo) • [Quick Start](#-quick-start) • [Benchmarks](#-benchmarks) • [Documentation](#-documentation)

</div>

---

## 🎯 Overview

Production-grade rate limiting infrastructure supporting multiple algorithms, distributed coordination via Redis, real-time monitoring with Prometheus, and an interactive web dashboard. Achieves **140,000+ requests/second** with sub-millisecond latency.

### Key Highlights

- 🔥 **140K+ req/s** throughput (proven via benchmarks)
- ⚡ **<1ms P99 latency** for all algorithms
- 🎨 **Beautiful web dashboard** with real-time visualization
- 🔄 **4 rate limiting algorithms** (Token Bucket, Leaky Bucket, Fixed/Sliding Window)
- 🌐 **Distributed mode** using Redis with atomic Lua scripts
- 📊 **Prometheus metrics** for production monitoring
- 🧪 **Comprehensive testing** with load tests and benchmarks

---

## ✨ Features

### Algorithms

| Algorithm | Throughput | Latency (P99) | Memory | Best For |
|-----------|------------|---------------|---------|----------|
| **Token Bucket** | 140K req/s | <850µs | Low | General APIs, variable traffic |
| **Leaky Bucket** | 136K req/s | <900µs | Low | Streaming, constant output |
| **Fixed Window** | 172K req/s ⚡ | <500µs 🏆 | Very Low | High-traffic APIs |
| **Sliding Window** | 75K req/s | <1.4ms | High | Precision, high-value APIs |

### Infrastructure

- ✅ **Redis Integration** - Distributed coordination with Lua scripts
- ✅ **Circuit Breaker** - Graceful degradation on failures
- ✅ **Prometheus Metrics** - Real-time observability
- ✅ **Multi-tier Limits** - User/IP/endpoint/global support
- ✅ **Web Dashboard** - Interactive testing and visualization

---

## 🎮 Demo

### Web Dashboard

**Live interactive dashboard at http://localhost:3000**

![Dashboard Screenshot](docs/screenshots/dashboard.png)

### Features:
- 📊 Real-time metrics cards
- 🎯 Interactive rate limit simulator
- 📈 Live charts showing request patterns
- 🌈 Visual allowed/blocked request indicators
- 📊 Algorithm comparison page

### Quick Demo:
```bash
# Start the web server
cargo run --bin web-server

# Open browser
http://localhost:3000
```

---

## 🚀 Quick Start

### Prerequisites

- Rust 1.75+ ([Install](https://rustup.rs/))
- Redis (optional, for distributed mode)

### Installation
```bash
# Clone repository
git clone https://github.com/Akchhya1108/distributed-rate-limiter.git
cd distributed-rate-limiter

# Build
cargo build --release

# Run tests
cargo test

# Run benchmarks
cargo bench
```

### Basic Usage
```rust
use distributed_rate_limiter::{RateLimiter, RateLimitConfig};
use distributed_rate_limiter::algorithms::TokenBucket;

// Create rate limiter: 100 requests per second
let config = RateLimitConfig::per_second(100);
let mut limiter = TokenBucket::new(config);

// Check if request is allowed
if limiter.allow_request("user_123").unwrap() {
    // Process request
    println!("✅ Request allowed");
} else {
    // Reject request
    println!("❌ Rate limit exceeded");
}
```

### Web Dashboard
```bash
# Start web server
cargo run --bin web-server

# Open http://localhost:3000 in browser
```

---

## 📊 Benchmarks

### Performance Results

**Environment**: Windows 11, Intel i7, 16GB RAM

| Algorithm | Throughput | P50 Latency | P99 Latency |
|-----------|------------|-------------|-------------|
| Token Bucket | 140,000 req/s | 450µs | 850µs |
| Leaky Bucket | 136,000 req/s | 500µs | 900µs |
| Fixed Window | **172,000 req/s** | 300µs | **500µs** |
| Sliding Window | 75,000 req/s | 800µs | 1.4ms |

### Run Benchmarks
```bash
# Criterion benchmarks
cargo bench

# Load tests
cargo test --release -- --nocapture

# View HTML report
open target/criterion/report/index.html
```

Full benchmark results: [results/BENCHMARKS.md](results/BENCHMARKS.md)

---

## 🏗️ Architecture