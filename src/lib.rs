use core::arch::x86_64::{__m128i, _mm_aesenc_si128, _mm_loadu_si128, _mm_xor_si128};
use core::mem::transmute;

// Seed values for each corresponding lane.
const SEEDS: [__m128i; 9] = [
    unsafe {
        transmute([
            0x6a_u8, 0xb0, 0xe3, 0x06, 0xe1, 0xfc, 0xa4, 0xfc, 0xe3, 0xac, 0x62, 0xc1, 0x22, 0xa4,
            0xdf, 0x5a,
        ])
    },
    unsafe {
        transmute([
            0xc2_u8, 0x14, 0x88, 0x29, 0x94, 0xdb, 0x67, 0xc3, 0x44, 0x97, 0xdd, 0x64, 0x05, 0xa2,
            0x77, 0x7b,
        ])
    },
    unsafe {
        transmute([
            0xfa_u8, 0xcc, 0x0e, 0xb3, 0xcb, 0xf3, 0xc5, 0x8c, 0xe1, 0xa0, 0xbb, 0x2b, 0x0b, 0x3c,
            0x74, 0x02,
        ])
    },
    unsafe {
        transmute([
            0x5b_u8, 0x59, 0x41, 0x06, 0x08, 0xb0, 0x8c, 0xfb, 0x46, 0x04, 0xb2, 0xa7, 0x46, 0xfc,
            0x38, 0x0e,
        ])
    },
    unsafe {
        transmute([
            0xe1_u8, 0xff, 0x10, 0x20, 0x61, 0xf6, 0xa6, 0x87, 0x8c, 0x9c, 0xfe, 0x9f, 0x01, 0xc6,
            0xc8, 0x32,
        ])
    },
    unsafe {
        transmute([
            0x24_u8, 0x35, 0x3e, 0xd2, 0xc3, 0xee, 0x39, 0x65, 0xd9, 0x0d, 0x50, 0xe2, 0x3d, 0x3f,
            0x24, 0x77,
        ])
    },
    unsafe {
        transmute([
            0x7f_u8, 0x27, 0x68, 0x93, 0xd1, 0x3f, 0x43, 0xd4, 0x2b, 0x5c, 0x37, 0x17, 0xb4, 0x5d,
            0x97, 0xfd,
        ])
    },
    unsafe {
        transmute([
            0x0d_u8, 0x1f, 0x4a, 0xe3, 0x7e, 0x24, 0x9c, 0xf0, 0x48, 0x47, 0xc9, 0x3a, 0xba, 0x09,
            0x09, 0x27,
        ])
    },
    unsafe {
        transmute([
            0x76_u8, 0xf1, 0xd3, 0x3a, 0xce, 0xb4, 0x95, 0xf9, 0x53, 0xd0, 0xad, 0x63, 0xed, 0x2f,
            0x9c, 0xe0,
        ])
    },
];

#[inline(never)]
pub fn hash256(data: &[u8]) -> [u8; 16] {
    unsafe {
        let mut state: [__m128i; 9] = SEEDS;

        process_data(&mut state, data);

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
            state[0] = _mm_aesenc_si128(state[0], _mm_loadu_si128(data_ptr));
            state[0] = _mm_aesenc_si128(state[0], SEEDS[0]);
            state[0] = _mm_aesenc_si128(state[0], SEEDS[0]);
        }

        transmute::<_, [u8; 16]>(state[0])
    }
}

#[inline(always)]
unsafe fn process_data(state: &mut [__m128i; 9], data: &[u8]) {
    let mut data = data;

    // Bulk processing with nine lanes.
    while data.len() >= (48 * 3) {
        for i in [0, 3, 6] {
            let data_ptr_0: *const __m128i = transmute(data.as_ptr());
            let data_ptr_1: *const __m128i = transmute((&data[16..]).as_ptr());
            let data_ptr_2: *const __m128i = transmute((&data[24..]).as_ptr());
            state[i + 0] = _mm_aesenc_si128(state[i + 0], _mm_loadu_si128(data_ptr_0));
            state[i + 1] = _mm_aesenc_si128(state[i + 1], _mm_loadu_si128(data_ptr_1));
            state[i + 2] = _mm_aesenc_si128(state[i + 2], _mm_loadu_si128(data_ptr_2));
            data = &data[48..];
        }

        for i in [0, 3, 6] {
            state[i + 0] = _mm_aesenc_si128(state[i + 0], SEEDS[i + 0]);
            state[i + 1] = _mm_aesenc_si128(state[i + 1], SEEDS[i + 1]);
            state[i + 2] = _mm_aesenc_si128(state[i + 2], SEEDS[i + 2]);
        }

        for i in [0, 3, 6] {
            state[i + 0] = _mm_aesenc_si128(state[i + 0], SEEDS[i + 0]);
            state[i + 1] = _mm_aesenc_si128(state[i + 1], SEEDS[i + 1]);
            state[i + 2] = _mm_aesenc_si128(state[i + 2], SEEDS[i + 2]);
        }
    }

    // Bulk processing with three lanes.
    while data.len() >= (16 * 3) {
        for i in [0, 1, 2] {
            let data_ptr_0: *const __m128i = transmute(data.as_ptr());
            state[i] = _mm_aesenc_si128(state[i], _mm_loadu_si128(data_ptr_0));
            data = &data[16..];
        }

        for i in [0, 1, 2] {
            state[i] = _mm_aesenc_si128(state[i], SEEDS[i]);
        }

        for i in [0, 1, 2] {
            state[i] = _mm_aesenc_si128(state[i], SEEDS[i]);
        }
    }

    // Bulk processing with one lane.
    while data.len() >= 16 {
        let data_ptr_0: *const __m128i = transmute(data.as_ptr());
        state[0] = _mm_aesenc_si128(state[0], _mm_loadu_si128(data_ptr_0));
        state[0] = _mm_aesenc_si128(state[0], SEEDS[0]);
        state[0] = _mm_aesenc_si128(state[0], SEEDS[0]);
        data = &data[16..];
    }

    // Last tail, with less data than a single lane.
    if data.len() > 0 {
        let mut buffer = [0u8; 16];
        (&mut buffer[..data.len()]).copy_from_slice(data);

        let data_ptr_0: *const __m128i = transmute(buffer.as_ptr());
        state[0] = _mm_aesenc_si128(state[0], _mm_loadu_si128(data_ptr_0));
        state[0] = _mm_aesenc_si128(state[0], SEEDS[0]);
        state[0] = _mm_aesenc_si128(state[0], SEEDS[0]);
    }
}
