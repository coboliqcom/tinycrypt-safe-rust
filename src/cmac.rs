//! Synthesized by DecodeIQ CodeTruth Engine (AST-SLS Synaptic Topology) — Target: `cmac_mode`
//! Algorithmic `safe_core` enforces `#![forbid(unsafe_code)]`.
#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, unused_parens, unused_assignments, unused_imports, unused_mut, dead_code, clippy::too_many_arguments, clippy::many_single_char_names, clippy::excessive_precision, clippy::cast_possible_truncation, clippy::cast_sign_loss, clippy::cast_possible_wrap, clippy::cast_lossless, clippy::float_cmp, clippy::manual_range_contains, clippy::double_parens, clippy::needless_return, clippy::unreadable_literal, clippy::identity_op, clippy::erasing_op, clippy::assign_op_pattern, clippy::collapsible_if, clippy::collapsible_else_if, clippy::needless_bool, clippy::comparison_chain, clippy::let_and_return, clippy::redundant_else, clippy::eq_op, clippy::overly_complex_bool_expr, clippy::precedence, clippy::unnecessary_cast, clippy::nonminimal_bool, clippy::redundant_field_names, clippy::redundant_comparisons, clippy::impossible_comparisons, clippy::unnecessary_to_owned)]


pub mod safe_core {
    #![forbid(unsafe_code)]
    use super::*;

    pub fn gf_double_safe(out: &mut [u8], in_buf: &mut [u8]) {
        let mut x: u8 = (((in_buf + ((TC_AES_BLOCK_SIZE - 1)))) as u8);
        let mut carry: u8 = ((if ((((in_buf[(0) as usize]) as u32)).wrapping_shr((7) as u32)) != 0 { gf_wrap } else { 0 }) as u8);
        out += ((TC_AES_BLOCK_SIZE - 1));
        while true {
            *out -= 1 = (((((*x) as u32)).wrapping_shl((1) as u32)) ^ ((carry) as u32));
            if (((x) as i32) == in_buf) {
                break;
            }
            carry = (((((*x = (x).wrapping_sub(1)) as u32)).wrapping_shr((7) as u32)) as u8);
        }
    }

    pub fn tc_cmac_setup_safe(mut s: TCCmacState_t, key: &[u8], mut sched: TCAesKeySched_t) -> i32 {
        if ((s == 0) || (key == 0)) {
            return TC_CRYPTO_FAIL;
        }
        _set(s, 0, sizeof(*s));
        s.sched = sched;
        tc_aes128_set_encrypt_key(s.sched, key);
        _set(s.iv, 0, TC_AES_BLOCK_SIZE);
        tc_aes_encrypt(s.iv, s.iv, s.sched);
        gf_double_safe(&mut { let _tmp = s.K1; _tmp }, &mut { let _tmp = s.iv; _tmp });
        gf_double_safe(&mut { let _tmp = s.K2; _tmp }, &mut { let _tmp = s.K1; _tmp });
        tc_cmac_init_safe(s);
        return TC_CRYPTO_SUCCESS;
    }

    pub fn tc_cmac_erase_safe(mut s: TCCmacState_t) -> i32 {
        if (s == 0) {
            return TC_CRYPTO_FAIL;
        }
        _set(s, 0, sizeof(*s));
        return TC_CRYPTO_SUCCESS;
    }

    pub fn tc_cmac_init_safe(mut s: TCCmacState_t) -> i32 {
        if (s == 0) {
            return TC_CRYPTO_FAIL;
        }
        _set(s.iv, 0, TC_AES_BLOCK_SIZE);
        _set(s.leftover, 0, TC_AES_BLOCK_SIZE);
        s.leftover_offset = 0;
        s.countdown = MAX_CALLS;
        return TC_CRYPTO_SUCCESS;
    }

