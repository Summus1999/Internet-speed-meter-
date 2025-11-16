use anyhow::Result;
use std::time::Instant;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpeedTestResult {
    pub download_speed: f64, // Mbps
    pub upload_speed: f64,   // Mbps
    pub ping: f64,           // ms
    pub test_duration: f64,  // seconds
}

impl SpeedTestResult {
    pub fn print_summary(&self) {
        println!("\n📊 Speed Test Results:");
        println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
        println!("📥 Download Speed: {:.2} Mbps", self.download_speed);
        println!("📤 Upload Speed:   {:.2} Mbps", self.upload_speed);
        println!("🔗 Ping:          {:.2} ms", self.ping);
        println!("⏱️  Duration:      {:.2} s", self.test_duration);
        println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    }
}

pub struct SpeedTester {
    _server_url: String,
}

impl SpeedTester {
    pub fn new(server_url: Option<String>) -> Self {
        let _server_url = server_url.unwrap_or_else(|| {
            "http://speedtest.ftp.otenet.gr/files/test10Mb.db".to_string()
        });

        SpeedTester { _server_url }
    }

    pub async fn run_test(&self) -> Result<SpeedTestResult> {
        println!("🔄 Initializing speed test...");

        // Simulate speed test process
        let start = Instant::now();

        // Measure ping
        let ping = self.measure_ping().await?;
        println!("✓ Ping: {:.2}ms", ping);

        // Measure download speed
        let download_speed = self.measure_download().await?;
        println!("✓ Download: {:.2} Mbps", download_speed);

        // Measure upload speed
        let upload_speed = self.measure_upload().await?;
        println!("✓ Upload: {:.2} Mbps", upload_speed);

        let test_duration = start.elapsed().as_secs_f64();

        Ok(SpeedTestResult {
            download_speed,
            upload_speed,
            ping,
            test_duration,
        })
    }

    async fn measure_ping(&self) -> Result<f64> {
        // Simulate ping measurement (5-50ms)
        let ping = (std::process::id() % 45 + 5) as f64;
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
        Ok(ping)
    }

    async fn measure_download(&self) -> Result<f64> {
        // Simulate download measurement (10-100 Mbps)
        let download = (std::process::id() % 90 + 10) as f64;
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
        Ok(download)
    }

    async fn measure_upload(&self) -> Result<f64> {
        // Simulate upload measurement (5-50 Mbps)
        let upload = (std::process::id() % 45 + 5) as f64;
        tokio::time::sleep(tokio::time::Duration::from_millis(800)).await;
        Ok(upload)
    }
}
