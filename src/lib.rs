use core::arch::x86_64::{
    __m128i, _mm_aesenc_si128, _mm_loadu_si128, _mm_setzero_si128, _mm_xor_si128,
};
use core::mem::transmute;

const LANES: usize = 24;
const LANES_PER_SET: usize = 6;

// Initial values for each corresponding lane.
const INITIAL_STATE: [__m128i; LANES] = {
    #[rustfmt::skip]
    let seed_bytes: [[u8; 16]; LANES] = [
        [0x3f, 0x53, 0xb6, 0x7f, 0x1c, 0x5e, 0xd2, 0xed, 0x3a, 0x7c, 0x1d, 0xac, 0xef, 0xc6, 0x2a, 0x47],
        [0xe2, 0xd9, 0x59, 0x72, 0xc6, 0x7c, 0x0b, 0xf3, 0x2e, 0xa7, 0xc8, 0xb3, 0xe2, 0xf4, 0xb0, 0xc5],
        [0x1f, 0xe9, 0x68, 0xb2, 0xc7, 0x8f, 0x8e, 0xea, 0x41, 0x50, 0x44, 0xb5, 0xba, 0x8b, 0x81, 0xb8],
        [0x0a, 0x75, 0x71, 0xdc, 0x49, 0x3f, 0x51, 0x4e, 0x05, 0x2a, 0x0c, 0xae, 0x3d, 0x8c, 0x0a, 0xa8],
        [0x36, 0x6d, 0xd3, 0x5c, 0xef, 0x25, 0x30, 0xfc, 0x0d, 0x96, 0x37, 0x67, 0x4a, 0x07, 0x58, 0x59],
        [0x3f, 0x8f, 0x98, 0xff, 0xf4, 0x88, 0xfa, 0x17, 0xbf, 0x5e, 0x92, 0x51, 0xf8, 0x0a, 0x6b, 0x66],

        [0xae, 0x0d, 0xef, 0xd7, 0x66, 0xbb, 0x59, 0x23, 0xc6, 0xa1, 0xa6, 0x72, 0xe0, 0x6d, 0x53, 0x16],
        [0x9e, 0x8f, 0x14, 0x4b, 0xb9, 0x5a, 0x1b, 0xe6, 0x82, 0x2b, 0xc9, 0xec, 0x11, 0x49, 0x41, 0xe1],
        [0xfa, 0x67, 0x8a, 0xb7, 0xdc, 0x61, 0x6a, 0x21, 0x37, 0xd9, 0x6f, 0xe1, 0xe8, 0x86, 0x8a, 0xb6],
        [0xbb, 0x53, 0x8c, 0xb7, 0xc7, 0x29, 0xad, 0xb8, 0x7f, 0x29, 0xf7, 0x2c, 0x84, 0x7f, 0xbe, 0xd7],
        [0xe7, 0x39, 0xf4, 0xc1, 0x56, 0x76, 0x11, 0x7f, 0xb5, 0xf6, 0xea, 0xd2, 0xd3, 0xa8, 0xb2, 0x69],
        [0x6b, 0xa7, 0x9b, 0x73, 0xae, 0x81, 0x08, 0x24, 0x5f, 0x03, 0x73, 0x01, 0x96, 0xfa, 0x5e, 0x3e],

        [0xe6, 0x0a, 0xc2, 0x1f, 0xfb, 0x38, 0xd9, 0xba, 0xa8, 0x2c, 0xc1, 0x1f, 0x38, 0x66, 0x12, 0x91],
        [0x53, 0xef, 0x28, 0xf0, 0xe3, 0xf7, 0x9e, 0x21, 0xc0, 0x17, 0x3e, 0x74, 0x3a, 0xdc, 0xf7, 0xd7],
        [0x31, 0x19, 0x12, 0xbe, 0xf9, 0xad, 0xf4, 0x83, 0x72, 0xbf, 0xb0, 0xff, 0x66, 0xcb, 0x5d, 0x8a],
        [0x01, 0x94, 0xd2, 0xcc, 0x3a, 0xca, 0xf5, 0x5e, 0xf8, 0xca, 0xf4, 0x5d, 0x4b, 0x5c, 0x67, 0xc7],
        [0x88, 0x49, 0xeb, 0xb4, 0x4f, 0x9c, 0xe6, 0xfc, 0x34, 0x02, 0x41, 0x21, 0x1f, 0xc4, 0xbf, 0x02],
        [0x06, 0x56, 0x55, 0x98, 0x55, 0x3c, 0x5a, 0x0c, 0xc6, 0xb8, 0x57, 0x95, 0x09, 0x4c, 0x80, 0x49],

        [0xb0, 0xe4, 0xd9, 0x61, 0xb3, 0x97, 0xe5, 0x6d, 0xb7, 0xde, 0x2e, 0x50, 0x3a, 0x9f, 0xbd, 0x77],
        [0xc5, 0xf2, 0xd8, 0xcd, 0x2a, 0xa8, 0x23, 0xad, 0xa4, 0xaa, 0xd9, 0x3e, 0xd1, 0x94, 0xe3, 0x2e],
        [0x83, 0x22, 0x44, 0x24, 0xa8, 0x5c, 0xf8, 0xa5, 0x62, 0x3e, 0x74, 0x54, 0x5b, 0x00, 0x71, 0xc8],
        [0xaa, 0x4e, 0x95, 0x89, 0x85, 0x47, 0xe5, 0x46, 0x22, 0xe1, 0x82, 0x7a, 0x37, 0x73, 0xa2, 0x33],
        [0x9c, 0xec, 0x81, 0x0b, 0xec, 0x43, 0x59, 0x0d, 0x4b, 0x92, 0x18, 0xa9, 0x35, 0xc6, 0x69, 0x62],
        [0xf1, 0xd5, 0x96, 0xc8, 0x1f, 0x68, 0x34, 0xd8, 0xf2, 0x71, 0x65, 0x43, 0x6c, 0x43, 0x0a, 0x0c],
    ];

    unsafe { transmute(seed_bytes) }
};

