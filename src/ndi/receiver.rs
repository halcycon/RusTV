use super::NdiSource;
use anyhow::Result;
use log::{debug, info};
use std::sync::{Arc, Mutex};

/// NDI receiver for viewing streams
pub struct NdiReceiver {
    source: Option<NdiSource>,
    is_active: Arc<Mutex<bool>>,
}

impl NdiReceiver {
    pub fn new() -> Self {
        Self {
            source: None,
            is_active: Arc::new(Mutex::new(false)),
        }
    }

    /// Connect to an NDI source
    pub fn connect(&mut self, source: NdiSource) -> Result<()> {
        info!("Connecting to NDI source: {}", source);

        // In a real implementation, this would use the NDI SDK's receiver API
        // Example:
        // let recv = ndi::Receiver::new();
        // recv.connect(&source);

        self.source = Some(source.clone());
        let mut is_active = self.is_active.lock().unwrap();
        *is_active = true;

        info!("Successfully connected to: {}", source.name);
        Ok(())
    }

    /// Disconnect from the current source
    pub fn disconnect(&mut self) {
        if let Some(source) = &self.source {
            info!("Disconnecting from: {}", source.name);
        }

        let mut is_active = self.is_active.lock().unwrap();
        *is_active = false;
        self.source = None;
    }

    /// Check if receiver is currently active
    pub fn is_active(&self) -> bool {
        *self.is_active.lock().unwrap()
    }

    /// Get current source
    #[allow(dead_code)]
    pub fn current_source(&self) -> Option<NdiSource> {
        self.source.clone()
    }

    /// Get video frame (placeholder for actual frame retrieval)
    pub fn receive_video_frame(&self) -> Result<()> {
        if !self.is_active() {
            anyhow::bail!("Receiver is not active");
        }

        // In real implementation:
        // let frame = recv.capture_video(timeout);
        // Process the frame data

        debug!("Receiving video frame...");
        Ok(())
    }

    /// Get audio frame (placeholder for actual frame retrieval)
    #[allow(dead_code)]
    pub fn receive_audio_frame(&self) -> Result<()> {
        if !self.is_active() {
            anyhow::bail!("Receiver is not active");
        }

        // In real implementation:
        // let frame = recv.capture_audio(timeout);
        // Process the frame data

        debug!("Receiving audio frame...");
        Ok(())
    }

    /// Get metadata (placeholder)
    #[allow(dead_code)]
    pub fn receive_metadata(&self) -> Result<String> {
        if !self.is_active() {
            anyhow::bail!("Receiver is not active");
        }

        // In real implementation:
        // let metadata = recv.capture_metadata(timeout);

        Ok(String::from("{}"))
    }
}

impl Default for NdiReceiver {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_receiver_connect_disconnect() {
        let mut receiver = NdiReceiver::new();
        let source = NdiSource::new("Test".to_string(), "ndi://test".to_string());

        assert!(!receiver.is_active());
        assert!(receiver.connect(source).is_ok());
        assert!(receiver.is_active());

        receiver.disconnect();
        assert!(!receiver.is_active());
    }
}
