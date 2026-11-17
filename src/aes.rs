//! Synthesized by DecodeIQ CodeTruth Engine (AST-SLS Synaptic Topology) — Target: `tiny_aes`
//! Reverse engineered & lifted from `kokke/tiny-AES-c` (AES-128 / AES-192 / AES-256 in ECB, CBC, CTR modes).
//! Algorithmic `safe_core` enforces `#![forbid(unsafe_code)]`.
//! Zero raw pointer casting (`(state_t*)buf`), zero UB, guaranteed bound-checked matrix operations.
#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, unused_parens, unused_assignments, unused_imports, unused_mut, dead_code, clippy::too_many_arguments, clippy::many_single_char_names, clippy::excessive_precision, clippy::cast_possible_truncation, clippy::cast_sign_loss, clippy::cast_possible_wrap, clippy::cast_lossless, clippy::float_cmp, clippy::manual_range_contains, clippy::double_parens, clippy::needless_return, clippy::unreadable_literal, clippy::identity_op, clippy::erasing_op, clippy::assign_op_pattern, clippy::collapsible_if, clippy::collapsible_else_if, clippy::needless_bool, clippy::comparison_chain, clippy::let_and_return, clippy::redundant_else, clippy::eq_op, clippy::overly_complex_bool_expr, clippy::precedence, clippy::unnecessary_cast, clippy::nonminimal_bool, clippy::redundant_field_names, clippy::redundant_comparisons, clippy::impossible_comparisons, clippy::unnecessary_to_owned, clippy::needless_range_loop, clippy::missing_safety_doc)]

pub const AES_BLOCKLEN: usize = 16;
pub const AES_KEYEXP_SIZE: usize = 240; // Max for AES-256

#[repr(C)]
#[derive(Clone, Copy)]
pub struct AES_ctx {
    pub round_key: [u8; AES_KEYEXP_SIZE],
    pub iv: [u8; AES_BLOCKLEN],
    pub key_len: usize,
    pub nr: usize,
    pub nk: usize,
}

impl Default for AES_ctx {
    fn default() -> Self {
        Self {
            round_key: [0u8; AES_KEYEXP_SIZE],
            iv: [0u8; AES_BLOCKLEN],
            key_len: 16,
            nr: 10,
            nk: 4,
        }
    }
}

pub mod safe_core {
    #![forbid(unsafe_code)]
    use super::*;

    const SBOX: [u8; 256] = [
        0x63, 0x7c, 0x77, 0x7b, 0xf2, 0x6b, 0x6f, 0xc5, 0x30, 0x01, 0x67, 0x2b, 0xfe, 0xd7, 0xab, 0x76,
        0xca, 0x82, 0xc9, 0x7d, 0xfa, 0x59, 0x47, 0xf0, 0xad, 0xd4, 0xa2, 0xaf, 0x9c, 0xa4, 0x72, 0xc0,
        0xb7, 0xfd, 0x93, 0x26, 0x36, 0x3f, 0xf7, 0xcc, 0x34, 0xa5, 0xe5, 0xf1, 0x71, 0xd8, 0x31, 0x15,
        0x04, 0xc7, 0x23, 0xc3, 0x18, 0x96, 0x05, 0x9a, 0x07, 0x12, 0x80, 0xe2, 0xeb, 0x27, 0xb2, 0x75,
        0x09, 0x83, 0x2c, 0x1a, 0x1b, 0x6e, 0x5a, 0xa0, 0x52, 0x3b, 0xd6, 0xb3, 0x29, 0xe3, 0x2f, 0x84,
        0x53, 0xd1, 0x00, 0xed, 0x20, 0xfc, 0xb1, 0x5b, 0x6a, 0xcb, 0xbe, 0x39, 0x4a, 0x4c, 0x58, 0xcf,
        0xd0, 0xef, 0xaa, 0xfb, 0x43, 0x4d, 0x33, 0x85, 0x45, 0xf9, 0x02, 0x7f, 0x50, 0x3c, 0x9f, 0xa8,
        0x51, 0xa3, 0x40, 0x8f, 0x92, 0x9d, 0x38, 0xf5, 0xbc, 0xb6, 0xda, 0x21, 0x10, 0xff, 0xf3, 0xd2,
        0xcd, 0x0c, 0x13, 0xec, 0x5f, 0x97, 0x44, 0x17, 0xc4, 0xa7, 0x7e, 0x3d, 0x64, 0x5d, 0x19, 0x73,
        0x60, 0x81, 0x4f, 0xdc, 0x22, 0x2a, 0x90, 0x88, 0x46, 0xee, 0xb8, 0x14, 0xde, 0x5e, 0x0b, 0xdb,
        0xe0, 0x32, 0x3a, 0x0a, 0x49, 0x06, 0x24, 0x5c, 0xc2, 0xd3, 0xac, 0x62, 0x91, 0x95, 0xe4, 0x79,
        0xe7, 0xc8, 0x37, 0x6d, 0x8d, 0xd5, 0x4e, 0xa9, 0x6c, 0x56, 0xf4, 0xea, 0x65, 0x7a, 0xae, 0x08,
        0xba, 0x78, 0x25, 0x2e, 0x1c, 0xa6, 0xb4, 0xc6, 0xe8, 0xdd, 0x74, 0x1f, 0x4b, 0xbd, 0x8b, 0x8a,
        0x70, 0x3e, 0xb5, 0x66, 0x48, 0x03, 0xf6, 0x0e, 0x61, 0x35, 0x57, 0xb9, 0x86, 0xc1, 0x1d, 0x9e,
        0xe1, 0xf8, 0x98, 0x11, 0x69, 0xd9, 0x8e, 0x94, 0x9b, 0x1e, 0x87, 0xe9, 0xce, 0x55, 0x28, 0xdf,
        0x8c, 0xa1, 0x89, 0x0d, 0xbf, 0xe6, 0x42, 0x68, 0x41, 0x99, 0x2d, 0x0f, 0xb0, 0x54, 0xbb, 0x16,
    ];

