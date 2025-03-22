const HUCKLEBERRY_FINN_INTRO: &str =
"You don't know about me without you have read a book by the name of The Adventures of Tom Sawyer; but that ain't no matter. That book was made by Mr. Mark Twain, and he told the truth, mainly. There was things which he stretched, but mainly he told the truth. That is nothing. I never seen anybody but lied one time or another, without it was Aunt Polly, or the widow, or maybe Mary. Aunt Polly--Tom's Aunt Polly, she is--and Mary, and the Widow Douglas is all told about in that book, which is mostly a true book, with some stretchers, as I said before.

Now the way that the book winds up is this: Tom and me found the money that the robbers hid in the cave, and it made us rich. We got six thousand dollars apiece--all gold. It was an awful sight of money when it was piled up. Well, Judge Thatcher he took it and put it out at interest, and it fetched us a dollar a day apiece all the year round--more than a body could tell what to do with. The Widow Douglas she took me for her son, and allowed she would sivilize me; but it was rough living in the house all the time, considering how dismal regular and decent the widow was in all her ways; and so when I couldn't stand it no longer I lit out. I got into my old rags and my sugar-hogshead again, and was free and satisfied. But Tom Sawyer he hunted me up and said he was going to start a band of robbers, and I might join if I would go back to the widow and be respectable. So I went back.";

const TEST_VECTORS: &[(&[u8], &str)] = &[
    (&[], "e8970a983d211daa8e4f118531820efe"),
    (&[0], "442ba25cf355109c5a67e43baeee2884"),
    (b"0123456789", "b23c6e285240cf759ab4b017ff06f8b1"),
    (
        b"abcdefghijklmnopqrstuvwxyz",
        "4ebe49faac96737f278d0ff07d9e2428",
    ),
    (b"this is 16 bytes", "f83562ecf27b3ae1ef8f56744085857c"),
    (
        b"This string is exactly 32 bytes.",
        "b0be6dda84589ae0842fbfb12e992691",
    ),
    (
        b"The quick brown fox jumps over the lazy dog.",
        "868667962fcd0a70c3f4f43a8d1e9374",
    ),
    (
        HUCKLEBERRY_FINN_INTRO.as_bytes(),
        "618da34386744572aaa8cc4ec891729f",
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

        // Also test the reference implemenation, to make sure we're consistent
        // with it.
        assert_eq!(digest_to_string(&camphash::hash_ref(data)), digest);
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
