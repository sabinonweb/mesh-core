use std::collections::HashSet;

use mesh_core::utils::handle_packet;

#[test]
fn test_packet_duplication() {
    let mut seen = HashSet::new();

    assert!(handle_packet(&mut seen, 1, b"rustmesh test 1"));
    assert!(handle_packet(&mut seen, 2, b"rustmesh test 2"));

    // duplicate packet
    assert!(!handle_packet(&mut seen, 1, b"rustmesh test 1"));
}
