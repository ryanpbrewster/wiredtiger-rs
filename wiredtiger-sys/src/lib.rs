#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(test)]
mod test {
    #[test]
    fn version_check() {
        // If this test fails, you probably need to bump the crate version too.
        assert_eq!(
            [
                crate::WIREDTIGER_VERSION_MAJOR,
                crate::WIREDTIGER_VERSION_MINOR,
                crate::WIREDTIGER_VERSION_PATCH
            ],
            [10, 0, 0]
        );
    }
}