    const RSBOX: [u8; 256] = [
        0x52, 0x09, 0x6a, 0xd5, 0x30, 0x36, 0xa5, 0x38, 0xbf, 0x40, 0xa3, 0x9e, 0x81, 0xf3, 0xd7, 0xfb,
        0x7c, 0xe3, 0x39, 0x82, 0x9b, 0x2f, 0xff, 0x87, 0x34, 0x8e, 0x43, 0x44, 0xc4, 0xde, 0xe9, 0xcb,
        0x54, 0x7b, 0x94, 0x32, 0xa6, 0xc2, 0x23, 0x3d, 0xee, 0x4c, 0x95, 0x0b, 0x42, 0xfa, 0xc3, 0x4e,
        0x08, 0x2e, 0xa1, 0x66, 0x28, 0xd9, 0x24, 0xb2, 0x76, 0x5b, 0xa2, 0x49, 0x6d, 0x8b, 0xd1, 0x25,
        0x72, 0xf8, 0xf6, 0x64, 0x86, 0x68, 0x98, 0x16, 0xd4, 0xa4, 0x5c, 0xcc, 0x5d, 0x65, 0xb6, 0x92,
        0x6c, 0x70, 0x48, 0x50, 0xfd, 0xed, 0xb9, 0xda, 0x5e, 0x15, 0x46, 0x57, 0xa7, 0x8d, 0x9d, 0x84,
        0x90, 0xd8, 0xab, 0x00, 0x8c, 0xbc, 0xd3, 0x0a, 0xf7, 0xe4, 0x58, 0x05, 0xb8, 0xb3, 0x45, 0x06,
        0xd0, 0x2c, 0x1e, 0x8f, 0xca, 0x3f, 0x0f, 0x02, 0xc1, 0xaf, 0xbd, 0x03, 0x01, 0x13, 0x8a, 0x6b,
        0x3a, 0x91, 0x11, 0x41, 0x4f, 0x67, 0xdc, 0xea, 0x97, 0xf2, 0xcf, 0xce, 0xf0, 0xb4, 0xe6, 0x73,
        0x96, 0xac, 0x74, 0x22, 0xe7, 0xad, 0x35, 0x85, 0xe2, 0xf9, 0x37, 0xe8, 0x1c, 0x75, 0xdf, 0x6e,
        0x47, 0xf1, 0x1a, 0x71, 0x1d, 0x29, 0xc5, 0x89, 0x6f, 0xb7, 0x62, 0x0e, 0xaa, 0x18, 0xbe, 0x1b,
        0xfc, 0x56, 0x3e, 0x4b, 0xc6, 0xd2, 0x79, 0x20, 0x9a, 0xdb, 0xc0, 0xfe, 0x78, 0xcd, 0x5a, 0xf4,
        0x1f, 0xdd, 0xa8, 0x33, 0x88, 0x07, 0xc7, 0x31, 0xb1, 0x12, 0x10, 0x59, 0x27, 0x80, 0xec, 0x5f,
        0x60, 0x51, 0x7f, 0xa9, 0x19, 0xb5, 0x4a, 0x0d, 0x2d, 0xe5, 0x7a, 0x9f, 0x93, 0xc9, 0x9c, 0xef,
        0xa0, 0xe0, 0x3b, 0x4d, 0xae, 0x2a, 0xf5, 0xb0, 0xc8, 0xeb, 0xbb, 0x3c, 0x83, 0x53, 0x99, 0x61,
        0x17, 0x2b, 0x04, 0x7e, 0xba, 0x77, 0xd6, 0x26, 0xe1, 0x69, 0x14, 0x63, 0x55, 0x21, 0x0c, 0x7d,
    ];

