//! Integration tests for SigmaOS-user-resource-limits.

use sigmaos_user_resource_limits_lib::Component;

#[test]
fn component_initialises_and_runs() {
    let mut c = Component::new();
    c.init().expect("init should succeed");
    c.run().expect("run should succeed after init");
}
