//! SigmaOS logging component
#![forbid(unsafe_code)]
#![warn(missing_docs)]

pub mod sigmalogmanager;

use thiserror::Error;

/// Error types for this component.
#[derive(Debug, Error)]
pub enum ComponentError {
    /// Not initialised.
    #[error("component not initialised")]
    NotInitialised,
    /// Unsupported operation.
    #[error("unsupported: {0}")]
    Unsupported(&'static str),
}

/// Result alias.
pub type ComponentResult<T> = Result<T, ComponentError>;

/// Component handle.
#[derive(Debug, Default, Clone)]
pub struct Component { initialised: bool }

impl Component {
    /// Create uninitialised component.
    pub fn new() -> Self { Self::default() }
    /// Initialise.
    pub fn init(&mut self) -> ComponentResult<()> { self.initialised = true; Ok(()) }
    /// Is ready?
    pub fn is_ready(&self) -> bool { self.initialised }
    /// Run.
    pub fn run(&self) -> ComponentResult<()> {
        if !self.initialised { return Err(ComponentError::NotInitialised); }
        Ok(())
    }
    /// Reset.
    pub fn reset(&mut self) { self.initialised = false; }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_new_not_ready() { assert!(!Component::new().is_ready()); }
    #[test]
    fn test_init_ok() { let mut c = Component::new(); c.init().unwrap(); assert!(c.is_ready()); }
    #[test]
    fn test_run_before_init_err() { assert!(Component::new().run().is_err()); }
    #[test]
    fn test_run_after_init_ok() { let mut c = Component::new(); c.init().unwrap(); assert!(c.run().is_ok()); }
    #[test]
    fn test_reset() { let mut c = Component::new(); c.init().unwrap(); c.reset(); assert!(!c.is_ready()); }
}