    pub fn tc_cmac_update_safe(mut s: TCCmacState_t, data: &[u8], mut data_length: usize) -> i32 {
        let mut i: u32 = ((0) as u32);
        if (s == 0) {
            return TC_CRYPTO_FAIL;
        }
        if (data_length == ((0) as usize)) {
            return TC_CRYPTO_SUCCESS;
        }
        if (data == 0) {
            return TC_CRYPTO_FAIL;
        }
        if (s.countdown == 0) {
            return TC_CRYPTO_FAIL;
        }
        s.countdown -= 1;
        if (s.leftover_offset > 0) {
            let mut remaining_space: usize = (((TC_AES_BLOCK_SIZE - s.leftover_offset)) as usize);
            if (data_length < remaining_space) {
                _copy(&s.leftover[(s.leftover_offset) as usize..], data_length, data, data_length);
                s.leftover_offset += ((data_length) as i32);
                return TC_CRYPTO_SUCCESS;
            }
            _copy(&s.leftover[(s.leftover_offset) as usize..], remaining_space, data, remaining_space);
            data_length = (data_length).wrapping_sub(remaining_space);
            data += remaining_space;
            s.leftover_offset = 0;
            i = ((0) as u32);
            while (i < ((TC_AES_BLOCK_SIZE) as u32)) {
                s.iv[(i) as usize] ^= s.leftover[(i) as usize];
                i = (i).wrapping_add(1);
            }
            tc_aes_encrypt(s.iv, s.iv, s.sched);
        }
        while (data_length > ((TC_AES_BLOCK_SIZE) as usize)) {
            i = ((0) as u32);
            while (i < ((TC_AES_BLOCK_SIZE) as u32)) {
                s.iv[(i) as usize] ^= ((data[(i) as usize]) as i32);
                i = (i).wrapping_add(1);
            }
            tc_aes_encrypt(s.iv, s.iv, s.sched);
            data += TC_AES_BLOCK_SIZE;
            data_length = (data_length).wrapping_sub(((TC_AES_BLOCK_SIZE) as usize));
        }
        if (data_length > ((0) as usize)) {
            _copy(s.leftover, data_length, data, data_length);
            s.leftover_offset = ((data_length) as i32);
        }
        return TC_CRYPTO_SUCCESS;
    }

    pub fn tc_cmac_final_safe(tag: &mut [u8], mut s: TCCmacState_t) -> i32 {
        let mut k: [u8; 4096] = [0; 4096];
        let mut i: u32 = ((0) as u32);
        if ((tag == 0) || (s == 0)) {
            return TC_CRYPTO_FAIL;
        }
        if (s.leftover_offset == TC_AES_BLOCK_SIZE) {
            k = s.K1;
        } else {
            let mut remaining: usize = (((TC_AES_BLOCK_SIZE - s.leftover_offset)) as usize);
            _set(&s.leftover[(s.leftover_offset) as usize..], 0, remaining);
            s.leftover[(s.leftover_offset) as usize] = TC_CMAC_PADDING;
            k = s.K2;
        }
        i = ((0) as u32);
        while (i < ((TC_AES_BLOCK_SIZE) as u32)) {
            s.iv[(i) as usize] ^= (s.leftover[(i) as usize] ^ ((k[(i) as usize]) as i32));
            i = (i).wrapping_add(1);
        }
        tc_aes_encrypt(tag, s.iv, s.sched);
        tc_cmac_erase_safe(s);
        return TC_CRYPTO_SUCCESS;
    }

}

/// # Safety
/// Pointer arguments must be valid for the duration of the call.
#[no_mangle]
pub unsafe extern "C" fn gf_double(out: *mut u8, in_buf: *mut u8) {
    let out_slice = unsafe { core::slice::from_raw_parts_mut(out, 3usize) };
    let in_buf_slice = unsafe { core::slice::from_raw_parts_mut(in_buf, 3usize) };
    safe_core::gf_double_safe(out_slice, in_buf_slice)
}

/// # Safety
/// Pointer arguments must be valid for the duration of the call.
#[no_mangle]
pub unsafe extern "C" fn tc_cmac_setup(s: TCCmacState_t, key: *const u8, sched: TCAesKeySched_t) -> i32 {
    let key_slice = unsafe { core::slice::from_raw_parts(key, 3usize) };
    safe_core::tc_cmac_setup_safe(s, key_slice, sched)
}

#[no_mangle]
pub extern "C" fn tc_cmac_erase(s: TCCmacState_t) -> i32 {
    safe_core::tc_cmac_erase_safe(s)
}

#[no_mangle]
pub extern "C" fn tc_cmac_init(s: TCCmacState_t) -> i32 {
    safe_core::tc_cmac_init_safe(s)
}

/// # Safety
/// Pointer arguments must be valid for the duration of the call.
#[no_mangle]
pub unsafe extern "C" fn tc_cmac_update(s: TCCmacState_t, data: *const u8, data_length: usize) -> i32 {
    let data_slice = unsafe { core::slice::from_raw_parts(data, 3usize) };
    safe_core::tc_cmac_update_safe(s, data_slice, data_length)
}

/// # Safety
/// Pointer arguments must be valid for the duration of the call.
#[no_mangle]
pub unsafe extern "C" fn tc_cmac_final(tag: *mut u8, s: TCCmacState_t) -> i32 {
    let tag_slice = unsafe { core::slice::from_raw_parts_mut(tag, 3usize) };
    safe_core::tc_cmac_final_safe(tag_slice, s)
}

