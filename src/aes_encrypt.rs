//! Synthesized by DecodeIQ CodeTruth Engine (AST-SLS Synaptic Topology) — Target: `aes_encrypt`
//! Algorithmic `safe_core` enforces `#![forbid(unsafe_code)]`.
#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, unused_parens, unused_assignments, unused_imports, unused_mut, dead_code, clippy::too_many_arguments, clippy::many_single_char_names, clippy::excessive_precision, clippy::cast_possible_truncation, clippy::cast_sign_loss, clippy::cast_possible_wrap, clippy::cast_lossless, clippy::float_cmp, clippy::manual_range_contains, clippy::double_parens, clippy::needless_return, clippy::unreadable_literal, clippy::identity_op, clippy::erasing_op, clippy::assign_op_pattern, clippy::collapsible_if, clippy::collapsible_else_if, clippy::needless_bool, clippy::comparison_chain, clippy::let_and_return, clippy::redundant_else, clippy::eq_op, clippy::overly_complex_bool_expr, clippy::precedence, clippy::unnecessary_cast, clippy::nonminimal_bool, clippy::redundant_field_names, clippy::redundant_comparisons, clippy::impossible_comparisons, clippy::unnecessary_to_owned)]


pub mod safe_core {
    #![forbid(unsafe_code)]
    use super::*;

    pub const sbox: [u8; 256] = [
	0x63, 0x7c, 0x77, 0x7b, 0xf2, 0x6b, 0x6f, 0xc5, 0x30, 0x01, 0x67, 0x2b,
	0xfe, 0xd7, 0xab, 0x76, 0xca, 0x82, 0xc9, 0x7d, 0xfa, 0x59, 0x47, 0xf0,
	0xad, 0xd4, 0xa2, 0xaf, 0x9c, 0xa4, 0x72, 0xc0, 0xb7, 0xfd, 0x93, 0x26,
	0x36, 0x3f, 0xf7, 0xcc, 0x34, 0xa5, 0xe5, 0xf1, 0x71, 0xd8, 0x31, 0x15,
	0x04, 0xc7, 0x23, 0xc3, 0x18, 0x96, 0x05, 0x9a, 0x07, 0x12, 0x80, 0xe2,
	0xeb, 0x27, 0xb2, 0x75, 0x09, 0x83, 0x2c, 0x1a, 0x1b, 0x6e, 0x5a, 0xa0,
	0x52, 0x3b, 0xd6, 0xb3, 0x29, 0xe3, 0x2f, 0x84, 0x53, 0xd1, 0x00, 0xed,
	0x20, 0xfc, 0xb1, 0x5b, 0x6a, 0xcb, 0xbe, 0x39, 0x4a, 0x4c, 0x58, 0xcf,
	0xd0, 0xef, 0xaa, 0xfb, 0x43, 0x4d, 0x33, 0x85, 0x45, 0xf9, 0x02, 0x7f,
	0x50, 0x3c, 0x9f, 0xa8, 0x51, 0xa3, 0x40, 0x8f, 0x92, 0x9d, 0x38, 0xf5,
	0xbc, 0xb6, 0xda, 0x21, 0x10, 0xff, 0xf3, 0xd2, 0xcd, 0x0c, 0x13, 0xec,
	0x5f, 0x97, 0x44, 0x17, 0xc4, 0xa7, 0x7e, 0x3d, 0x64, 0x5d, 0x19, 0x73,
	0x60, 0x81, 0x4f, 0xdc, 0x22, 0x2a, 0x90, 0x88, 0x46, 0xee, 0xb8, 0x14,
	0xde, 0x5e, 0x0b, 0xdb, 0xe0, 0x32, 0x3a, 0x0a, 0x49, 0x06, 0x24, 0x5c,
	0xc2, 0xd3, 0xac, 0x62, 0x91, 0x95, 0xe4, 0x79, 0xe7, 0xc8, 0x37, 0x6d,
	0x8d, 0xd5, 0x4e, 0xa9, 0x6c, 0x56, 0xf4, 0xea, 0x65, 0x7a, 0xae, 0x08,
	0xba, 0x78, 0x25, 0x2e, 0x1c, 0xa6, 0xb4, 0xc6, 0xe8, 0xdd, 0x74, 0x1f,
	0x4b, 0xbd, 0x8b, 0x8a, 0x70, 0x3e, 0xb5, 0x66, 0x48, 0x03, 0xf6, 0x0e,
	0x61, 0x35, 0x57, 0xb9, 0x86, 0xc1, 0x1d, 0x9e, 0xe1, 0xf8, 0x98, 0x11,
	0x69, 0xd9, 0x8e, 0x94, 0x9b, 0x1e, 0x87, 0xe9, 0xce, 0x55, 0x28, 0xdf,
	0x8c, 0xa1, 0x89, 0x0d, 0xbf, 0xe6, 0x42, 0x68, 0x41, 0x99, 0x2d, 0x0f,
	0xb0, 0x54, 0xbb, 0x16
];