    const RCON: [u8; 11] = [0x8d, 0x01, 0x02, 0x04, 0x08, 0x10, 0x20, 0x40, 0x80, 0x1b, 0x36];

    #[inline]
    const fn xtime(x: u8) -> u8 {
        (x << 1) ^ (((x >> 7) & 1) * 0x1b)
    }

    #[inline]
    const fn multiply(x: u8, y: u8) -> u8 {
        ((y & 1) * x)
            ^ (((y >> 1) & 1) * xtime(x))
            ^ (((y >> 2) & 1) * xtime(xtime(x)))
            ^ (((y >> 3) & 1) * xtime(xtime(xtime(x))))
            ^ (((y >> 4) & 1) * xtime(xtime(xtime(xtime(x)))))
    }

    pub fn key_expansion(round_key: &mut [u8], key: &[u8], nk: usize, nr: usize) {
        for i in 0..nk {
            round_key[i * 4] = key[i * 4];
            round_key[i * 4 + 1] = key[i * 4 + 1];
            round_key[i * 4 + 2] = key[i * 4 + 2];
            round_key[i * 4 + 3] = key[i * 4 + 3];
        }

        let mut tempa = [0u8; 4];
        for i in nk..(4 * (nr + 1)) {
            let k = (i - 1) * 4;
            tempa[0] = round_key[k];
            tempa[1] = round_key[k + 1];
            tempa[2] = round_key[k + 2];
            tempa[3] = round_key[k + 3];

            if i % nk == 0 {
                let u8tmp = tempa[0];
                tempa[0] = tempa[1];
                tempa[1] = tempa[2];
                tempa[2] = tempa[3];
                tempa[3] = u8tmp;

                tempa[0] = SBOX[tempa[0] as usize];
                tempa[1] = SBOX[tempa[1] as usize];
                tempa[2] = SBOX[tempa[2] as usize];
                tempa[3] = SBOX[tempa[3] as usize];

                tempa[0] ^= RCON[i / nk];
            }

            if nk == 8 && i % nk == 4 {
                tempa[0] = SBOX[tempa[0] as usize];
                tempa[1] = SBOX[tempa[1] as usize];
                tempa[2] = SBOX[tempa[2] as usize];
                tempa[3] = SBOX[tempa[3] as usize];
            }

            let j = i * 4;
            let prev_k = (i - nk) * 4;
            round_key[j] = round_key[prev_k] ^ tempa[0];
            round_key[j + 1] = round_key[prev_k + 1] ^ tempa[1];
            round_key[j + 2] = round_key[prev_k + 2] ^ tempa[2];
            round_key[j + 3] = round_key[prev_k + 3] ^ tempa[3];
        }
    }

    pub fn init_ctx_safe(ctx: &mut AES_ctx, key: &[u8], key_bits: usize) {
        match key_bits {
            192 => {
                ctx.key_len = 24;
                ctx.nk = 6;
                ctx.nr = 12;
            }
            256 => {
                ctx.key_len = 32;
                ctx.nk = 8;
                ctx.nr = 14;
            }
            _ => {
                ctx.key_len = 16;
                ctx.nk = 4;
                ctx.nr = 10;
            }
        }
        key_expansion(&mut ctx.round_key, key, ctx.nk, ctx.nr);
    }

