use crate::ndi::NdiSource;
use anyhow::{Context, Result};
use log::{info, warn};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Represents a routing from an input to an output
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Route {
    pub input: String,
    pub output: String,
}

impl Route {
    pub fn new(input: String, output: String) -> Self {
        Self { input, output }
    }
}

/// Matrix router for managing input/output routing
pub struct MatrixRouter {
    routes: HashMap<String, String>,
    inputs: Vec<NdiSource>,
    outputs: Vec<String>,
}

impl MatrixRouter {
    pub fn new() -> Self {
        Self {
            routes: HashMap::new(),
            inputs: Vec::new(),
            outputs: Vec::new(),
        }
    }

    /// Add an input source
    #[allow(dead_code)]
    pub fn add_input(&mut self, source: NdiSource) {
        if !self.inputs.iter().any(|s| s.url == source.url) {
            info!("Added input: {}", source.name);
            self.inputs.push(source);
        }
    }

    /// Add an output destination
    pub fn add_output(&mut self, output: String) {
        if !self.outputs.contains(&output) {
            info!("Added output: {}", output);
            self.outputs.push(output);
        }
    }

    /// Create a route from input to output
    pub fn route(&mut self, input: &str, output: &str) -> Result<()> {
        // Validate input exists
        if !self
            .inputs
            .iter()
            .any(|s| s.url == input || s.name == input)
        {
            anyhow::bail!("Input '{}' not found", input);
        }

        // Validate output exists
        if !self.outputs.contains(&output.to_string()) {
            anyhow::bail!("Output '{}' not found", output);
        }

        info!("Routing {} -> {}", input, output);
        self.routes.insert(output.to_string(), input.to_string());
        Ok(())
    }

    /// Create a placeholder route to an input that may not exist yet
    /// This allows creating routes to NDI sources before they are discovered
    pub fn route_placeholder(&mut self, input: &str, output: &str) -> Result<()> {
        // Validate output exists
        if !self.outputs.contains(&output.to_string()) {
            anyhow::bail!("Output '{}' not found", output);
        }

        info!("Creating placeholder route: {} -> {}", input, output);
        self.routes.insert(output.to_string(), input.to_string());
        Ok(())
    }

    /// Check if an input for a route exists (is not a placeholder)
    pub fn input_exists(&self, input: &str) -> bool {
        self.inputs
            .iter()
            .any(|s| s.url == input || s.name == input)
    }

    /// Remove a route for a specific output
    pub fn unroute(&mut self, output: &str) -> Option<String> {
        if let Some(input) = self.routes.remove(output) {
            info!("Removed route: {} -> {}", input, output);
            Some(input)
        } else {
            warn!("No route found for output: {}", output);
            None
        }
    }

    /// Get current route for an output
    #[allow(dead_code)]
    pub fn get_route(&self, output: &str) -> Option<&String> {
        self.routes.get(output)
    }

    /// Get all current routes
    pub fn get_all_routes(&self) -> Vec<Route> {
        self.routes
            .iter()
            .map(|(output, input)| Route::new(input.clone(), output.clone()))
            .collect()
    }

    /// Get all inputs
    pub fn get_inputs(&self) -> &[NdiSource] {
        &self.inputs
    }

    /// Get all outputs
    pub fn get_outputs(&self) -> &[String] {
        &self.outputs
    }

    /// Clear all routes
    #[allow(dead_code)]
    pub fn clear_routes(&mut self) {
        info!("Clearing all routes");
        self.routes.clear();
    }

    /// Load routes from a configuration
    #[allow(dead_code)]
    pub fn load_routes(&mut self, routes: Vec<Route>) -> Result<()> {
        for route in routes {
            self.route(&route.input, &route.output)
                .with_context(|| format!("Failed to load route: {:?}", route))?;
        }
        Ok(())
    }
}

impl Default for MatrixRouter {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_matrix_routing() {
        let mut router = MatrixRouter::new();

        let source = NdiSource::new("Camera 1".to_string(), "ndi://cam1".to_string());
        router.add_input(source);
        router.add_output("Output 1".to_string());

        assert!(router.route("ndi://cam1", "Output 1").is_ok());
        assert_eq!(
            router.get_route("Output 1"),
            Some(&"ndi://cam1".to_string())
        );

        router.unroute("Output 1");
        assert_eq!(router.get_route("Output 1"), None);
    }

    #[test]
    fn test_invalid_routing() {
        let mut router = MatrixRouter::new();

        // Try to route without adding input/output
        assert!(router.route("ndi://invalid", "Output 1").is_err());
    }
}
