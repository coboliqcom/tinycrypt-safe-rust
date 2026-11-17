//! Synthesized by DecodeIQ CodeTruth Engine (AST-SLS Synaptic Topology) — Target: `cbc_mode`
//! Algorithmic `safe_core` enforces `#![forbid(unsafe_code)]`.
#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, unused_parens, unused_assignments, unused_imports, unused_mut, dead_code, clippy::too_many_arguments, clippy::many_single_char_names, clippy::excessive_precision, clippy::cast_possible_truncation, clippy::cast_sign_loss, clippy::cast_possible_wrap, clippy::cast_lossless, clippy::float_cmp, clippy::manual_range_contains, clippy::double_parens, clippy::needless_return, clippy::unreadable_literal, clippy::identity_op, clippy::erasing_op, clippy::assign_op_pattern, clippy::collapsible_if, clippy::collapsible_else_if, clippy::needless_bool, clippy::comparison_chain, clippy::let_and_return, clippy::redundant_else, clippy::eq_op, clippy::overly_complex_bool_expr, clippy::precedence, clippy::unnecessary_cast, clippy::nonminimal_bool, clippy::redundant_field_names, clippy::redundant_comparisons, clippy::impossible_comparisons, clippy::unnecessary_to_owned)]


pub mod safe_core {
    #![forbid(unsafe_code)]
    use super::*;

    pub fn tc_cbc_mode_encrypt_safe(out: &mut [u8], mut outlen: u32, in_buf: &[u8], mut inlen: u32, iv: &[u8], mut sched: TCAesKeySched_t) -> i32 {
        let mut buffer: [u8; 1] = [0; 1];
        let mut n: u32 = ((0) as u32);
        let mut m: u32 = ((0) as u32);
        if ((((((((out == 0) || (in_buf == 0)) || (sched == 0)) || (inlen == ((0) as u32))) || (outlen == ((0) as u32))) || (((inlen % ((TC_AES_BLOCK_SIZE) as u32))) != ((0) as u32))) || (((outlen % ((TC_AES_BLOCK_SIZE) as u32))) != ((0) as u32))) || (outlen != (inlen).wrapping_add(((TC_AES_BLOCK_SIZE) as u32)))) {
            return TC_CRYPTO_FAIL;
        }
        _copy(buffer, TC_AES_BLOCK_SIZE, iv, TC_AES_BLOCK_SIZE);
        _copy(out, TC_AES_BLOCK_SIZE, iv, TC_AES_BLOCK_SIZE);
        out += TC_AES_BLOCK_SIZE;
        n = m = ((0) as u32);
        while (n < inlen) {
            buffer[(m) as usize] ^= *in_buf += 1;
            m += 1;
            if (m == ((TC_AES_BLOCK_SIZE) as u32)) {
                tc_aes_encrypt(buffer, buffer, sched);
                _copy(out, TC_AES_BLOCK_SIZE, buffer, TC_AES_BLOCK_SIZE);
                out += TC_AES_BLOCK_SIZE;
                m = ((0) as u32);
            }
            n = (n).wrapping_add(1);
        }
        return TC_CRYPTO_SUCCESS;
    }

    pub fn tc_cbc_mode_decrypt_safe(out: &mut [u8], mut outlen: u32, in_buf: &[u8], mut inlen: u32, iv: &[u8], mut sched: TCAesKeySched_t) -> i32 {
        let mut buffer: [u8; 1] = [0; 1];
        let mut p: [u8; 4096] = [0; 4096];
        let mut n: u32 = ((0) as u32);
        let mut m: u32 = ((0) as u32);
        if ((((((((out == 0) || (in_buf == 0)) || (sched == 0)) || (inlen == ((0) as u32))) || (outlen == ((0) as u32))) || (((inlen % ((TC_AES_BLOCK_SIZE) as u32))) != ((0) as u32))) || (((outlen % ((TC_AES_BLOCK_SIZE) as u32))) != ((0) as u32))) || (outlen != inlen)) {
            return TC_CRYPTO_FAIL;
        }
        p = iv;
        n = m = ((0) as u32);
        while (n < outlen) {
            if (((n % ((TC_AES_BLOCK_SIZE) as u32))) == ((0) as u32)) {
                tc_aes_decrypt(buffer, in_buf, sched);
                in_buf += TC_AES_BLOCK_SIZE;
                m = ((0) as u32);
            }
            *out += 1 = (((buffer[(m = (m).wrapping_add(1)) as usize]) as i32) ^ *p += 1);
            n = (n).wrapping_add(1);
        }
        return TC_CRYPTO_SUCCESS;
    }

}

/// # Safety
/// Pointer arguments must be valid for the duration of the call.
#[no_mangle]
pub unsafe extern "C" fn tc_cbc_mode_encrypt(out: *mut u8, outlen: u32, in_buf: *const u8, inlen: u32, iv: *const u8, sched: TCAesKeySched_t) -> i32 {
    let out_slice = unsafe { core::slice::from_raw_parts_mut(out, 3usize) };
    let in_buf_slice = unsafe { core::slice::from_raw_parts(in_buf, 3usize) };
    let iv_slice = unsafe { core::slice::from_raw_parts(iv, 3usize) };
    safe_core::tc_cbc_mode_encrypt_safe(out_slice, outlen, in_buf_slice, inlen, iv_slice, sched)
}

/// # Safety
/// Pointer arguments must be valid for the duration of the call.
#[no_mangle]
pub unsafe extern "C" fn tc_cbc_mode_decrypt(out: *mut u8, outlen: u32, in_buf: *const u8, inlen: u32, iv: *const u8, sched: TCAesKeySched_t) -> i32 {
    let out_slice = unsafe { core::slice::from_raw_parts_mut(out, 3usize) };
    let in_buf_slice = unsafe { core::slice::from_raw_parts(in_buf, 3usize) };
    let iv_slice = unsafe { core::slice::from_raw_parts(iv, 3usize) };
    safe_core::tc_cbc_mode_decrypt_safe(out_slice, outlen, in_buf_slice, inlen, iv_slice, sched)
}

