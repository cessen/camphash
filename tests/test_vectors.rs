const TEST_VECTORS: &[(&[u8], &str)] = &[
    (&[], "ff93753796465584e24962026ff5db9c"),
    (&[0], "c324614fc09cc5913bbb87d8eb9cb778"),
    (b"0123456789", "8726f89a886a719431b5a5163a3afa8c"),
    (b"abcdefghijklmnopqrstuvwxyz", "2994002e16ebc402c76768419b458824"),
    (b"this is 16 bytes", "853b156237d0ddc94be88a13bb645c67"),
    (b"This string is exactly 32 bytes.", "3b65682040574ae7af2da973bba06dee"),
    (b"The quick brown fox jumps over the lazy dog.", "6924f291abdd13c4ad3a6226ed64f787"),
    (
        b"Lorem ipsum dolor sit amet, consectetur adipisicing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.",
        "cc8221211e4966b74bf054903f2e12c7",
    ),
];

/// Returns a printable hex string version of the digest.
pub fn digest_to_string(digest: &[u8]) -> String {
    fn low_bits_to_char(n: u8) -> char {
        match n {
            0 => '0',
            1 => '1',
            2 => '2',
            3 => '3',
            4 => '4',
            5 => '5',
            6 => '6',
            7 => '7',
            8 => '8',
            9 => '9',
            10 => 'a',
            11 => 'b',
            12 => 'c',
            13 => 'd',
            14 => 'e',
            15 => 'f',
            _ => unreachable!(),
        }
    }

    let mut s = String::new();
    for byte in digest.iter() {
        s.push(low_bits_to_char(byte >> 4u8));
        s.push(low_bits_to_char(byte & 0b00001111));
    }
    s
}

#[test]
fn single_call() {
    for (data, digest) in TEST_VECTORS.iter().copied() {
        assert_eq!(digest_to_string(&camphash::hash(data)), digest);
    }
}

// #[test]
// fn streaming_one_chunk() {
//     for (data, digest) in TEST_VECTORS.iter().copied() {
//         let mut hasher = CampHash::new();
//         hasher.update(data);
//         assert_eq!(digest_to_string(&hasher.finalize()), digest);
//     }
// }

// #[test]
// fn streaming_multi_chunk() {
//     for chunk_size in 1..1024 {
//         for (data, digest) in TEST_VECTORS.iter().copied() {
//             if data.len() >= chunk_size {
//                 let mut hasher = CampHash::new();
//                 for chunk in data.chunks(chunk_size) {
//                     hasher.update(chunk);
//                 }
//                 assert_eq!(digest_to_string(&hasher.finalize()), digest);
//             }
//         }
//     }
// }
