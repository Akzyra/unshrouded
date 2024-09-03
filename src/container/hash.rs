#![allow(dead_code)]

use fnv_rs::{Fnv32, FnvHasher};

const XZ: crc::Crc<u64> = crc::Crc::<u64>::new(&crc::CRC_64_XZ);

pub fn crc64(bytes: &[u8]) -> u64 {
    XZ.checksum(bytes)
}

pub fn fnv1a32(bytes: &[u8]) -> u32 {
    u32::from_be_bytes(Fnv32::hash(bytes).as_bytes().try_into().unwrap())
}

#[cfg(test)]
mod tests {
    use crate::container::hash::{crc64, fnv1a32};

    #[test]
    fn test_crc64() {
        assert_eq!(crc64("resourceids.txt".as_bytes()), 0x983ff32191171bc0);
        assert_eq!(
            crc64("600d1357-a74e-4ad9-8f22-2f1ecab8e7ec_e64ee036_0".as_bytes()),
            0xf82db56f9d04b033,
        );
    }

    #[test]
    fn test_fnv1a32() {
        assert_eq!(fnv1a32("keen::ResourceId".as_bytes()), 2802512579);
        assert_eq!(fnv1a32("keen::ContentHash".as_bytes()), 2543063895);
    }
}
