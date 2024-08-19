#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(test)]
mod test {
    use crate::{
        wiredtiger_checksum_crc32c, WIREDTIGER_VERSION_MAJOR, WIREDTIGER_VERSION_MINOR,
        WIREDTIGER_VERSION_PATCH, WIREDTIGER_VERSION_STRING,
    };

    #[test]
    fn version_check() {
        // If this test fails, you probably need to bump the crate version too.
        assert_eq!(
            [
                WIREDTIGER_VERSION_MAJOR,
                WIREDTIGER_VERSION_MINOR,
                WIREDTIGER_VERSION_PATCH
            ],
            [3, 1, 0]
        );
    }
}