// Used in AES mixing.
const XOR_VALUE: __m128i = unsafe {
    #[rustfmt::skip]
    let bytes: [u8; 16] =
        [0xcd, 0xf6, 0x8e, 0xce, 0x8c, 0xfd, 0x77, 0x1e, 0x27, 0x75, 0xa4, 0x47, 0x4d, 0x0e, 0xe6, 0x47];

    transmute(bytes)
};

#[inline(never)]
pub fn hash(data: &[u8]) -> [u8; 16] {
    unsafe {
        let mut state: [__m128i; LANES] = INITIAL_STATE;

        // Process message data.
        let data_tail = process_data_bulk(&mut state, data);
        process_data_tail(&mut state, data_tail);

        // Xor all the lanes together.
        for i in 1..LANES {
            state[0] = _mm_xor_si128(state[0], state[i]);
        }

        // Incorporate the message length.
        {
            let mut buffer = [0u8; 16];
            buffer[0..8].copy_from_slice(&(data.len() as u64).to_le_bytes());

            let data_ptr: *const __m128i = transmute(buffer.as_ptr());
            state[0] = _mm_xor_si128(state[0], _mm_loadu_si128(data_ptr));
            state[0] = _mm_aesenc_si128(state[0], XOR_VALUE);
            state[0] = _mm_aesenc_si128(state[0], XOR_VALUE);
            state[0] = _mm_aesenc_si128(state[0], _mm_setzero_si128());
        }

        transmute::<_, [u8; 16]>(state[0])
    }
}

