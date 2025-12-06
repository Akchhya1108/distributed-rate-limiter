// Initialize Chart
const ctx = document.getElementById('requestChart').getContext('2d');

const chart = new Chart(ctx, {
    type: 'line',
    data: {
        labels: [],
        datasets: [
            {
                label: 'Allowed',
                data: [],
                borderColor: 'rgb(74, 222, 128)',
                backgroundColor: 'rgba(74, 222, 128, 0.2)',
                tension: 0.4,
                fill: true,
                pointRadius: 4,
                pointHoverRadius: 6
            },
            {
                label: 'Blocked',
                data: [],
                borderColor: 'rgb(248, 113, 113)',
                backgroundColor: 'rgba(248, 113, 113, 0.2)',
                tension: 0.4,
                fill: true,
                pointRadius: 4,
                pointHoverRadius: 6
            }
        ]
    },
    options: {
        responsive: true,
        maintainAspectRatio: true,
        animation: {
            duration: 500
        },
        plugins: {
            legend: {
                labels: {
                    color: 'rgb(156, 163, 175)',
                    font: {
                        size: 14
                    }
                }
            },
            tooltip: {
                mode: 'index',
                intersect: false
            }
        },
        scales: {
            y: {
                beginAtZero: true,
                ticks: {
                    color: 'rgb(156, 163, 175)',
                    callback: function(value) {
                        return value.toLocaleString();
                    }
                },
                grid: {
                    color: 'rgba(75, 85, 99, 0.3)'
                }
            },
            x: {
                ticks: {
                    color: 'rgb(156, 163, 175)',
                    maxRotation: 45,
                    minRotation: 0
                },
                grid: {
                    color: 'rgba(75, 85, 99, 0.3)'
                }
            }
        }
    }
});

// Update sliders
document.getElementById('max-requests').addEventListener('input', (e) => {
    document.getElementById('max-requests-value').textContent = e.target.value;
});

document.getElementById('window').addEventListener('input', (e) => {
    document.getElementById('window-value').textContent = e.target.value;
});

document.getElementById('test-requests').addEventListener('input', (e) => {
    document.getElementById('test-requests-value').textContent = e.target.value;
});

// Fetch and update metrics
async function fetchMetrics() {
    try {
        const response = await fetch('http://localhost:3001/api/metrics');
        const data = await response.json();
        
        // Update metric cards
        document.getElementById('total-requests').textContent = data.total.toLocaleString();
        document.getElementById('allowed-requests').textContent = data.allowed.toLocaleString();
        document.getElementById('blocked-requests').textContent = data.blocked.toLocaleString();
        document.getElementById('allow-rate').textContent = data.allow_rate.toFixed(1) + '%';
        
        // Update chart
        const now = new Date().toLocaleTimeString();
        
        // Add new data point
        chart.data.labels.push(now);
        chart.data.datasets[0].data.push(data.allowed);
        chart.data.datasets[1].data.push(data.blocked);
        
        // Keep only last 20 data points
        if (chart.data.labels.length > 20) {
            chart.data.labels.shift();
            chart.data.datasets[0].data.shift();
            chart.data.datasets[1].data.shift();
        }
        
        chart.update('none');
        
    } catch (error) {
        console.error('Failed to fetch metrics:', error);
    }
}

// Run test
document.getElementById('run-test').addEventListener('click', async () => {
    const button = document.getElementById('run-test');
    button.disabled = true;
    button.textContent = '⏳ Running test...';
    
    const testData = {
        algorithm: document.getElementById('algorithm').value,
        max_requests: parseInt(document.getElementById('max-requests').value),
        window_seconds: parseInt(document.getElementById('window').value),
        num_requests: parseInt(document.getElementById('test-requests').value)
    };
    
    try {
        const response = await fetch('http://localhost:3001/api/test', {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json'
            },
            body: JSON.stringify(testData)
        });
        
        const results = await response.json();
        
        // Show results
        document.getElementById('test-results').classList.remove('hidden');
        document.getElementById('result-allowed').textContent = results.allowed;
        document.getElementById('result-blocked').textContent = results.blocked;
        document.getElementById('result-duration').textContent = results.duration_ms.toFixed(2) + 'ms';
        document.getElementById('result-throughput').textContent = Math.round(results.requests_per_sec).toLocaleString() + '/s';
        
        // Visualize results
        const viz = document.getElementById('result-visualization');
        viz.innerHTML = '';
        results.results.forEach((allowed, i) => {
            const dot = document.createElement('div');
            dot.className = `w-3 h-3 rounded-full ${allowed ? 'bg-green-400' : 'bg-red-400'} transition-all hover:scale-150`;
            dot.title = `Request ${i + 1}: ${allowed ? 'Allowed' : 'Blocked'}`;
            viz.appendChild(dot);
        });
        
        // Fetch updated metrics immediately after test
        setTimeout(fetchMetrics, 100);
        
    } catch (error) {
        console.error('Test failed:', error);
        alert('Test failed! Make sure the server is running at http://localhost:3001');
    } finally {
        button.disabled = false;
        button.textContent = '🚀 Run Test';
    }
});

// Initial fetch
fetchMetrics();

// Auto-refresh metrics every 2 seconds
setInterval(fetchMetrics, 2000);

console.log('✅ Dashboard loaded!');