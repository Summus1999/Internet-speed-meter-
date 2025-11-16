use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiagnosticResult {
    pub target_host: String,
    pub connectivity: bool,
    pub dns_resolution: bool,
    pub dns_servers: Vec<String>,
    pub average_latency: f64,
    pub packet_loss: f64,
    pub issues_detected: Vec<String>,
    pub recommendations: Vec<String>,
}

impl DiagnosticResult {
    pub fn print_report(&self) {
        println!("\n📋 Network Diagnostic Report");
        println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
        
        println!("\n🎯 Target: {}", self.target_host);
        
        println!("\n✅ Status:");
        println!("   • Connectivity: {}", if self.connectivity { "✓ OK" } else { "✗ Failed" });
        println!("   • DNS Resolution: {}", if self.dns_resolution { "✓ OK" } else { "✗ Failed" });
        
        if !self.dns_servers.is_empty() {
            println!("\n🔍 DNS Servers:");
            for (i, server) in self.dns_servers.iter().enumerate() {
                println!("   {}. {}", i + 1, server);
            }
        }
        
        println!("\n📊 Network Metrics:");
        println!("   • Average Latency: {:.2} ms", self.average_latency);
        println!("   • Packet Loss: {:.2}%", self.packet_loss);
        
        if !self.issues_detected.is_empty() {
            println!("\n⚠️  Issues Detected:");
            for issue in &self.issues_detected {
                println!("   • {}", issue);
            }
        } else {
            println!("\n✅ No issues detected");
        }
        
        if !self.recommendations.is_empty() {
            println!("\n💡 Recommendations:");
            for (i, rec) in self.recommendations.iter().enumerate() {
                println!("   {}. {}", i + 1, rec);
            }
        }
        
        println!("\n━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    }
}

pub struct NetworkDiagnostics;

impl NetworkDiagnostics {
    pub fn new() -> Self {
        NetworkDiagnostics
    }

    pub async fn run_full_diagnostic(&self, target_host: &str) -> Result<DiagnosticResult> {
        println!("🔍 Analyzing network configuration...");
        
        // Check connectivity
        let connectivity = self.check_connectivity(target_host).await?;
        println!("✓ Connectivity check: {}", if connectivity { "OK" } else { "Failed" });

        // Check DNS resolution
        let dns_resolution = self.check_dns_resolution(target_host).await?;
        println!("✓ DNS resolution check: {}", if dns_resolution { "OK" } else { "Failed" });

        // Get DNS servers
        let dns_servers = self.get_dns_servers().await?;
        println!("✓ Retrieved {} DNS server(s)", dns_servers.len());

        // Measure latency
        let average_latency = self.measure_latency(target_host).await?;
        println!("✓ Average latency: {:.2}ms", average_latency);

        // Check packet loss
        let packet_loss = self.check_packet_loss(target_host).await?;
        println!("✓ Packet loss: {:.2}%", packet_loss);

        // Detect issues
        let issues_detected = self.detect_issues(
            connectivity,
            dns_resolution,
            average_latency,
            packet_loss,
        );

        // Generate recommendations
        let recommendations = self.generate_recommendations(&issues_detected);

        Ok(DiagnosticResult {
            target_host: target_host.to_string(),
            connectivity,
            dns_resolution,
            dns_servers,
            average_latency,
            packet_loss,
            issues_detected,
            recommendations,
        })
    }

    async fn check_connectivity(&self, _target_host: &str) -> Result<bool> {
        // Simulate connectivity check
        tokio::time::sleep(tokio::time::Duration::from_millis(200)).await;
        Ok(true)
    }

    async fn check_dns_resolution(&self, _target_host: &str) -> Result<bool> {
        // Simulate DNS resolution check
        tokio::time::sleep(tokio::time::Duration::from_millis(150)).await;
        Ok(true)
    }

    async fn get_dns_servers(&self) -> Result<Vec<String>> {
        // Simulate getting DNS servers
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
        Ok(vec![
            "8.8.8.8".to_string(),
            "8.8.4.4".to_string(),
            "1.1.1.1".to_string(),
        ])
    }

    async fn measure_latency(&self, _target_host: &str) -> Result<f64> {
        // Simulate latency measurement
        tokio::time::sleep(tokio::time::Duration::from_millis(300)).await;
        Ok(25.5)
    }

    async fn check_packet_loss(&self, _target_host: &str) -> Result<f64> {
        // Simulate packet loss check
        tokio::time::sleep(tokio::time::Duration::from_millis(200)).await;
        Ok(0.0)
    }

    fn detect_issues(&self, connectivity: bool, dns_resolution: bool, latency: f64, packet_loss: f64) -> Vec<String> {
        let mut issues = Vec::new();

        if !connectivity {
            issues.push("No network connectivity detected".to_string());
        }

        if !dns_resolution {
            issues.push("DNS resolution is failing".to_string());
        }

        if latency > 100.0 {
            issues.push(format!("High latency detected: {:.2}ms", latency));
        }

        if packet_loss > 0.5 {
            issues.push(format!("Packet loss detected: {:.2}%", packet_loss));
        }

        issues
    }

    fn generate_recommendations(&self, issues: &[String]) -> Vec<String> {
        let mut recommendations = Vec::new();

        if issues.is_empty() {
            recommendations.push("Your network appears to be functioning normally".to_string());
        } else {
            for issue in issues {
                if issue.contains("connectivity") {
                    recommendations.push("Check your network cable or WiFi connection".to_string());
                } else if issue.contains("DNS") {
                    recommendations.push("Try changing your DNS servers to 8.8.8.8 or 1.1.1.1".to_string());
                } else if issue.contains("latency") {
                    recommendations.push("Try restarting your router or contacting your ISP".to_string());
                } else if issue.contains("Packet loss") {
                    recommendations.push("Check for WiFi interference or try using a wired connection".to_string());
                }
            }
        }

        recommendations
    }
}

impl Default for NetworkDiagnostics {
    fn default() -> Self {
        Self::new()
    }
}
