//! Companion client for communicating with Companion server

use super::{CompanionAction, CompanionFeedback};
use anyhow::{Context, Result};
use log::{debug, error, info};
use reqwest::Client;
use std::time::Duration;

/// Client for communicating with Companion server
pub struct CompanionClient {
    /// HTTP client
    client: Client,
    /// Base URL for Companion server
    base_url: String,
    /// Whether the client is enabled
    enabled: bool,
}

impl CompanionClient {
    /// Create a new Companion client
    pub fn new(host: &str, port: u16, enabled: bool) -> Self {
        let client = Client::builder()
            .timeout(Duration::from_secs(5))
            .build()
            .unwrap();

        let base_url = format!("http://{}:{}", host, port);

        Self {
            client,
            base_url,
            enabled,
        }
    }

    /// Check if the client is enabled
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    /// Send an action to Companion
    pub async fn send_action(&self, action: CompanionAction) -> Result<()> {
        if !self.enabled {
            debug!("Companion client is disabled, skipping action");
            return Ok(());
        }

        let url = format!("{}/api/action", self.base_url);
        let response = self
            .client
            .post(&url)
            .json(&action)
            .send()
            .await
            .context("Failed to send action to Companion")?;

        if response.status().is_success() {
            info!("Action sent to Companion successfully");
            Ok(())
        } else {
            error!("Failed to send action: {}", response.status());
            Err(anyhow::anyhow!(
                "Companion server returned error: {}",
                response.status()
            ))
        }
    }

    /// Get feedback from Companion
    pub async fn get_feedback(&self) -> Result<CompanionFeedback> {
        if !self.enabled {
            debug!("Companion client is disabled, returning empty feedback");
            return Ok(CompanionFeedback {
                layout: None,
                routes: vec![],
                sources: vec![],
            });
        }

        let url = format!("{}/api/feedback", self.base_url);
        let response = self
            .client
            .get(&url)
            .send()
            .await
            .context("Failed to get feedback from Companion")?;

        if response.status().is_success() {
            let feedback = response
                .json::<CompanionFeedback>()
                .await
                .context("Failed to parse feedback from Companion")?;
            Ok(feedback)
        } else {
            error!("Failed to get feedback: {}", response.status());
            Err(anyhow::anyhow!(
                "Companion server returned error: {}",
                response.status()
            ))
        }
    }

    /// Press a button on the streamdeck
    pub async fn press_button(&self, page: u8, bank: u8) -> Result<()> {
        self.send_action(CompanionAction::PressButton { page, bank })
            .await
    }

    /// Set button text
    #[allow(dead_code)]
    pub async fn set_button_text(&self, page: u8, bank: u8, text: String) -> Result<()> {
        self.send_action(CompanionAction::SetButtonText { page, bank, text })
            .await
    }

    /// Set button color
    #[allow(dead_code)]
    pub async fn set_button_color(&self, page: u8, bank: u8, color: String) -> Result<()> {
        self.send_action(CompanionAction::SetButtonColor { page, bank, color })
            .await
    }

    /// Change layout
    pub async fn set_layout(&self, layout: &str) -> Result<()> {
        self.send_action(CompanionAction::SetLayout {
            layout: layout.to_string(),
        })
        .await
    }

    /// Create a route
    pub async fn route(&self, input: &str, output: &str) -> Result<()> {
        self.send_action(CompanionAction::Route {
            input: input.to_string(),
            output: output.to_string(),
        })
        .await
    }

    /// Remove a route
    pub async fn unroute(&self, output: &str) -> Result<()> {
        self.send_action(CompanionAction::Unroute {
            output: output.to_string(),
        })
        .await
    }

    /// Refresh sources
    #[allow(dead_code)]
    pub async fn refresh_sources(&self) -> Result<()> {
        self.send_action(CompanionAction::RefreshSources).await
    }

    /// Test connection to Companion server
    pub async fn test_connection(&self) -> bool {
        if !self.enabled {
            return false;
        }

        let url = format!("{}/api/feedback", self.base_url);
        match self.client.get(&url).send().await {
            Ok(response) => response.status().is_success(),
            Err(_) => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_client_creation() {
        let client = CompanionClient::new("localhost", 8888, true);
        assert!(client.is_enabled());
        assert_eq!(client.base_url, "http://localhost:8888");
    }

    #[test]
    fn test_client_disabled() {
        let client = CompanionClient::new("localhost", 8888, false);
        assert!(!client.is_enabled());
    }

    #[tokio::test]
    async fn test_disabled_client_actions() {
        let client = CompanionClient::new("localhost", 8888, false);
        // Should not fail even if server is not running
        assert!(client.set_layout("1+7 Layout").await.is_ok());
        assert!(client.press_button(1, 1).await.is_ok());
    }
}
