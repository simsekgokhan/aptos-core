#[test_only]
module std::ol_vdf_tests {
    use std::ol_vdf;
    use std::ol_debug;
    use std::ol_test_fixtures;

    #[test]
    fun extract_address() {
        let challenge = ol_test_fixtures::eve_0_easy_chal();
        // Parse key and check
        let (eve_addr, _auth_key) = ol_vdf::extract_address_from_challenge(&challenge);
        ol_debug::print(&eve_addr);
        ol_debug::print(&_auth_key);
        assert!(eve_addr == @0x3DC18D1CF61FAAC6AC70E3A63F062E4B, 7357401001);    
    }
}