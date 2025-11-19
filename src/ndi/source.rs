use serde::{Deserialize, Serialize};
use std::fmt;

/// Represents an NDI source discovered on the network
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct NdiSource {
    pub name: String,
    pub url: String,
    pub groups: Vec<String>,
}

impl NdiSource {
    pub fn new(name: String, url: String) -> Self {
        Self {
            name,
            url,
            groups: Vec::new(),
        }
    }

    #[allow(dead_code)]
    pub fn with_groups(mut self, groups: Vec<String>) -> Self {
        self.groups = groups;
        self
    }
}

impl fmt::Display for NdiSource {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "NDI Source: {} ({})", self.name, self.url)
    }
}
