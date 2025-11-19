use super::NdiSource;
use anyhow::Result;
use log::{debug, info, warn};
use std::sync::{Arc, Mutex};
use std::time::Duration;
use tokio::time;

/// NDI source discovery service
pub struct NdiDiscovery {
    sources: Arc<Mutex<Vec<NdiSource>>>,
    is_running: Arc<Mutex<bool>>,
}

impl NdiDiscovery {
    pub fn new() -> Self {
        Self {
            sources: Arc::new(Mutex::new(Vec::new())),
            is_running: Arc::new(Mutex::new(false)),
        }
    }

    /// Start automatic NDI source discovery
    pub async fn start(&self) -> Result<()> {
        let mut is_running = self.is_running.lock().unwrap();
        if *is_running {
            warn!("Discovery already running");
            return Ok(());
        }
        *is_running = true;
        drop(is_running);

        info!("Starting NDI source discovery...");

        let sources = Arc::clone(&self.sources);
        let is_running = Arc::clone(&self.is_running);

        tokio::spawn(async move {
            while {
                let running = *is_running.lock().unwrap();
                running
            } {
                // Simulate NDI source discovery
                // In a real implementation, this would use the NDI SDK's find functionality
                debug!("Scanning for NDI sources...");

                // For now, we'll create a mock discovery mechanism
                // Real implementation would use ndi-sdk crate's finder
                let discovered = Self::discover_ndi_sources().await;

                {
                    let mut sources_lock = sources.lock().unwrap();
                    *sources_lock = discovered;
                }

                time::sleep(Duration::from_secs(5)).await;
            }
        });

        Ok(())
    }

    /// Stop the discovery process
    pub fn stop(&self) {
        let mut is_running = self.is_running.lock().unwrap();
        *is_running = false;
        info!("Stopped NDI source discovery");
    }

    /// Get currently discovered sources
    pub fn get_sources(&self) -> Vec<NdiSource> {
        self.sources.lock().unwrap().clone()
    }

    /// Internal method to discover NDI sources
    async fn discover_ndi_sources() -> Vec<NdiSource> {
        // This is a placeholder implementation
        // Real implementation would use the NDI SDK's finder API
        //
        // Example real implementation would look like:
        // let finder = ndi::Finder::new();
        // finder.wait_for_sources(timeout);
        // let sources = finder.get_current_sources();

        debug!("Discovering NDI sources on network...");

        // Return mock sources for demonstration
        // In production, this would query the actual NDI network
        vec![]
    }

    /// Manually add a source (useful for static sources)
    #[allow(dead_code)]
    pub fn add_source(&self, source: NdiSource) {
        let mut sources = self.sources.lock().unwrap();
        if !sources.iter().any(|s| s.url == source.url) {
            info!("Added NDI source: {}", source);
            sources.push(source);
        }
    }

    /// Remove a source by URL
    #[allow(dead_code)]
    pub fn remove_source(&self, url: &str) -> bool {
        let mut sources = self.sources.lock().unwrap();
        let len_before = sources.len();
        sources.retain(|s| s.url != url);
        sources.len() < len_before
    }
}

impl Default for NdiDiscovery {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_discovery_start_stop() {
        let discovery = NdiDiscovery::new();
        assert!(discovery.start().await.is_ok());
        discovery.stop();
    }

    #[test]
    fn test_add_remove_source() {
        let discovery = NdiDiscovery::new();
        let source = NdiSource::new("Test Source".to_string(), "ndi://test".to_string());

        discovery.add_source(source.clone());
        assert_eq!(discovery.get_sources().len(), 1);

        assert!(discovery.remove_source(&source.url));
        assert_eq!(discovery.get_sources().len(), 0);
    }
}