    pub fn init_ctx_iv_safe(ctx: &mut AES_ctx, key: &[u8], iv: &[u8], key_bits: usize) {
        init_ctx_safe(ctx, key, key_bits);
        ctx.iv.copy_from_slice(&iv[..AES_BLOCKLEN]);
    }

    pub fn ctx_set_iv_safe(ctx: &mut AES_ctx, iv: &[u8]) {
        ctx.iv.copy_from_slice(&iv[..AES_BLOCKLEN]);
    }

    #[inline]
    fn add_round_key(round: usize, state: &mut [[u8; 4]; 4], round_key: &[u8]) {
        for i in 0..4 {
            for j in 0..4 {
                state[i][j] ^= round_key[round * 16 + i * 4 + j];
            }
        }
    }

    #[inline]
    fn sub_bytes(state: &mut [[u8; 4]; 4]) {
        for i in 0..4 {
            for j in 0..4 {
                state[j][i] = SBOX[state[j][i] as usize];
            }
        }
    }

    #[inline]
    fn inv_sub_bytes(state: &mut [[u8; 4]; 4]) {
        for i in 0..4 {
            for j in 0..4 {
                state[j][i] = RSBOX[state[j][i] as usize];
            }
        }
    }

    #[inline]
    fn shift_rows(state: &mut [[u8; 4]; 4]) {
        // Rotate row 1
        let temp = state[0][1];
        state[0][1] = state[1][1];
        state[1][1] = state[2][1];
        state[2][1] = state[3][1];
        state[3][1] = temp;

        // Rotate row 2
        let temp = state[0][2];
        state[0][2] = state[2][2];
        state[2][2] = temp;
        let temp = state[1][2];
        state[1][2] = state[3][2];
        state[3][2] = temp;

        // Rotate row 3
        let temp = state[0][3];
        state[0][3] = state[3][3];
        state[3][3] = state[2][3];
        state[2][3] = state[1][3];
        state[1][3] = temp;
    }

    #[inline]
    fn inv_shift_rows(state: &mut [[u8; 4]; 4]) {
        // Rotate row 1 right
        let temp = state[3][1];
        state[3][1] = state[2][1];
        state[2][1] = state[1][1];
        state[1][1] = state[0][1];
        state[0][1] = temp;

        // Rotate row 2 right
        let temp = state[0][2];
        state[0][2] = state[2][2];
        state[2][2] = temp;
        let temp = state[1][2];
        state[1][2] = state[3][2];
        state[3][2] = temp;

        // Rotate row 3 right
        let temp = state[0][3];
        state[0][3] = state[1][3];
        state[1][3] = state[2][3];
        state[2][3] = state[3][3];
        state[3][3] = temp;
    }

    #[inline]
    fn mix_columns(state: &mut [[u8; 4]; 4]) {
        for i in 0..4 {
            let t = state[i][0];
            let tmp = state[i][0] ^ state[i][1] ^ state[i][2] ^ state[i][3];

            let mut tm = state[i][0] ^ state[i][1];
            tm = xtime(tm);
            state[i][0] ^= tm ^ tmp;

            let mut tm = state[i][1] ^ state[i][2];
            tm = xtime(tm);
            state[i][1] ^= tm ^ tmp;

            let mut tm = state[i][2] ^ state[i][3];
            tm = xtime(tm);
            state[i][2] ^= tm ^ tmp;

            let mut tm = state[i][3] ^ t;
            tm = xtime(tm);
            state[i][3] ^= tm ^ tmp;
        }
    }

    #[inline]
    fn inv_mix_columns(state: &mut [[u8; 4]; 4]) {
        for i in 0..4 {
            let a = state[i][0];
            let b = state[i][1];
            let c = state[i][2];
            let d = state[i][3];

            state[i][0] = multiply(a, 0x0e) ^ multiply(b, 0x0b) ^ multiply(c, 0x0d) ^ multiply(d, 0x09);
            state[i][1] = multiply(a, 0x09) ^ multiply(b, 0x0e) ^ multiply(c, 0x0b) ^ multiply(d, 0x0d);
            state[i][2] = multiply(a, 0x0d) ^ multiply(b, 0x09) ^ multiply(c, 0x0e) ^ multiply(d, 0x0b);
            state[i][3] = multiply(a, 0x0b) ^ multiply(b, 0x0d) ^ multiply(c, 0x09) ^ multiply(d, 0x0e);
        }
    }