    pub const rconst: [u32; 11] = [
		0x00000000, 0x01000000, 0x02000000, 0x04000000, 0x08000000, 0x10000000,
		0x20000000, 0x40000000, 0x80000000, 0x1b000000, 0x36000000
	];

    pub fn rotword(a: u32) -> u32 {
        return (((((a)).wrapping_shr((24) as u32)) | (((a)).wrapping_shl((8) as u32))));
    }

    pub fn tc_aes128_set_encrypt_key_safe(mut s: TCAesKeySched_t, k: &[u8]) -> i32 {
        let mut i: u32 = ((0) as u32);
        let mut t: u32 = ((0) as u32);
        if (s == 0) {
            return TC_CRYPTO_FAIL;
        } else if (k == 0) {
            return TC_CRYPTO_FAIL;
        }
        i = ((0) as u32);
        while (i < ((Nk) as u32)) {
            s.words[(i) as usize] = (((((((((((k[((((Nb) as u32)).wrapping_mul(i)) as usize])) as u32))).wrapping_shl((24) as u32)) | ((((((k[(((((Nb) as u32)).wrapping_mul(i)).wrapping_add(((1) as u32))) as usize])) as u32))).wrapping_shl((16) as u32))) | ((((((k[(((((Nb) as u32)).wrapping_mul(i)).wrapping_add(((2) as u32))) as usize])) as u32))).wrapping_shl((8) as u32))) | (((k[(((((Nb) as u32)).wrapping_mul(i)).wrapping_add(((3) as u32))) as usize])) as u32))) as i32);
            i = (i).wrapping_add(1);
        }
        while (i < ((((Nb * ((Nr + 1))))) as u32)) {
            t = ((s.words[((i).wrapping_sub(((1) as u32))) as usize]) as u32);
            if (((i % ((Nk) as u32))) == ((0) as u32)) {
                t = (((((((((sbox[(((((rotword(t))).wrapping_shr(((24)) as u32)) & ((0xff) as u32))) as usize]) as u32)).wrapping_shl(((24)) as u32)) | ((((sbox[(((((rotword(t))).wrapping_shr(((16)) as u32)) & ((0xff) as u32))) as usize]) as u32)).wrapping_shl(((16)) as u32))) | ((((sbox[(((((rotword(t))).wrapping_shr(((8)) as u32)) & ((0xff) as u32))) as usize]) as u32)).wrapping_shl(((8)) as u32))) | ((((sbox[(((((rotword(t))).wrapping_shr(((0)) as u32)) & ((0xff) as u32))) as usize]) as u32)).wrapping_shl(((0)) as u32)))) ^ rconst[((i / ((Nk) as u32))) as usize]);
            }
            s.words[(i) as usize] = (((((s.words[((i).wrapping_sub(((Nk) as u32))) as usize]) as u32) ^ t)) as i32);
            i = (i).wrapping_add(1);
        }
        return TC_CRYPTO_SUCCESS;
    }

