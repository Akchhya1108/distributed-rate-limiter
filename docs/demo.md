# 🎮 Demo Guide

## Quick Start

### 1. Start the Server
```bash
cargo run --bin web-server
```

### 2. Open Dashboard

Navigate to: http://localhost:3000

---

## 🎯 Demo Scenarios

### Scenario 1: Basic Rate Limiting

**Goal**: Show how rate limiting blocks excessive requests

1. Set **Max Requests: 10**
2. Set **Test Requests: 20**
3. Click **Run Test**
4. **Result**: First 10 allowed ✅, next 10 blocked ❌

### Scenario 2: Algorithm Comparison

**Goal**: Compare Fixed Window (fastest) vs Sliding Window (most accurate)

1. Test with **Fixed Window**: Note the throughput
2. Test with **Sliding Window**: Note the throughput
3. **Result**: Fixed Window ~2x faster

### Scenario 3: Burst Handling

**Goal**: Show Token Bucket allows bursts

1. Algorithm: **Token Bucket**
2. Max Requests: **50**, Window: **1 second**
3. Test Requests: **100**
4. **Result**: ~50 allowed, showing burst capability

### Scenario 4: High Load

**Goal**: Demonstrate performance under load

1. Max Requests: **100**
2. Test Requests: **200**
3. Run multiple times
4. **Result**: Consistent sub-millisecond latency

---

## 📊 Expected Results

| Algorithm | Throughput | Best Use Case |
|-----------|------------|---------------|
| Token Bucket | ~140K/s | General APIs |
| Leaky Bucket | ~136K/s | Streaming |
| Fixed Window | ~172K/s | High traffic |
| Sliding Window | ~75K/s | Precision |

---

## 🎥 Recording a Demo

### 30-Second Demo Script:

1. **0:00-0:05**: Show dashboard, explain what it does
2. **0:05-0:10**: Adjust sliders, show configurability
3. **0:10-0:15**: Click "Run Test", watch results appear
4. **0:15-0:20**: Show green/red dot visualization
5. **0:20-0:25**: Switch algorithm, run again
6. **0:25-0:30**: Show comparison page

### Recording Tips:

- Use OBS Studio or Windows Game Bar
- 1920x1080 resolution
- Speak clearly and enthusiastically
- Keep it under 1 minute
- Upload to YouTube, add to README