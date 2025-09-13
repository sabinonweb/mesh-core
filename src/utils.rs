use std::collections::HashSet;

pub fn handle_packet(seen: &mut HashSet<u64>, seq: u64, payload: &[u8]) -> bool {
    if seen.contains(&seq) {
        log::info!("Duplicate packet {}", seq);
        false
    } else {
        seen.insert(seq);
        println!("Hashset {:?}", seen);
        log::info!("Received {}: {}", seq, String::from_utf8_lossy(payload));
        true
    }
}
