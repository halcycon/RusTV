//! Companion integration module for streamdeck control
//!
//! This module provides integration with Companion software (https://bitfocus.io/companion)
//! which enhances the usability of streamdecks and other control surfaces.

mod client;

pub use client::CompanionClient;

use serde::{Deserialize, Serialize};

/// Action types that can be sent to Companion
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum CompanionAction {
    /// Change layout
    SetLayout { layout: String },
    /// Route input to output
    Route { input: String, output: String },
    /// Remove route
    Unroute { output: String },
    /// Refresh sources
    RefreshSources,
    /// Press button
    PressButton { page: u8, bank: u8 },
    /// Set button text
    SetButtonText { page: u8, bank: u8, text: String },
    /// Set button color
    SetButtonColor { page: u8, bank: u8, color: String },
}

/// Feedback from Companion
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompanionFeedback {
    /// Current layout
    pub layout: Option<String>,
    /// Active routes
    pub routes: Vec<CompanionRoute>,
    /// Available sources
    pub sources: Vec<String>,
}

/// Route information for Companion
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompanionRoute {
    pub input: String,
    pub output: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_companion_action_serialization() {
        let action = CompanionAction::SetLayout {
            layout: "1+7 Layout".to_string(),
        };
        let json = serde_json::to_string(&action).unwrap();
        assert!(json.contains("SetLayout"));
    }

    #[test]
    fn test_companion_route() {
        let route = CompanionRoute {
            input: "Camera 1".to_string(),
            output: "Monitor 1".to_string(),
        };
        assert_eq!(route.input, "Camera 1");
        assert_eq!(route.output, "Monitor 1");
    }
}
