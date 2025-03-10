const TEST_VECTORS: &[(&[u8], &str)] = &[
    (&[], "e9b81ae936aa1c0b41dd1d5e9ef113b0"),
    (&[0], "ab58c150434c100754a4abba17ca712e"),
    (b"0123456789", "c766242a05087d6985174cbb95b81ebd"),
    (b"abcdefghijklmnopqrstuvwxyz", "5cf5a9690009b3ecfcc446c428ea2104"),
    (b"this is 16 bytes", "7c9d132f3348860872cbd651ab0f9db7"),
    (b"This string is exactly 32 bytes.", "dd4f5e8181af0326ced21d1a29562bd8"),
    (b"The quick brown fox jumps over the lazy dog.", "2825a3a32e58f531bb1e90c8f54a6efa"),
    (
        b"Lorem ipsum dolor sit amet, consectetur adipisicing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.",
        "7b24c01f19878070cbedbcc1a2de0e6b",
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
