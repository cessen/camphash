const TEST_VECTORS: &[(&[u8], &str)] = &[
    (&[], "e8970a983d211daa8e4f118531820efe"),
    (&[0], "442ba25cf355109c5a67e43baeee2884"),
    (b"0123456789", "b23c6e285240cf759ab4b017ff06f8b1"),
    (b"abcdefghijklmnopqrstuvwxyz", "f10a141bcc55c4c22807c266fbc5e3c9"),
    (b"this is 16 bytes", "f83562ecf27b3ae1ef8f56744085857c"),
    (b"This string is exactly 32 bytes.", "c5aa5c4e0556ee6e09bc975be6f0d894"),
    (b"The quick brown fox jumps over the lazy dog.", "2deb358867cbb26fe76cf3f704f17264"),
    (
        b"You don't know about me without you have read a book by the name of The Adventures of Tom Sawyer; but that ain't no matter. That book was made by Mr. Mark Twain, and he told the truth, mainly. There was things which he stretched, but mainly he told the truth. That is nothing. I never seen anybody but lied one time or another, without it was Aunt Polly, or the widow, or maybe Mary. Aunt Polly--Tom's Aunt Polly, she is--and Mary, and the Widow Douglas is all told about in that book, which is mostly a true book, with some stretchers, as I said before.",
        "6c04f4f95f8c29728814bf569361ea6b",
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
