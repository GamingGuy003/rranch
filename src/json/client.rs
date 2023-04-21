use std::fmt::Display;

use serde_derive::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Client {
    #[serde(rename = "Connection timestamp")]
    pub connection_timestamp: f64,
    #[serde(rename = "Architecture")]
    pub architecture: Option<String>,
    #[serde(rename = "CPU count")]
    pub cpu_count: Option<u32>,
    #[serde(rename = "CPU name")]
    pub cpu_name: Option<String>,
    #[serde(rename = "Host Distribution")]
    pub host_distribution: Option<String>,
    #[serde(rename = "Host Kernel")]
    pub host_kernel: Option<String>,
    #[serde(rename = "Host Python Version")]
    pub host_python: Option<String>,
    #[serde(rename = "Host libc")]
    pub host_libc: Option<String>,
    #[serde(rename = "Hostname")]
    pub hostname: Option<String>,
    #[serde(rename = "Memory available")]
    pub memory: Option<String>,
    #[serde(rename = "Performance Rating")]
    pub performance_rating: Option<f32>,
    #[serde(rename = "Timed out commands [recovered]")]
    pub timeout_recovery_count: Option<u32>,
}

impl Display for Client {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let not_set = String::from("Not set");
        if self.hostname.is_none() {
            write!(
                f,
                "{:<30}{}",
                "Connection timestamp:", self.connection_timestamp
            )
        } else {
            write!(
                f,
                "{:<30}{}\n{:<30}{}\n{:<30}{}x{} {}\n{:<30}{}\n{:<30}{}\n{:<30}{}\n{:<30}{}\n{:<30}{}\n{:<30}{}\n{:<30}{}",
                "Hostname:",
                self.hostname.clone().unwrap_or(not_set.clone()),
                "Connection timestamp:",
                self.connection_timestamp,
                "CPU:",
                self.cpu_count.unwrap_or(0),
                self.cpu_name.clone().unwrap_or(not_set.clone()),
                self.architecture.clone().unwrap_or(not_set.clone()),
                "Memory:",
                self.memory.clone().unwrap_or(not_set.clone()),
                "Host OS:",
                self.host_distribution.clone().unwrap_or(not_set.clone()),
                "Host Kernel:",
                self.host_kernel.clone().unwrap_or(not_set.clone()),
                "Host Python Version:",
                self.host_python.clone().unwrap_or(not_set.clone()),
                "Host libc:",
                self.host_libc.clone().unwrap_or(not_set),
                "Performance Rating:",
                self.performance_rating.unwrap_or(0.0),
                "Timeout Recovery Count:",
                self.timeout_recovery_count.unwrap_or(0)
            )
        }
    }
}