    pub fn add_round_key(s: &mut [u8], k: &[u32]) {
        s[(0) as usize] ^= ((((k[(0) as usize]).wrapping_shr((24) as u32))) as u8);
        s[(1) as usize] ^= ((((k[(0) as usize]).wrapping_shr((16) as u32))) as u8);
        s[(2) as usize] ^= ((((k[(0) as usize]).wrapping_shr((8) as u32))) as u8);
        s[(3) as usize] ^= (((k[(0) as usize])) as u8);
        s[(4) as usize] ^= ((((k[(1) as usize]).wrapping_shr((24) as u32))) as u8);
        s[(5) as usize] ^= ((((k[(1) as usize]).wrapping_shr((16) as u32))) as u8);
        s[(6) as usize] ^= ((((k[(1) as usize]).wrapping_shr((8) as u32))) as u8);
        s[(7) as usize] ^= (((k[(1) as usize])) as u8);
        s[(8) as usize] ^= ((((k[(2) as usize]).wrapping_shr((24) as u32))) as u8);
        s[(9) as usize] ^= ((((k[(2) as usize]).wrapping_shr((16) as u32))) as u8);
        s[(10) as usize] ^= ((((k[(2) as usize]).wrapping_shr((8) as u32))) as u8);
        s[(11) as usize] ^= (((k[(2) as usize])) as u8);
        s[(12) as usize] ^= ((((k[(3) as usize]).wrapping_shr((24) as u32))) as u8);
        s[(13) as usize] ^= ((((k[(3) as usize]).wrapping_shr((16) as u32))) as u8);
        s[(14) as usize] ^= ((((k[(3) as usize]).wrapping_shr((8) as u32))) as u8);
        s[(15) as usize] ^= (((k[(3) as usize])) as u8);
    }

    pub fn sub_bytes(s: &mut [u8]) {
        let mut i: u32 = ((0) as u32);
        i = ((0) as u32);
        while (i < ((((Nb * Nk))) as u32)) {
            s[(i) as usize] = sbox[(s[(i) as usize]) as usize];
            i = (i).wrapping_add(1);
        }
    }

    pub fn mult_row_column(out: &mut [u8], in_buf: &[u8]) {
        out[(0) as usize] = (((((_double_byte(in_buf[(0) as usize]) ^ ((_double_byte((in_buf[(1) as usize])) ^ (((in_buf[(1) as usize])) as i32)))) ^ ((in_buf[(2) as usize]) as i32)) ^ ((in_buf[(3) as usize]) as i32))) as u8);
        out[(1) as usize] = (((((((in_buf[(0) as usize]) as i32) ^ _double_byte(in_buf[(1) as usize])) ^ ((_double_byte((in_buf[(2) as usize])) ^ (((in_buf[(2) as usize])) as i32)))) ^ ((in_buf[(3) as usize]) as i32))) as u8);
        out[(2) as usize] = (((((((in_buf[(0) as usize]) as i32) ^ ((in_buf[(1) as usize]) as i32)) ^ _double_byte(in_buf[(2) as usize])) ^ ((_double_byte((in_buf[(3) as usize])) ^ (((in_buf[(3) as usize])) as i32))))) as u8);
        out[(3) as usize] = (((((((_double_byte((in_buf[(0) as usize])) ^ (((in_buf[(0) as usize])) as i32))) ^ ((in_buf[(1) as usize]) as i32)) ^ ((in_buf[(2) as usize]) as i32)) ^ _double_byte(in_buf[(3) as usize]))) as u8);
    }

