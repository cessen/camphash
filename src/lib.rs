use core::arch::x86_64::{
    __m128i, _mm_aesenc_si128, _mm_loadu_si128, _mm_setzero_si128, _mm_xor_si128,
};
use core::mem::transmute;

// Seed values for each corresponding lane.
const SEEDS: [__m128i; 9] = {
    let seed_bytes: [[u8; 16]; 9] = [
        [
            0x6a, 0xb0, 0xe3, 0x06, 0xe1, 0xfc, 0xa4, 0xfc, 0xe3, 0xac, 0x62, 0xc1, 0x22, 0xa4,
            0xdf, 0x5a,
        ],
        [
            0xc2, 0x14, 0x88, 0x29, 0x94, 0xdb, 0x67, 0xc3, 0x44, 0x97, 0xdd, 0x64, 0x05, 0xa2,
            0x77, 0x7b,
        ],
        [
            0xfa, 0xcc, 0x0e, 0xb3, 0xcb, 0xf3, 0xc5, 0x8c, 0xe1, 0xa0, 0xbb, 0x2b, 0x0b, 0x3c,
            0x74, 0x02,
        ],
        [
            0x5b, 0x59, 0x41, 0x06, 0x08, 0xb0, 0x8c, 0xfb, 0x46, 0x04, 0xb2, 0xa7, 0x46, 0xfc,
            0x38, 0x0e,
        ],
        [
            0xe1, 0xff, 0x10, 0x20, 0x61, 0xf6, 0xa6, 0x87, 0x8c, 0x9c, 0xfe, 0x9f, 0x01, 0xc6,
            0xc8, 0x32,
        ],
        [
            0x24, 0x35, 0x3e, 0xd2, 0xc3, 0xee, 0x39, 0x65, 0xd9, 0x0d, 0x50, 0xe2, 0x3d, 0x3f,
            0x24, 0x77,
        ],
        [
            0x7f, 0x27, 0x68, 0x93, 0xd1, 0x3f, 0x43, 0xd4, 0x2b, 0x5c, 0x37, 0x17, 0xb4, 0x5d,
            0x97, 0xfd,
        ],
        [
            0x0d, 0x1f, 0x4a, 0xe3, 0x7e, 0x24, 0x9c, 0xf0, 0x48, 0x47, 0xc9, 0x3a, 0xba, 0x09,
            0x09, 0x27,
        ],
        [
            0x76, 0xf1, 0xd3, 0x3a, 0xce, 0xb4, 0x95, 0xf9, 0x53, 0xd0, 0xad, 0x63, 0xed, 0x2f,
            0x9c, 0xe0,
        ],
    ];

    unsafe { transmute(seed_bytes) }
};

#[inline(never)]
pub fn hash256(data: &[u8]) -> [u8; 16] {
    unsafe {
        let mut state: [__m128i; 9] = SEEDS;

        // Process message data.
        let data_tail = process_data_bulk(&mut state, data);
        process_data_tail(&mut state, data_tail);

        // Xor all the lanes together.
        {
            state[0] = _mm_xor_si128(state[0], state[4]);
            state[1] = _mm_xor_si128(state[1], state[5]);
            state[2] = _mm_xor_si128(state[2], state[6]);
            state[3] = _mm_xor_si128(state[3], state[7]);

            state[0] = _mm_xor_si128(state[0], state[2]);
            state[1] = _mm_xor_si128(state[1], state[3]);

            state[0] = _mm_xor_si128(state[0], state[8]);
        }

        // Incorporate the message length.
        {
            let mut buffer = [0u8; 16];
            buffer[0..8].copy_from_slice(&(data.len() as u64).to_le_bytes());

            let data_ptr: *const __m128i = transmute(buffer.as_ptr());
            state[0] = _mm_xor_si128(state[0], _mm_loadu_si128(data_ptr));
            state[0] = _mm_aesenc_si128(state[0], SEEDS[0]);
            state[0] = _mm_aesenc_si128(state[0], SEEDS[0]);
            state[0] = _mm_aesenc_si128(state[0], _mm_setzero_si128());
        }

        transmute::<_, [u8; 16]>(state[0])
    }
}