    pub fn cipher_block(buf: &mut [u8; AES_BLOCKLEN], round_key: &[u8], nr: usize) {
        let mut state = [[0u8; 4]; 4];
        for i in 0..4 {
            for j in 0..4 {
                state[i][j] = buf[i * 4 + j];
            }
        }

        add_round_key(0, &mut state, round_key);

        for round in 1..nr {
            sub_bytes(&mut state);
            shift_rows(&mut state);
            mix_columns(&mut state);
            add_round_key(round, &mut state, round_key);
        }

        sub_bytes(&mut state);
        shift_rows(&mut state);
        add_round_key(nr, &mut state, round_key);

        for i in 0..4 {
            for j in 0..4 {
                buf[i * 4 + j] = state[i][j];
            }
        }
    }

    pub fn inv_cipher_block(buf: &mut [u8; AES_BLOCKLEN], round_key: &[u8], nr: usize) {
        let mut state = [[0u8; 4]; 4];
        for i in 0..4 {
            for j in 0..4 {
                state[i][j] = buf[i * 4 + j];
            }
        }

        add_round_key(nr, &mut state, round_key);

        for round in (1..nr).rev() {
            inv_shift_rows(&mut state);
            inv_sub_bytes(&mut state);
            add_round_key(round, &mut state, round_key);
            inv_mix_columns(&mut state);
        }

        inv_shift_rows(&mut state);
        inv_sub_bytes(&mut state);
        add_round_key(0, &mut state, round_key);

        for i in 0..4 {
            for j in 0..4 {
                buf[i * 4 + j] = state[i][j];
            }
        }
    }

    pub fn ecb_encrypt_safe(ctx: &AES_ctx, buf: &mut [u8; AES_BLOCKLEN]) {
        cipher_block(buf, &ctx.round_key, ctx.nr);
    }

    pub fn ecb_decrypt_safe(ctx: &AES_ctx, buf: &mut [u8; AES_BLOCKLEN]) {
        inv_cipher_block(buf, &ctx.round_key, ctx.nr);
    }

    pub fn cbc_encrypt_buffer_safe(ctx: &mut AES_ctx, buf: &mut [u8]) {
        let len = buf.len();
        let mut iv = ctx.iv;
        let mut offset = 0;

        while offset + AES_BLOCKLEN <= len {
            for i in 0..AES_BLOCKLEN {
                buf[offset + i] ^= iv[i];
            }
            let mut block = [0u8; AES_BLOCKLEN];
            block.copy_from_slice(&buf[offset..offset + AES_BLOCKLEN]);
            cipher_block(&mut block, &ctx.round_key, ctx.nr);
            buf[offset..offset + AES_BLOCKLEN].copy_from_slice(&block);
            iv = block;
            offset += AES_BLOCKLEN;
        }

        ctx.iv = iv;
    }

    pub fn cbc_decrypt_buffer_safe(ctx: &mut AES_ctx, buf: &mut [u8]) {
        let len = buf.len();
        let mut offset = 0;

        while offset + AES_BLOCKLEN <= len {
            let mut store_next_iv = [0u8; AES_BLOCKLEN];
            store_next_iv.copy_from_slice(&buf[offset..offset + AES_BLOCKLEN]);

            let mut block = store_next_iv;
            inv_cipher_block(&mut block, &ctx.round_key, ctx.nr);

            for i in 0..AES_BLOCKLEN {
                buf[offset + i] = block[i] ^ ctx.iv[i];
            }

            ctx.iv = store_next_iv;
            offset += AES_BLOCKLEN;
        }
    }

    pub fn ctr_xcrypt_buffer_safe(ctx: &mut AES_ctx, buf: &mut [u8]) {
        let mut buffer = [0u8; AES_BLOCKLEN];
        let mut bi = AES_BLOCKLEN;

        for byte in buf.iter_mut() {
            if bi == AES_BLOCKLEN {
                buffer = ctx.iv;
                cipher_block(&mut buffer, &ctx.round_key, ctx.nr);

                // Increment 128-bit big-endian counter
                for k in (0..AES_BLOCKLEN).rev() {
                    if ctx.iv[k] == 255 {
                        ctx.iv[k] = 0;
                    } else {
                        ctx.iv[k] += 1;
                        break;
                    }
                }
                bi = 0;
            }

            *byte ^= buffer[bi];
            bi += 1;
        }
    }
}

pub use safe_core::*;


