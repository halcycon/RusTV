use super::ptz::{PtzCommand, PtzPosition};
use anyhow::{Context, Result};
use log::{debug, info};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::time::Duration;

/// BirdDog camera API client
pub struct BirdDogClient {
    base_url: String,
    client: Client,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CameraInfo {
    pub model: String,
    pub firmware_version: String,
    pub serial_number: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CameraStatus {
    pub online: bool,
    pub recording: bool,
    pub streaming: bool,
    pub temperature: f64,
}

impl BirdDogClient {
    /// Create a new BirdDog API client
    pub fn new(camera_ip: &str) -> Self {
        let base_url = format!("http://{}", camera_ip);
        let client = Client::builder()
            .timeout(Duration::from_secs(10))
            .build()
            .expect("Failed to create HTTP client");

        Self { base_url, client }
    }

    /// Get camera information
    pub async fn get_info(&self) -> Result<CameraInfo> {
        info!("Fetching camera info from {}", self.base_url);
        
        // BirdDog API endpoint for camera info
        let url = format!("{}/api/camera/info", self.base_url);
        
        let response = self.client
            .get(&url)
            .send()
            .await
            .context("Failed to send request")?;

        let info: CameraInfo = response
            .json()
            .await
            .context("Failed to parse camera info")?;

        debug!("Camera info: {:?}", info);
        Ok(info)
    }

    /// Get camera status
    pub async fn get_status(&self) -> Result<CameraStatus> {
        debug!("Fetching camera status from {}", self.base_url);
        
        let url = format!("{}/api/camera/status", self.base_url);
        
        let response = self.client
            .get(&url)
            .send()
            .await
            .context("Failed to send request")?;

        let status: CameraStatus = response
            .json()
            .await
            .context("Failed to parse camera status")?;

        Ok(status)
    }

    /// Send PTZ command to camera
    pub async fn send_ptz_command(&self, command: &PtzCommand) -> Result<()> {
        info!("Sending PTZ command: {:?}", command);
        
        let url = format!("{}/api/ptz/control", self.base_url);
        let params = command.to_birddog_api_params();
        
        let response = self.client
            .post(&url)
            .form(&params)
            .send()
            .await
            .context("Failed to send PTZ command")?;

        if !response.status().is_success() {
            anyhow::bail!("PTZ command failed with status: {}", response.status());
        }

        info!("PTZ command executed successfully");
        Ok(())
    }

    /// Get current PTZ position
    pub async fn get_ptz_position(&self) -> Result<PtzPosition> {
        debug!("Fetching PTZ position from {}", self.base_url);
        
        let url = format!("{}/api/ptz/position", self.base_url);
        
        let response = self.client
            .get(&url)
            .send()
            .await
            .context("Failed to send request")?;

        let position: PtzPosition = response
            .json()
            .await
            .context("Failed to parse PTZ position")?;

        Ok(position)
    }

    /// Move camera to absolute position
    pub async fn move_absolute(&self, position: PtzPosition) -> Result<()> {
        self.send_ptz_command(&PtzCommand::MoveAbsolute(position)).await
    }

    /// Move camera relative to current position
    pub async fn move_relative(&self, pan: f64, tilt: f64, zoom: f64) -> Result<()> {
        self.send_ptz_command(&PtzCommand::MoveRelative { pan, tilt, zoom }).await
    }

    /// Stop camera movement
    pub async fn stop(&self) -> Result<()> {
        self.send_ptz_command(&PtzCommand::Stop).await
    }

    /// Move camera to home position
    pub async fn home(&self) -> Result<()> {
        self.send_ptz_command(&PtzCommand::Home).await
    }

    /// Save current position as preset
    pub async fn save_preset(&self, preset_id: u8) -> Result<()> {
        self.send_ptz_command(&PtzCommand::SavePreset(preset_id)).await
    }

    /// Recall preset position
    pub async fn recall_preset(&self, preset_id: u8) -> Result<()> {
        self.send_ptz_command(&PtzCommand::RecallPreset(preset_id)).await
    }

    /// Set focus value
    pub async fn set_focus(&self, focus: f64) -> Result<()> {
        self.send_ptz_command(&PtzCommand::SetFocus(focus)).await
    }

    /// Enable auto focus
    pub async fn auto_focus(&self) -> Result<()> {
        self.send_ptz_command(&PtzCommand::AutoFocus).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_client_creation() {
        let client = BirdDogClient::new("192.168.1.100");
        assert!(client.base_url.contains("192.168.1.100"));
    }
}