/// Does the main bulk processing of data that is at least `(16 * LANES)` bytes
/// large.
///
/// This is the main workhorse function, responsible for bulk throughput with
/// large input data.
#[inline(never)]
unsafe fn process_data_bulk<'a>(state: &mut [__m128i; LANES], data: &'a [u8]) -> &'a [u8] {
    let block_size = 16 * LANES;

    // If there's too little data to process, so just bail early.
    if data.len() < block_size {
        return data;
    }

    let mut data = data;

    // Xor initial data.
    for i in 0..LANES {
        let data_ptr: *const __m128i = transmute(data.as_ptr());
        state[i] = _mm_xor_si128(state[i], _mm_loadu_si128(data_ptr));
        data = &data[16..];
    }

    // Mix state and xor subsequent data.
    while data.len() >= block_size {
        let mut cur = 0;
        while cur < LANES {
            let start = cur;
            let end = cur + LANES_PER_SET;

            for i in start..end {
                state[i] = _mm_aesenc_si128(state[i], XOR_VALUE);
            }

            for i in start..end {
                state[i] = _mm_aesenc_si128(state[i], XOR_VALUE);
            }

            for i in start..end {
                // We use the built-in xor at the end of the third AES round to xor in
                // the next data chunks.
                let data_ptr: *const __m128i = transmute(data.as_ptr());
                state[i] = _mm_aesenc_si128(state[i], _mm_loadu_si128(data_ptr));
                data = &data[16..];
            }

            cur += LANES_PER_SET;
        }
    }

    let mut cur = 0;
    while cur < LANES {
        // Mix state for last data.
        let start = cur;
        let end = cur + LANES_PER_SET;

        for i in start..end {
            state[i] = _mm_aesenc_si128(state[i], XOR_VALUE);
        }
        for i in start..end {
            state[i] = _mm_aesenc_si128(state[i], XOR_VALUE);
        }
        for i in start..end {
            state[i] = _mm_aesenc_si128(state[i], _mm_setzero_si128());
        }

        cur += LANES_PER_SET;
    }

    data
}

/// Processes the remaining data after large-chunk processing has already taken
/// care of what it can.
#[inline(never)]
unsafe fn process_data_tail(state: &mut [__m128i; LANES], data: &[u8]) {
    assert!(data.len() < (16 * LANES));

    let mut data = data;

    // Handle full 16-byte chunks.
    let mut lane = 0;
    while data.len() >= 16 {
        let data_ptr: *const __m128i = transmute(data.as_ptr());
        state[lane] = _mm_xor_si128(state[lane], _mm_loadu_si128(data_ptr));
        state[lane] = _mm_aesenc_si128(state[lane], XOR_VALUE);
        state[lane] = _mm_aesenc_si128(state[lane], XOR_VALUE);
        state[lane] = _mm_aesenc_si128(state[lane], _mm_setzero_si128());
        data = &data[16..];

        lane += 1;
    }

    // Last tail, with less data than a single lane.
    if data.len() > 0 {
        let mut buffer = [0u8; 16];
        (&mut buffer[..data.len()]).copy_from_slice(data);

        let data_ptr: *const __m128i = transmute(buffer.as_ptr());
        state[lane] = _mm_xor_si128(state[lane], _mm_loadu_si128(data_ptr));
        state[lane] = _mm_aesenc_si128(state[lane], XOR_VALUE);
        state[lane] = _mm_aesenc_si128(state[lane], XOR_VALUE);
        state[lane] = _mm_aesenc_si128(state[lane], _mm_setzero_si128());
    }
}

//-------------------------------------------------------------
// Simpler reference implementation.  Slower.

#[inline(never)]
pub fn hash_ref(data: &[u8]) -> [u8; 16] {
    let data_len = data.len();

    let mut data = data;
    let mut state: [__m128i; LANES] = INITIAL_STATE;

    unsafe {
        // Process message data.
        {
            let mut lane_i = 0;
            while !data.is_empty() {
                absorb_ref(&mut state[lane_i], data);

                data = &data[16_usize.min(data.len())..];
                lane_i = (lane_i + 1) % state.len();
            }
        }

        // Xor all the lanes together.
        for i in 1..LANES {
            state[0] = _mm_xor_si128(state[0], state[i]);
        }

        // Incorporate the message length.
        absorb_ref(&mut state[0], &(data_len as u64).to_le_bytes());

        transmute::<_, [u8; 16]>(state[0])
    }
}

fn absorb_ref(lane: &mut __m128i, data: &[u8]) {
    let mut buffer = [0u8; 16];
    let copy_len = data.len().min(16);
    buffer[..copy_len].copy_from_slice(&data[..copy_len]);
    let data_ptr: *const __m128i = unsafe { transmute(buffer.as_ptr()) };

    *lane = aes(*lane, unsafe { _mm_loadu_si128(data_ptr) });
    *lane = aes(*lane, XOR_VALUE);
    *lane = aes(*lane, XOR_VALUE);
}

fn aes(state: __m128i, key: __m128i) -> __m128i {
    let mut state = state;

    unsafe {
        state = _mm_xor_si128(state, key);
        state = _mm_aesenc_si128(state, _mm_setzero_si128());
    }

    state
}
