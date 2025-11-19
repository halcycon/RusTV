use serde::{Deserialize, Serialize};

/// PTZ (Pan-Tilt-Zoom) position
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PtzPosition {
    pub pan: f64,  // -1.0 to 1.0
    pub tilt: f64, // -1.0 to 1.0
    pub zoom: f64, // 0.0 to 1.0
}

impl PtzPosition {
    pub fn new(pan: f64, tilt: f64, zoom: f64) -> Self {
        Self {
            pan: pan.clamp(-1.0, 1.0),
            tilt: tilt.clamp(-1.0, 1.0),
            zoom: zoom.clamp(0.0, 1.0),
        }
    }

    #[allow(dead_code)]
    pub fn home() -> Self {
        Self {
            pan: 0.0,
            tilt: 0.0,
            zoom: 0.0,
        }
    }
}

/// PTZ commands
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum PtzCommand {
    /// Move to absolute position
    MoveAbsolute(PtzPosition),
    /// Move relative to current position
    MoveRelative { pan: f64, tilt: f64, zoom: f64 },
    /// Stop all movement
    Stop,
    /// Go to home position
    Home,
    /// Save current position to preset
    SavePreset(u8),
    /// Recall a preset position
    RecallPreset(u8),
    /// Set focus (0.0 to 1.0)
    SetFocus(f64),
    /// Auto focus
    AutoFocus,
}

impl PtzCommand {
    pub fn to_birddog_api_params(&self) -> Vec<(String, String)> {
        match self {
            PtzCommand::MoveAbsolute(pos) => vec![
                ("pan".to_string(), pos.pan.to_string()),
                ("tilt".to_string(), pos.tilt.to_string()),
                ("zoom".to_string(), pos.zoom.to_string()),
            ],
            PtzCommand::MoveRelative { pan, tilt, zoom } => vec![
                ("rel_pan".to_string(), pan.to_string()),
                ("rel_tilt".to_string(), tilt.to_string()),
                ("rel_zoom".to_string(), zoom.to_string()),
            ],
            PtzCommand::Stop => vec![("command".to_string(), "stop".to_string())],
            PtzCommand::Home => vec![("command".to_string(), "home".to_string())],
            PtzCommand::SavePreset(id) => vec![
                ("command".to_string(), "save_preset".to_string()),
                ("preset".to_string(), id.to_string()),
            ],
            PtzCommand::RecallPreset(id) => vec![
                ("command".to_string(), "recall_preset".to_string()),
                ("preset".to_string(), id.to_string()),
            ],
            PtzCommand::SetFocus(value) => vec![("focus".to_string(), value.to_string())],
            PtzCommand::AutoFocus => vec![("command".to_string(), "autofocus".to_string())],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ptz_position_clamping() {
        let pos = PtzPosition::new(2.0, -2.0, 1.5);
        assert_eq!(pos.pan, 1.0);
        assert_eq!(pos.tilt, -1.0);
        assert_eq!(pos.zoom, 1.0);
    }

    #[test]
    fn test_ptz_home() {
        let home = PtzPosition::home();
        assert_eq!(home.pan, 0.0);
        assert_eq!(home.tilt, 0.0);
        assert_eq!(home.zoom, 0.0);
    }
}
