//! Synthesized by DecodeIQ CodeTruth Engine (AST-SLS Synaptic Topology) — Target: `ctr_mode`
//! Algorithmic `safe_core` enforces `#![forbid(unsafe_code)]`.
#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, unused_parens, unused_assignments, unused_imports, unused_mut, dead_code, clippy::too_many_arguments, clippy::many_single_char_names, clippy::excessive_precision, clippy::cast_possible_truncation, clippy::cast_sign_loss, clippy::cast_possible_wrap, clippy::cast_lossless, clippy::float_cmp, clippy::manual_range_contains, clippy::double_parens, clippy::needless_return, clippy::unreadable_literal, clippy::identity_op, clippy::erasing_op, clippy::assign_op_pattern, clippy::collapsible_if, clippy::collapsible_else_if, clippy::needless_bool, clippy::comparison_chain, clippy::let_and_return, clippy::redundant_else, clippy::eq_op, clippy::overly_complex_bool_expr, clippy::precedence, clippy::unnecessary_cast, clippy::nonminimal_bool, clippy::redundant_field_names, clippy::redundant_comparisons, clippy::impossible_comparisons, clippy::unnecessary_to_owned)]


pub mod safe_core {
    #![forbid(unsafe_code)]
    use super::*;

    pub fn tc_ctr_mode_safe(out: &mut [u8], mut outlen: u32, in_buf: &[u8], mut inlen: u32, ctr: &mut [u8], mut sched: TCAesKeySched_t) -> i32 {
        let mut buffer: [u8; 1] = [0; 1];
        let mut nonce: [u8; 1] = [0; 1];
        let mut block_num: u32 = ((0) as u32);
        let mut i: u32 = ((0) as u32);
        if (((((((out == 0) || (in_buf == 0)) || (ctr == 0)) || (sched == 0)) || (inlen == ((0) as u32))) || (outlen == ((0) as u32))) || (outlen != inlen)) {
            return TC_CRYPTO_FAIL;
        }
        _copy(nonce, sizeof(nonce), ctr, sizeof(nonce));
        block_num = (((((((((nonce[(12) as usize])) as u32))).wrapping_shl((24) as u32)) | ((((((nonce[(13) as usize])) as u32))).wrapping_shl((16) as u32))) | ((((((nonce[(14) as usize])) as u32))).wrapping_shl((8) as u32))) | (((nonce[(15) as usize])) as u32));
        i = ((0) as u32);
        while (i < inlen) {
            if (((i % (((TC_AES_BLOCK_SIZE)) as u32))) == ((0) as u32)) {
                if (tc_aes_encrypt(buffer, nonce, sched)) != 0 {
                    block_num = (block_num).wrapping_add(1);
                    nonce[(12) as usize] = ((((block_num).wrapping_shr((24) as u32))) as u8);
                    nonce[(13) as usize] = ((((block_num).wrapping_shr((16) as u32))) as u8);
                    nonce[(14) as usize] = ((((block_num).wrapping_shr((8) as u32))) as u8);
                    nonce[(15) as usize] = (((block_num)) as u8);
                } else {
                    return TC_CRYPTO_FAIL;
                }
            }
            *out += 1 = (((buffer[((i % (((TC_AES_BLOCK_SIZE)) as u32))) as usize]) as i32) ^ *in_buf += 1);
            i = (i).wrapping_add(1);
        }
        ctr[(12) as usize] = nonce[(12) as usize];
        ctr[(13) as usize] = nonce[(13) as usize];
        ctr[(14) as usize] = nonce[(14) as usize];
        ctr[(15) as usize] = nonce[(15) as usize];
        return TC_CRYPTO_SUCCESS;
    }

}

/// # Safety
/// Pointer arguments must be valid for the duration of the call.
#[no_mangle]
pub unsafe extern "C" fn tc_ctr_mode(out: *mut u8, outlen: u32, in_buf: *const u8, inlen: u32, ctr: *mut u8, sched: TCAesKeySched_t) -> i32 {
    let out_slice = unsafe { core::slice::from_raw_parts_mut(out, 3usize) };
    let in_buf_slice = unsafe { core::slice::from_raw_parts(in_buf, 3usize) };
    let ctr_slice = unsafe { core::slice::from_raw_parts_mut(ctr, 3usize) };
    safe_core::tc_ctr_mode_safe(out_slice, outlen, in_buf_slice, inlen, ctr_slice, sched)
}

