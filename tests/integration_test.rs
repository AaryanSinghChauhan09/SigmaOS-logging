use logging::Component;

#[test]
fn test_component_lifecycle_aaa() {
    let mut c = Component::new();
    assert!(!c.is_ready());
    c.init().unwrap();
    assert!(c.is_ready());
}

#[test]
fn test_run_requires_init_aaa() {
    let c = Component::new();
    assert!(c.run().is_err());
}

#[test]
fn test_run_after_init_aaa() {
    let mut c = Component::new();
    c.init().unwrap();
    assert!(c.run().is_ok());
}

#[test]
fn test_reset_then_run_fails_aaa() {
    let mut c = Component::new();
    c.init().unwrap();
    c.reset();
    assert!(c.run().is_err());
}
