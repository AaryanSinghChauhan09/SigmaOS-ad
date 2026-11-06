// SPDX-License-Identifier: Apache-2.0
//! SigmaOS-ad - System utility component
//! Part of SigmaOS multi-repository architecture


/// Component
pub struct Component {
    name: &'static str,
}

impl Component {
    pub fn new() -> Self {
        Self { name: "SigmaOS-ad" }
    }
}

impl Default for Component {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_creation() {
        let c = super::Component::new();
        assert_eq!(c.name, "SigmaOS-ad");
    }
}

pub mod config;
pub mod error;
pub mod service;

pub use config::Config;
pub use error::{Error, Result};
