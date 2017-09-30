extern crate hcube;

use hcube::enumeration::piece::all;

#[test]
fn count_all_pieces() {
    assert_eq!(65536, all().count());
}