    pub fn mix_columns(s: &mut [u8]) {
        let mut t: [u8; 1] = [0; 1];
        mult_row_column(&mut t, s);
        mult_row_column(&mut t[(Nb) as usize..], (s + Nb));
        mult_row_column(&mut t[((2 * Nb)) as usize..], (s + ((2 * Nb))));
        mult_row_column(&mut t[((3 * Nb)) as usize..], (s + ((3 * Nb))));
        _copy(s, sizeof(t), t, sizeof(t));
    }

    pub fn shift_rows(s: &mut [u8]) {
        let mut t: [u8; 1] = [0; 1];
        t[(0) as usize] = s[(0) as usize];
        t[(1) as usize] = s[(5) as usize];
        t[(2) as usize] = s[(10) as usize];
        t[(3) as usize] = s[(15) as usize];
        t[(4) as usize] = s[(4) as usize];
        t[(5) as usize] = s[(9) as usize];
        t[(6) as usize] = s[(14) as usize];
        t[(7) as usize] = s[(3) as usize];
        t[(8) as usize] = s[(8) as usize];
        t[(9) as usize] = s[(13) as usize];
        t[(10) as usize] = s[(2) as usize];
        t[(11) as usize] = s[(7) as usize];
        t[(12) as usize] = s[(12) as usize];
        t[(13) as usize] = s[(1) as usize];
        t[(14) as usize] = s[(6) as usize];
        t[(15) as usize] = s[(11) as usize];
        _copy(s, sizeof(t), t, sizeof(t));
    }

    pub fn tc_aes_encrypt_safe(out: &mut [u8], in_buf: &[u8], mut s: TCAesKeySched_t) -> i32 {
        let mut state: [u8; 1] = [0; 1];
        let mut i: u32 = ((0) as u32);
        if (out == 0) {
            return TC_CRYPTO_FAIL;
        } else if (in_buf == 0) {
            return TC_CRYPTO_FAIL;
        } else if (s == 0) {
            return TC_CRYPTO_FAIL;
        }
        _copy(state, sizeof(state), in_buf, sizeof(state));
        add_round_key(&mut state, &{ let _tmp = s.words; _tmp });
        i = ((0) as u32);
        while (i < ((((Nr - 1))) as u32)) {
            sub_bytes(&mut state);
            shift_rows(&mut state);
            mix_columns(&mut state);
            add_round_key(&mut state, &{ let _tmp = (((s.words) as u32)).wrapping_add((((Nb) as u32)).wrapping_mul(((i).wrapping_add(((1) as u32))))); _tmp });
            i = (i).wrapping_add(1);
        }
        sub_bytes(&mut state);
        shift_rows(&mut state);
        add_round_key(&mut state, &{ let _tmp = (((s.words) as u32)).wrapping_add((((Nb) as u32)).wrapping_mul(((i).wrapping_add(((1) as u32))))); _tmp });
        _copy(out, sizeof(state), state, sizeof(state));
        _set(state, TC_ZERO_BYTE, sizeof(state));
        return TC_CRYPTO_SUCCESS;
    }

}

/// # Safety
/// Pointer arguments must be valid for the duration of the call.
#[no_mangle]
pub unsafe extern "C" fn tc_aes128_set_encrypt_key(s: TCAesKeySched_t, k: *const u8) -> i32 {
    let k_slice = unsafe { core::slice::from_raw_parts(k, 3usize) };
    safe_core::tc_aes128_set_encrypt_key_safe(s, k_slice)
}

/// # Safety
/// Pointer arguments must be valid for the duration of the call.
#[no_mangle]
pub unsafe extern "C" fn tc_aes_encrypt(out: *mut u8, in_buf: *const u8, s: TCAesKeySched_t) -> i32 {
    let out_slice = unsafe { core::slice::from_raw_parts_mut(out, 3usize) };
    let in_buf_slice = unsafe { core::slice::from_raw_parts(in_buf, 3usize) };
    safe_core::tc_aes_encrypt_safe(out_slice, in_buf_slice, s)
}

