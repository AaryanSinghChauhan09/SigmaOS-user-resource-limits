// Integration test for SigmaOS-user-resource-limits
// Verifies the crate builds and links as a dependency.
#![allow(unused_imports)]

use user_resource_limits as _;

#[test]
fn crate_links_and_runs() {
    assert_eq!(2 + 2, 4);
}
