// Integration test for SigmaOS-logging
// Verifies the crate builds and links as a dependency.
#![allow(unused_imports)]

use logging as _;

#[test]
fn crate_links_and_runs() {
    // The crate compiled and linked; exercise a no-op assertion.
    assert_eq!(2 + 2, 4);
}
