// Integration test for SigmaOS-arch-fragment-shader-kernel
// Verifies the crate builds and links as a dependency.
#![allow(unused_imports)]

use arch_fragment_shader_kernel as _;

#[test]
fn crate_links_and_runs() {
    assert_eq!(2 + 2, 4);
}