/// Does the main bulk processing of data that is at least 144 bytes large,
/// using nine AES lanes.
///
/// This is the main workhorse function, responsible for bulk throughput with
/// large input data.
#[inline(always)]
unsafe fn process_data_bulk<'a>(state: &mut [__m128i; 9], data: &'a [u8]) -> &'a [u8] {
    let mut data = data;

    while data.len() >= (48 * 3) {
        for i in [0, 3, 6] {
            let data_ptr_0: *const __m128i = transmute(data.as_ptr());
            let data_ptr_1: *const __m128i = transmute((&data[16..]).as_ptr());
            let data_ptr_2: *const __m128i = transmute((&data[32..]).as_ptr());

            state[i + 0] = _mm_xor_si128(state[i + 0], _mm_loadu_si128(data_ptr_0));
            state[i + 1] = _mm_xor_si128(state[i + 1], _mm_loadu_si128(data_ptr_1));
            state[i + 2] = _mm_xor_si128(state[i + 2], _mm_loadu_si128(data_ptr_2));

            state[i + 0] = _mm_aesenc_si128(state[i + 0], SEEDS[i + 0]);
            state[i + 1] = _mm_aesenc_si128(state[i + 1], SEEDS[i + 1]);
            state[i + 2] = _mm_aesenc_si128(state[i + 2], SEEDS[i + 2]);

            data = &data[48..];
        }

        for i in [0, 3, 6] {
            state[i + 0] = _mm_aesenc_si128(state[i + 0], SEEDS[i + 0]);
            state[i + 1] = _mm_aesenc_si128(state[i + 1], SEEDS[i + 1]);
            state[i + 2] = _mm_aesenc_si128(state[i + 2], SEEDS[i + 2]);
        }

        for i in [0, 3, 6] {
            state[i + 0] = _mm_aesenc_si128(state[i + 0], _mm_setzero_si128());
            state[i + 1] = _mm_aesenc_si128(state[i + 1], _mm_setzero_si128());
            state[i + 2] = _mm_aesenc_si128(state[i + 2], _mm_setzero_si128());
        }
    }

    data
}

/// Processes the remaining data after all 144-byte chunks have already been processed.
///
/// This is notably slower than the bulk processing.
#[inline(always)]
unsafe fn process_data_tail(state: &mut [__m128i; 9], data: &[u8]) {
    let mut data = data;

    // Bulk processing with three lanes.
    while data.len() >= (16 * 3) {
        for i in [0, 1, 2] {
            let data_ptr: *const __m128i = transmute(data.as_ptr());
            state[i] = _mm_xor_si128(state[i], _mm_loadu_si128(data_ptr));
            state[i] = _mm_aesenc_si128(state[i], SEEDS[i]);
            data = &data[16..];
        }

        for i in [0, 1, 2] {
            state[i] = _mm_aesenc_si128(state[i], SEEDS[i]);
        }

        for i in [0, 1, 2] {
            state[i] = _mm_aesenc_si128(state[i], _mm_setzero_si128());
        }
    }

    // Bulk processing with one lane.
    while data.len() >= 16 {
        let data_ptr: *const __m128i = transmute(data.as_ptr());
        state[0] = _mm_xor_si128(state[0], _mm_loadu_si128(data_ptr));
        state[0] = _mm_aesenc_si128(state[0], SEEDS[0]);
        state[0] = _mm_aesenc_si128(state[0], SEEDS[0]);
        state[0] = _mm_aesenc_si128(state[0], _mm_setzero_si128());
        data = &data[16..];
    }

    // Last tail, with less data than a single lane.
    if data.len() > 0 {
        let mut buffer = [0u8; 16];
        (&mut buffer[..data.len()]).copy_from_slice(data);

        let data_ptr: *const __m128i = transmute(buffer.as_ptr());
        state[0] = _mm_xor_si128(state[0], _mm_loadu_si128(data_ptr));
        state[0] = _mm_aesenc_si128(state[0], SEEDS[0]);
        state[0] = _mm_aesenc_si128(state[0], SEEDS[0]);
        state[0] = _mm_aesenc_si128(state[0], _mm_setzero_si128());
    }
}
