//! Synthesized by DecodeIQ CodeTruth Engine (AST-SLS Synaptic Topology) — Target: `hmac`
//! Algorithmic `safe_core` enforces `#![forbid(unsafe_code)]`.
#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, unused_parens, unused_assignments, unused_imports, unused_mut, dead_code, clippy::too_many_arguments, clippy::many_single_char_names, clippy::excessive_precision, clippy::cast_possible_truncation, clippy::cast_sign_loss, clippy::cast_possible_wrap, clippy::cast_lossless, clippy::float_cmp, clippy::manual_range_contains, clippy::double_parens, clippy::needless_return, clippy::unreadable_literal, clippy::identity_op, clippy::erasing_op, clippy::assign_op_pattern, clippy::collapsible_if, clippy::collapsible_else_if, clippy::needless_bool, clippy::comparison_chain, clippy::let_and_return, clippy::redundant_else, clippy::eq_op, clippy::overly_complex_bool_expr, clippy::precedence, clippy::unnecessary_cast, clippy::nonminimal_bool, clippy::redundant_field_names, clippy::redundant_comparisons, clippy::impossible_comparisons, clippy::unnecessary_to_owned)]


pub mod safe_core {
    #![forbid(unsafe_code)]
    use super::*;

    pub fn rekey(key: &mut [u8], new_key: &[u8], key_size: u32) {
        let mut inner_pad: u8 = ((0x36) as u8);
        let mut outer_pad: u8 = ((0x5c) as u8);
        let mut i: u32 = ((0) as u32);
        i = ((0) as u32);
        while (i < key_size) {
            key[(i) as usize] = (((((inner_pad) as i32) ^ ((new_key[(i) as usize]) as i32))) as u8);
            key[((i).wrapping_add(((TC_SHA256_BLOCK_SIZE) as u32))) as usize] = (((((outer_pad) as i32) ^ ((new_key[(i) as usize]) as i32))) as u8);
            i = (i).wrapping_add(1);
        }
        while (i < ((TC_SHA256_BLOCK_SIZE) as u32)) {
            key[(i) as usize] = inner_pad;
            key[((i).wrapping_add(((TC_SHA256_BLOCK_SIZE) as u32))) as usize] = outer_pad;
            i = (i).wrapping_add(1);
        }
    }

    pub fn tc_hmac_set_key_safe(mut ctx: TCHmacState_t, key: &[u8], mut key_size: u32) -> i32 {
        if (((ctx == 0) || (key == 0)) || (key_size == ((0) as u32))) {
            return TC_CRYPTO_FAIL;
        }
        let mut dummy_key: [u8; 1] = [0; 1];
        let mut dummy_state: tc_hmac_state_ = tc_hmac_state_::default();
        if (key_size <= ((TC_SHA256_BLOCK_SIZE) as u32)) {
            tc_sha256_init(&mut dummy_state.hash_state);
            tc_sha256_update(&mut dummy_state.hash_state, dummy_key, key_size);
            tc_sha256_final(&dummy_state.key[(TC_SHA256_DIGEST_SIZE) as usize..], &mut dummy_state.hash_state);
            rekey(&mut { let _tmp = ctx.key; _tmp }, key, key_size);
        } else {
            tc_sha256_init(&mut ctx.hash_state);
            tc_sha256_update(&mut ctx.hash_state, key, key_size);
            tc_sha256_final(&ctx.key[(TC_SHA256_DIGEST_SIZE) as usize..], &mut ctx.hash_state);
            rekey(&mut { let _tmp = ctx.key; _tmp }, &ctx.key[(TC_SHA256_DIGEST_SIZE) as usize..], ((TC_SHA256_DIGEST_SIZE) as u32));
        }
        return TC_CRYPTO_SUCCESS;
    }

    pub fn tc_hmac_init_safe(mut ctx: TCHmacState_t) -> i32 {
        if (ctx == 0) {
            return TC_CRYPTO_FAIL;
        }
        tc_sha256_init(&mut ctx.hash_state);
        tc_sha256_update(&mut ctx.hash_state, ctx.key, TC_SHA256_BLOCK_SIZE);
        return TC_CRYPTO_SUCCESS;
    }

    pub fn tc_hmac_update_safe(mut ctx: TCHmacState_t, data: &mut (), data_length: u32) -> i32 {
        if (ctx == 0) {
            return TC_CRYPTO_FAIL;
        }
        tc_sha256_update(&mut ctx.hash_state, data, data_length);
        return TC_CRYPTO_SUCCESS;
    }

    pub fn tc_hmac_final_safe(tag: &mut [u8], taglen: u32, mut ctx: TCHmacState_t) -> i32 {
        if (((tag == 0) || (taglen != ((TC_SHA256_DIGEST_SIZE) as u32))) || (ctx == 0)) {
            return TC_CRYPTO_FAIL;
        }
        tc_sha256_final(tag, &mut ctx.hash_state);
        tc_sha256_init(&mut ctx.hash_state);
        tc_sha256_update(&mut ctx.hash_state, &ctx.key[(TC_SHA256_BLOCK_SIZE) as usize..], TC_SHA256_BLOCK_SIZE);
        tc_sha256_update(&mut ctx.hash_state, tag, TC_SHA256_DIGEST_SIZE);
        tc_sha256_final(tag, &mut ctx.hash_state);
        _set(ctx, 0, sizeof(*ctx));
        return TC_CRYPTO_SUCCESS;
    }

}

/// # Safety
/// Pointer arguments must be valid for the duration of the call.
#[no_mangle]
pub unsafe extern "C" fn tc_hmac_set_key(ctx: TCHmacState_t, key: *const u8, key_size: u32) -> i32 {
    let key_slice = unsafe { core::slice::from_raw_parts(key, 3usize) };
    safe_core::tc_hmac_set_key_safe(ctx, key_slice, key_size)
}

#[no_mangle]
pub extern "C" fn tc_hmac_init(ctx: TCHmacState_t) -> i32 {
    safe_core::tc_hmac_init_safe(ctx)
}

/// # Safety
/// Pointer arguments must be valid for the duration of the call.
#[no_mangle]
pub unsafe extern "C" fn tc_hmac_update(ctx: TCHmacState_t, data: *const (), data_length: u32) -> i32 {
    let data_ref = unsafe { &mut *data };
    safe_core::tc_hmac_update_safe(ctx, data_ref, data_length)
}

/// # Safety
/// Pointer arguments must be valid for the duration of the call.
#[no_mangle]
pub unsafe extern "C" fn tc_hmac_final(tag: *mut u8, taglen: u32, ctx: TCHmacState_t) -> i32 {
    let tag_slice = unsafe { core::slice::from_raw_parts_mut(tag, 3usize) };
    safe_core::tc_hmac_final_safe(tag_slice, taglen, ctx)
}

