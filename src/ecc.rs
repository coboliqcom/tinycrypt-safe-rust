//! Synthesized by DecodeIQ CodeTruth Engine (AST-SLS Synaptic Topology) — Target: `ecc`
//! Algorithmic `safe_core` enforces `#![forbid(unsafe_code)]`.
#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, unused_parens, unused_assignments, unused_imports, unused_mut, dead_code, clippy::too_many_arguments, clippy::many_single_char_names, clippy::excessive_precision, clippy::cast_possible_truncation, clippy::cast_sign_loss, clippy::cast_possible_wrap, clippy::cast_lossless, clippy::float_cmp, clippy::manual_range_contains, clippy::double_parens, clippy::needless_return, clippy::unreadable_literal, clippy::identity_op, clippy::erasing_op, clippy::assign_op_pattern, clippy::collapsible_if, clippy::collapsible_else_if, clippy::needless_bool, clippy::comparison_chain, clippy::let_and_return, clippy::redundant_else, clippy::eq_op, clippy::overly_complex_bool_expr, clippy::precedence, clippy::unnecessary_cast, clippy::nonminimal_bool, clippy::redundant_field_names, clippy::redundant_comparisons, clippy::impossible_comparisons, clippy::unnecessary_to_owned)]


pub mod safe_core {
    #![forbid(unsafe_code)]
    use super::*;

    pub fn uECC_set_rng_safe(mut rng_function: uECC_RNG_Function) {
        g_rng_function = rng_function;
    }

    pub fn uECC_get_rng_safe() -> uECC_RNG_Function {
        return g_rng_function;
    }

    pub fn uECC_curve_private_key_size_safe(curve: uECC_Curve) -> i32 {
        return BITS_TO_BYTES(curve.num_n_bits);
    }

    pub fn uECC_curve_public_key_size_safe(curve: uECC_Curve) -> i32 {
        return (2 * curve.num_bytes);
    }

    pub fn uECC_vli_clear_safe(vli: &mut uECC_word_t, num_words: wordcount_t) {
        let mut i: wordcount_t = wordcount_t::default();
        i = 0;
        while (i < num_words) {
            vli[(i) as usize] = 0;
            i += 1;
        }
    }

    pub fn uECC_vli_isZero_safe(vli: &uECC_word_t, num_words: wordcount_t) -> uECC_word_t {
        let mut bits: uECC_word_t = 0;
        let mut i: wordcount_t = wordcount_t::default();
        i = 0;
        while (i < num_words) {
            bits |= vli[(i) as usize];
            i += 1;
        }
        return ((bits == 0));
    }

    pub fn uECC_vli_testBit_safe(vli: &uECC_word_t, bit: bitcount_t) -> uECC_word_t {
        return ((vli[((bit >> (uECC_WORD_BITS_SHIFT))) as usize] & ((1 << (((bit & uECC_WORD_BITS_MASK)))))));
    }

    pub fn vli_numDigits(vli: &uECC_word_t, max_words: wordcount_t) -> wordcount_t {
        let mut i: wordcount_t = wordcount_t::default();
        i = (max_words - 1);
        while ((i >= 0) && (vli[(i) as usize] == 0)) {
            i -= 1;
        }
        return ((i + 1));
    }

    pub fn uECC_vli_numBits_safe(vli: &uECC_word_t, max_words: wordcount_t) -> bitcount_t {
        let mut i: uECC_word_t = uECC_word_t::default();
        let mut digit: uECC_word_t = uECC_word_t::default();
        let mut num_digits: wordcount_t = vli_numDigits(vli, max_words);
        if (num_digits == 0) {
            return 0;
        }
        digit = vli[((num_digits - 1)) as usize];
        i = 0;
        while (digit) != 0 {
            digit >>= 1;
            i += 1;
        }
        return (((((bitcount_t)((num_digits - 1)) << (uECC_WORD_BITS_SHIFT))) + i));
    }

    pub fn uECC_vli_set_safe(dest: &mut uECC_word_t, src: &uECC_word_t, num_words: wordcount_t) {
        let mut i: wordcount_t = wordcount_t::default();
        i = 0;
        while (i < num_words) {
            dest[(i) as usize] = src[(i) as usize];
            i += 1;
        }
    }

    pub fn uECC_vli_cmp_unsafe_safe(left: &uECC_word_t, right: &uECC_word_t, num_words: wordcount_t) -> cmpresult_t {
        let mut i: wordcount_t = wordcount_t::default();
        i = (num_words - 1);
        while (i >= 0) {
            if (left[(i) as usize] > right[(i) as usize]) {
                return 1;
            } else if (left[(i) as usize] < right[(i) as usize]) {
                return -1;
            }
            i -= 1;
        }
        return 0;
    }

    pub fn uECC_vli_equal_safe(left: &uECC_word_t, right: &uECC_word_t, num_words: wordcount_t) -> uECC_word_t {
        let mut diff: uECC_word_t = 0;
        let mut i: wordcount_t = wordcount_t::default();
        i = (num_words - 1);
        while (i >= 0) {
            diff |= ((left[(i) as usize] ^ right[(i) as usize]));
            i -= 1;
        }
        return !(((diff == 0)));
    }

    pub fn cond_set_safe(p_true: uECC_word_t, p_false: uECC_word_t, cond: u32) -> uECC_word_t {
        return (((p_true).wrapping_mul((cond))) | ((((p_false * i32::from((((cond) == 0)))))) as u32));
    }

    pub fn uECC_vli_sub_safe(result: &mut uECC_word_t, left: &uECC_word_t, right: &uECC_word_t, num_words: wordcount_t) -> uECC_word_t {
        let mut borrow: uECC_word_t = 0;
        let mut i: wordcount_t = wordcount_t::default();
        i = 0;
        while (i < num_words) {
            let mut diff: uECC_word_t = ((left[(i) as usize] - right[(i) as usize]) - borrow);
            let mut val: uECC_word_t = ((diff > left[(i) as usize]));
            borrow = cond_set_safe(val, borrow, u32::from(((diff != left[(i) as usize]))));
            result[(i) as usize] = diff;
            i += 1;
        }
        return borrow;
    }

    pub fn uECC_vli_add(result: &mut uECC_word_t, left: &uECC_word_t, right: &uECC_word_t, num_words: wordcount_t) -> uECC_word_t {
        let mut carry: uECC_word_t = 0;
        let mut i: wordcount_t = wordcount_t::default();
        i = 0;
        while (i < num_words) {
            let mut sum: uECC_word_t = ((left[(i) as usize] + right[(i) as usize]) + carry);
            let mut val: uECC_word_t = ((sum < left[(i) as usize]));
            carry = cond_set_safe(val, carry, u32::from(((sum != left[(i) as usize]))));
            result[(i) as usize] = sum;
            i += 1;
        }
        return carry;
    }

    pub fn uECC_vli_cmp_safe(left: &uECC_word_t, right: &uECC_word_t, num_words: wordcount_t) -> cmpresult_t {
        let mut tmp: [uECC_word_t; 1] = [uECC_word_t::default(); 1];
        let mut neg: uECC_word_t = ((uECC_vli_sub_safe(tmp, left, right, num_words)) != 0);
        let mut equal: uECC_word_t = uECC_vli_isZero_safe(tmp, num_words);
        return ((i32::from(((equal) == 0)) - (2 * neg)));
    }

    pub fn uECC_vli_rshift1(vli: &mut uECC_word_t, num_words: wordcount_t) {
        let mut end: uECC_word_t = vli;
        let mut carry: uECC_word_t = 0;
        vli += num_words;
        while (vli -= 1 > end) {
            let mut temp: uECC_word_t = *vli;
            *vli = (((temp >> (1))) | carry);
            carry = (temp << (((uECC_WORD_BITS - 1))));
        }
    }

    pub fn muladd(a: uECC_word_t, b: uECC_word_t, r0: &mut uECC_word_t, r1: &mut uECC_word_t, r2: &mut uECC_word_t) {
        let mut p: uECC_dword_t = (a * b);
        let mut r01: uECC_dword_t = ((((uECC_dword_t)(*r1) << (uECC_WORD_BITS))) | *r0);
        r01 += p;
        *r2 += ((r01 < p));
        *r1 = (r01 >> (uECC_WORD_BITS));
        *r0 = r01;
    }

    pub fn uECC_vli_mult(result: &mut uECC_word_t, left: &uECC_word_t, right: &uECC_word_t, num_words: wordcount_t) {
        let mut r0: uECC_word_t = 0;
        let mut r1: uECC_word_t = 0;
        let mut r2: uECC_word_t = 0;
        let mut i: wordcount_t = wordcount_t::default();
        let mut k: wordcount_t = wordcount_t::default();
        k = 0;
        while (k < num_words) {
            i = 0;
            while (i <= k) {
                muladd(left[(i) as usize], right[((k - i)) as usize], &mut r0, &mut r1, &mut r2);
                i += 1;
            }
            result[(k) as usize] = r0;
            r0 = r1;
            r1 = r2;
            r2 = 0;
            k += 1;
        }
        k = num_words;
        while (k < ((num_words * 2) - 1)) {
            i = (((k + 1)) - num_words);
            while (i < num_words) {
                muladd(left[(i) as usize], right[((k - i)) as usize], &mut r0, &mut r1, &mut r2);
                i += 1;
            }
            result[(k) as usize] = r0;
            r0 = r1;
            r1 = r2;
            r2 = 0;
            k += 1;
        }
        result[(((num_words * 2) - 1)) as usize] = r0;
    }

    pub fn uECC_vli_modAdd_safe(result: &mut uECC_word_t, left: &uECC_word_t, right: &uECC_word_t, mod: &uECC_word_t, num_words: wordcount_t) {
        let mut carry: uECC_word_t = uECC_vli_add(result, left, right, num_words);
        if (carry || (uECC_vli_cmp_unsafe_safe(mod, result, num_words) != 1)) {
            uECC_vli_sub_safe(result, result, mod, num_words);
        }
    }

    pub fn uECC_vli_modSub_safe(result: &mut uECC_word_t, left: &uECC_word_t, right: &uECC_word_t, mod: &uECC_word_t, num_words: wordcount_t) {
        let mut l_borrow: uECC_word_t = uECC_vli_sub_safe(result, left, right, num_words);
        if (l_borrow) != 0 {
            uECC_vli_add(result, result, mod, num_words);
        }
    }

    pub fn uECC_vli_mmod_safe(result: &mut uECC_word_t, product: &mut uECC_word_t, mod: &uECC_word_t, num_words: wordcount_t) {
        let mut mod_multiple: [uECC_word_t; 2] = [uECC_word_t::default(); 2];
        let mut tmp: [uECC_word_t; 2] = [uECC_word_t::default(); 2];
        let mut v[2]: [uECC_word_t; 4096] = [uECC_word_t::default(); 4096];
        let mut index: uECC_word_t = uECC_word_t::default();
        let mut shift: bitcount_t = ((((num_words * 2) * uECC_WORD_BITS)) - uECC_vli_numBits_safe(mod, num_words));
        let mut word_shift: wordcount_t = (shift / uECC_WORD_BITS);
        let mut bit_shift: wordcount_t = (shift % uECC_WORD_BITS);
        let mut carry: uECC_word_t = 0;
        uECC_vli_clear_safe(mod_multiple, word_shift);
        if (bit_shift > 0) {
            index = 0;
            while (index < num_words) {
                mod_multiple[((word_shift + index)) as usize] = (((mod[(index) as usize] << (bit_shift))) | carry);
                carry = (mod[(index) as usize] >> (((uECC_WORD_BITS - bit_shift))));
                index += 1;
            }
        } else {
            uECC_vli_set_safe((mod_multiple + word_shift), mod, num_words);
        }
        index = 1;
        while (shift >= 0) {
            let mut borrow: uECC_word_t = 0;
            let mut i: wordcount_t = wordcount_t::default();
            i = 0;
            while (i < (num_words * 2)) {
                let mut diff: uECC_word_t = ((v[(index) as usize][(i) as usize] - mod_multiple[(i) as usize]) - borrow);
                if (diff != v[(index) as usize][(i) as usize]) {
                    borrow = ((diff > v[(index) as usize][(i) as usize]));
                }
                v[((1 - index)) as usize][(i) as usize] = diff;
                i += 1;
            }
            index = ((((index ^ borrow))) == 0);
            uECC_vli_rshift1(mod_multiple, num_words);
            mod_multiple[((num_words - 1)) as usize] |= (mod_multiple[(num_words) as usize] << (((uECC_WORD_BITS - 1))));
            uECC_vli_rshift1((mod_multiple + num_words), num_words);
            shift -= 1;
        }
        uECC_vli_set_safe(result, v[(index) as usize], num_words);
    }

    pub fn uECC_vli_modMult_safe(result: &mut uECC_word_t, left: &uECC_word_t, right: &uECC_word_t, mod: &uECC_word_t, num_words: wordcount_t) {
        let mut product: [uECC_word_t; 2] = [uECC_word_t::default(); 2];
        uECC_vli_mult(product, left, right, num_words);
        uECC_vli_mmod_safe(result, product, mod, num_words);
    }

    pub fn uECC_vli_modMult_fast_safe(result: &mut uECC_word_t, left: &uECC_word_t, right: &uECC_word_t, curve: uECC_Curve) {
        let mut product: [uECC_word_t; 2] = [uECC_word_t::default(); 2];
        uECC_vli_mult(product, left, right, curve.num_words);
        curve->mmod_fast(result, product);
    }

    pub fn uECC_vli_modSquare_fast(result: &mut uECC_word_t, left: &uECC_word_t, curve: uECC_Curve) {
        uECC_vli_modMult_fast_safe(result, left, left, curve);
    }

    pub fn vli_modInv_update(uv: &mut uECC_word_t, mod: &uECC_word_t, num_words: wordcount_t) {
        let mut carry: uECC_word_t = 0;
        if !(((((((uv)[(0) as usize] & 1))) == 0))) {
            carry = uECC_vli_add(uv, uv, mod, num_words);
        }
        uECC_vli_rshift1(uv, num_words);
        if (carry) != 0 {
            uv[((num_words - 1)) as usize] |= HIGH_BIT_SET;
        }
    }

    pub fn uECC_vli_modInv_safe(result: &mut uECC_word_t, input: &uECC_word_t, mod: &uECC_word_t, num_words: wordcount_t) {
        let mut a: [uECC_word_t; 1] = [uECC_word_t::default(); 1];
        let mut b: [uECC_word_t; 1] = [uECC_word_t::default(); 1];
        let mut u: [uECC_word_t; 1] = [uECC_word_t::default(); 1];
        let mut v: [uECC_word_t; 1] = [uECC_word_t::default(); 1];
        let mut cmpResult: cmpresult_t = cmpresult_t::default();
        if (uECC_vli_isZero_safe(input, num_words)) != 0 {
            uECC_vli_clear_safe(result, num_words);
            return;
        }
        uECC_vli_set_safe(a, input, num_words);
        uECC_vli_set_safe(b, mod, num_words);
        uECC_vli_clear_safe(u, num_words);
        u[(0) as usize] = 1;
        uECC_vli_clear_safe(v, num_words);
        while ((cmpResult = uECC_vli_cmp_unsafe_safe(a, b, num_words)) != 0) {
            if (((((a)[(0) as usize] & 1))) == 0) {
                uECC_vli_rshift1(a, num_words);
                vli_modInv_update(u, mod, num_words);
            } else if (((((b)[(0) as usize] & 1))) == 0) {
                uECC_vli_rshift1(b, num_words);
                vli_modInv_update(v, mod, num_words);
            } else if (cmpResult > 0) {
                uECC_vli_sub_safe(a, a, b, num_words);
                uECC_vli_rshift1(a, num_words);
                if (uECC_vli_cmp_unsafe_safe(u, v, num_words) < 0) {
                    uECC_vli_add(u, u, mod, num_words);
                }
                uECC_vli_sub_safe(u, u, v, num_words);
                vli_modInv_update(u, mod, num_words);
            } else {
                uECC_vli_sub_safe(b, b, a, num_words);
                uECC_vli_rshift1(b, num_words);
                if (uECC_vli_cmp_unsafe_safe(v, u, num_words) < 0) {
                    uECC_vli_add(v, v, mod, num_words);
                }
                uECC_vli_sub_safe(v, v, u, num_words);
                vli_modInv_update(v, mod, num_words);
            }
        }
        uECC_vli_set_safe(result, u, num_words);
    }

    pub fn double_jacobian_default_safe(X1: &mut uECC_word_t, Y1: &mut uECC_word_t, Z1: &mut uECC_word_t, curve: uECC_Curve) {
        let mut t4: [uECC_word_t; 1] = [uECC_word_t::default(); 1];
        let mut t5: [uECC_word_t; 1] = [uECC_word_t::default(); 1];
        let mut num_words: wordcount_t = curve.num_words;
        if (uECC_vli_isZero_safe(Z1, num_words)) != 0 {
            return;
        }
        uECC_vli_modSquare_fast(t4, Y1, curve);
        uECC_vli_modMult_fast_safe(t5, X1, t4, curve);
        uECC_vli_modSquare_fast(t4, t4, curve);
        uECC_vli_modMult_fast_safe(Y1, Y1, Z1, curve);
        uECC_vli_modSquare_fast(Z1, Z1, curve);
        uECC_vli_modAdd_safe(X1, X1, Z1, curve.p, num_words);
        uECC_vli_modAdd_safe(Z1, Z1, Z1, curve.p, num_words);
        uECC_vli_modSub_safe(Z1, X1, Z1, curve.p, num_words);
        uECC_vli_modMult_fast_safe(X1, X1, Z1, curve);
        uECC_vli_modAdd_safe(Z1, X1, X1, curve.p, num_words);
        uECC_vli_modAdd_safe(X1, X1, Z1, curve.p, num_words);
        if (uECC_vli_testBit_safe(X1, 0)) != 0 {
            let mut l_carry: uECC_word_t = uECC_vli_add(X1, X1, curve.p, num_words);
            uECC_vli_rshift1(X1, num_words);
            X1[((num_words - 1)) as usize] |= (l_carry << (((uECC_WORD_BITS - 1))));
        } else {
            uECC_vli_rshift1(X1, num_words);
        }
        uECC_vli_modSquare_fast(Z1, X1, curve);
        uECC_vli_modSub_safe(Z1, Z1, t5, curve.p, num_words);
        uECC_vli_modSub_safe(Z1, Z1, t5, curve.p, num_words);
        uECC_vli_modSub_safe(t5, t5, Z1, curve.p, num_words);
        uECC_vli_modMult_fast_safe(X1, X1, t5, curve);
        uECC_vli_modSub_safe(t4, X1, t4, curve.p, num_words);
        uECC_vli_set_safe(X1, Z1, num_words);
        uECC_vli_set_safe(Z1, Y1, num_words);
        uECC_vli_set_safe(Y1, t4, num_words);
    }

    pub fn x_side_default_safe(result: &mut uECC_word_t, x: &uECC_word_t, curve: uECC_Curve) {
        let mut _3: [uECC_word_t; 1] = [3];
        let mut num_words: wordcount_t = curve.num_words;
        uECC_vli_modSquare_fast(result, x, curve);
        uECC_vli_modSub_safe(result, result, _3, curve.p, num_words);
        uECC_vli_modMult_fast_safe(result, result, x, curve);
        uECC_vli_modAdd_safe(result, result, curve.b, curve.p, num_words);
    }

    pub fn uECC_secp256r1_safe() -> uECC_Curve {
        return &mut curve_secp256r1;
    }

    pub fn vli_mmod_fast_secp256r1_safe(result: &mut u32, product: &mut [u32]) {
        let mut tmp: [u32; 1] = [0; 1];
        let mut carry: i32 = 0;
        uECC_vli_set_safe(result, product, NUM_ECC_WORDS);
        tmp[(0) as usize] = tmp[(1) as usize] = tmp[(2) as usize] = ((0) as u32);
        tmp[(3) as usize] = product[(11) as usize];
        tmp[(4) as usize] = product[(12) as usize];
        tmp[(5) as usize] = product[(13) as usize];
        tmp[(6) as usize] = product[(14) as usize];
        tmp[(7) as usize] = product[(15) as usize];
        carry = uECC_vli_add(tmp, tmp, tmp, NUM_ECC_WORDS);
        carry += uECC_vli_add(result, result, tmp, NUM_ECC_WORDS);
        tmp[(3) as usize] = product[(12) as usize];
        tmp[(4) as usize] = product[(13) as usize];
        tmp[(5) as usize] = product[(14) as usize];
        tmp[(6) as usize] = product[(15) as usize];
        tmp[(7) as usize] = ((0) as u32);
        carry += uECC_vli_add(tmp, tmp, tmp, NUM_ECC_WORDS);
        carry += uECC_vli_add(result, result, tmp, NUM_ECC_WORDS);
        tmp[(0) as usize] = product[(8) as usize];
        tmp[(1) as usize] = product[(9) as usize];
        tmp[(2) as usize] = product[(10) as usize];
        tmp[(3) as usize] = tmp[(4) as usize] = tmp[(5) as usize] = ((0) as u32);
        tmp[(6) as usize] = product[(14) as usize];
        tmp[(7) as usize] = product[(15) as usize];
        carry += uECC_vli_add(result, result, tmp, NUM_ECC_WORDS);
        tmp[(0) as usize] = product[(9) as usize];
        tmp[(1) as usize] = product[(10) as usize];
        tmp[(2) as usize] = product[(11) as usize];
        tmp[(3) as usize] = product[(13) as usize];
        tmp[(4) as usize] = product[(14) as usize];
        tmp[(5) as usize] = product[(15) as usize];
        tmp[(6) as usize] = product[(13) as usize];
        tmp[(7) as usize] = product[(8) as usize];
        carry += uECC_vli_add(result, result, tmp, NUM_ECC_WORDS);
        tmp[(0) as usize] = product[(11) as usize];
        tmp[(1) as usize] = product[(12) as usize];
        tmp[(2) as usize] = product[(13) as usize];
        tmp[(3) as usize] = tmp[(4) as usize] = tmp[(5) as usize] = ((0) as u32);
        tmp[(6) as usize] = product[(8) as usize];
        tmp[(7) as usize] = product[(10) as usize];
        carry -= uECC_vli_sub_safe(result, result, tmp, NUM_ECC_WORDS);
        tmp[(0) as usize] = product[(12) as usize];
        tmp[(1) as usize] = product[(13) as usize];
        tmp[(2) as usize] = product[(14) as usize];
        tmp[(3) as usize] = product[(15) as usize];
        tmp[(4) as usize] = tmp[(5) as usize] = ((0) as u32);
        tmp[(6) as usize] = product[(9) as usize];
        tmp[(7) as usize] = product[(11) as usize];
        carry -= uECC_vli_sub_safe(result, result, tmp, NUM_ECC_WORDS);
        tmp[(0) as usize] = product[(13) as usize];
        tmp[(1) as usize] = product[(14) as usize];
        tmp[(2) as usize] = product[(15) as usize];
        tmp[(3) as usize] = product[(8) as usize];
        tmp[(4) as usize] = product[(9) as usize];
        tmp[(5) as usize] = product[(10) as usize];
        tmp[(6) as usize] = ((0) as u32);
        tmp[(7) as usize] = product[(12) as usize];
        carry -= uECC_vli_sub_safe(result, result, tmp, NUM_ECC_WORDS);
        tmp[(0) as usize] = product[(14) as usize];
        tmp[(1) as usize] = product[(15) as usize];
        tmp[(2) as usize] = ((0) as u32);
        tmp[(3) as usize] = product[(9) as usize];
        tmp[(4) as usize] = product[(10) as usize];
        tmp[(5) as usize] = product[(11) as usize];
        tmp[(6) as usize] = ((0) as u32);
        tmp[(7) as usize] = product[(13) as usize];
        carry -= uECC_vli_sub_safe(result, result, tmp, NUM_ECC_WORDS);
        if (carry < 0) {
            loop {
                carry += uECC_vli_add(result, result, curve_secp256r1.p, NUM_ECC_WORDS);
                if !((carry < 0)) {
                    break;
                }
            }
        } else {
            while ((carry) != 0 || (uECC_vli_cmp_unsafe_safe(curve_secp256r1.p, result, NUM_ECC_WORDS) != 1)) {
                carry -= uECC_vli_sub_safe(result, result, curve_secp256r1.p, NUM_ECC_WORDS);
            }
        }
    }

    pub fn EccPoint_isZero_safe(point: &uECC_word_t, curve: uECC_Curve) -> uECC_word_t {
        return uECC_vli_isZero_safe(point, (curve.num_words * 2));
    }

    pub fn apply_z_safe(X1: &mut uECC_word_t, Y1: &mut uECC_word_t, const Z: &uECC_word_t, curve: uECC_Curve) {
        let mut t1: [uECC_word_t; 1] = [uECC_word_t::default(); 1];
        uECC_vli_modSquare_fast(t1, Z, curve);
        uECC_vli_modMult_fast_safe(X1, X1, t1, curve);
        uECC_vli_modMult_fast_safe(t1, t1, Z, curve);
        uECC_vli_modMult_fast_safe(Y1, Y1, t1, curve);
    }

    pub fn XYcZ_initial_double(X1: &mut uECC_word_t, Y1: &mut uECC_word_t, X2: &mut uECC_word_t, Y2: &mut uECC_word_t, const initial_Z: &uECC_word_t, curve: uECC_Curve) {
        let mut z: [uECC_word_t; 1] = [uECC_word_t::default(); 1];
        let mut num_words: wordcount_t = curve.num_words;
        if (initial_Z) != 0 {
            uECC_vli_set_safe(z, initial_Z, num_words);
        } else {
            uECC_vli_clear_safe(z, num_words);
            z[(0) as usize] = 1;
        }
        uECC_vli_set_safe(X2, X1, num_words);
        uECC_vli_set_safe(Y2, Y1, num_words);
        apply_z_safe(X1, Y1, z, curve);
        curve->double_jacobian(X1, Y1, z, curve);
        apply_z_safe(X2, Y2, z, curve);
    }

    pub fn XYcZ_add_safe(X1: &mut uECC_word_t, Y1: &mut uECC_word_t, X2: &mut uECC_word_t, Y2: &mut uECC_word_t, curve: uECC_Curve) {
        let mut t5: [uECC_word_t; 1] = [uECC_word_t::default(); 1];
        let mut num_words: wordcount_t = curve.num_words;
        uECC_vli_modSub_safe(t5, X2, X1, curve.p, num_words);
        uECC_vli_modSquare_fast(t5, t5, curve);
        uECC_vli_modMult_fast_safe(X1, X1, t5, curve);
        uECC_vli_modMult_fast_safe(X2, X2, t5, curve);
        uECC_vli_modSub_safe(Y2, Y2, Y1, curve.p, num_words);
        uECC_vli_modSquare_fast(t5, Y2, curve);
        uECC_vli_modSub_safe(t5, t5, X1, curve.p, num_words);
        uECC_vli_modSub_safe(t5, t5, X2, curve.p, num_words);
        uECC_vli_modSub_safe(X2, X2, X1, curve.p, num_words);
        uECC_vli_modMult_fast_safe(Y1, Y1, X2, curve);
        uECC_vli_modSub_safe(X2, X1, t5, curve.p, num_words);
        uECC_vli_modMult_fast_safe(Y2, Y2, X2, curve);
        uECC_vli_modSub_safe(Y2, Y2, Y1, curve.p, num_words);
        uECC_vli_set_safe(X2, t5, num_words);
    }

    pub fn XYcZ_addC(X1: &mut uECC_word_t, Y1: &mut uECC_word_t, X2: &mut uECC_word_t, Y2: &mut uECC_word_t, curve: uECC_Curve) {
        let mut t5: [uECC_word_t; 1] = [uECC_word_t::default(); 1];
        let mut t6: [uECC_word_t; 1] = [uECC_word_t::default(); 1];
        let mut t7: [uECC_word_t; 1] = [uECC_word_t::default(); 1];
        let mut num_words: wordcount_t = curve.num_words;
        uECC_vli_modSub_safe(t5, X2, X1, curve.p, num_words);
        uECC_vli_modSquare_fast(t5, t5, curve);
        uECC_vli_modMult_fast_safe(X1, X1, t5, curve);
        uECC_vli_modMult_fast_safe(X2, X2, t5, curve);
        uECC_vli_modAdd_safe(t5, Y2, Y1, curve.p, num_words);
        uECC_vli_modSub_safe(Y2, Y2, Y1, curve.p, num_words);
        uECC_vli_modSub_safe(t6, X2, X1, curve.p, num_words);
        uECC_vli_modMult_fast_safe(Y1, Y1, t6, curve);
        uECC_vli_modAdd_safe(t6, X1, X2, curve.p, num_words);
        uECC_vli_modSquare_fast(X2, Y2, curve);
        uECC_vli_modSub_safe(X2, X2, t6, curve.p, num_words);
        uECC_vli_modSub_safe(t7, X1, X2, curve.p, num_words);
        uECC_vli_modMult_fast_safe(Y2, Y2, t7, curve);
        uECC_vli_modSub_safe(Y2, Y2, Y1, curve.p, num_words);
        uECC_vli_modSquare_fast(t7, t5, curve);
        uECC_vli_modSub_safe(t7, t7, t6, curve.p, num_words);
        uECC_vli_modSub_safe(t6, t7, X1, curve.p, num_words);
        uECC_vli_modMult_fast_safe(t6, t6, t5, curve);
        uECC_vli_modSub_safe(Y1, t6, Y1, curve.p, num_words);
        uECC_vli_set_safe(X1, t7, num_words);
    }

    pub fn EccPoint_mult_safe(result: &mut uECC_word_t, point: &uECC_word_t, scalar: &uECC_word_t, initial_Z: &uECC_word_t, num_bits: bitcount_t, curve: uECC_Curve) {
        let mut Rx: [[uECC_word_t; 1]; 2] = [[uECC_word_t::default(); 1]; 2];
        let mut Ry: [[uECC_word_t; 1]; 2] = [[uECC_word_t::default(); 1]; 2];
        let mut z: [uECC_word_t; 1] = [uECC_word_t::default(); 1];
        let mut i: bitcount_t = bitcount_t::default();
        let mut nb: uECC_word_t = uECC_word_t::default();
        let mut num_words: wordcount_t = curve.num_words;
        uECC_vli_set_safe(Rx[(1) as usize], point, num_words);
        uECC_vli_set_safe(Ry[(1) as usize], (point + num_words), num_words);
        XYcZ_initial_double(Rx[(1) as usize], Ry[(1) as usize], Rx[(0) as usize], Ry[(0) as usize], initial_Z, curve);
        i = (num_bits - 2);
        while (i > 0) {
            nb = ((uECC_vli_testBit_safe(scalar, i)) == 0);
            XYcZ_addC(Rx[((1 - nb)) as usize], Ry[((1 - nb)) as usize], Rx[(nb) as usize], Ry[(nb) as usize], curve);
            XYcZ_add_safe(Rx[(nb) as usize], Ry[(nb) as usize], Rx[((1 - nb)) as usize], Ry[((1 - nb)) as usize], curve);
            i -= 1;
        }
        nb = ((uECC_vli_testBit_safe(scalar, 0)) == 0);
        XYcZ_addC(Rx[((1 - nb)) as usize], Ry[((1 - nb)) as usize], Rx[(nb) as usize], Ry[(nb) as usize], curve);
        uECC_vli_modSub_safe(z, Rx[(1) as usize], Rx[(0) as usize], curve.p, num_words);
        uECC_vli_modMult_fast_safe(z, z, Ry[((1 - nb)) as usize], curve);
        uECC_vli_modMult_fast_safe(z, z, point, curve);
        uECC_vli_modInv_safe(z, z, curve.p, num_words);
        uECC_vli_modMult_fast_safe(z, z, (point + num_words), curve);
        uECC_vli_modMult_fast_safe(z, z, Rx[((1 - nb)) as usize], curve);
        XYcZ_add_safe(Rx[(nb) as usize], Ry[(nb) as usize], Rx[((1 - nb)) as usize], Ry[((1 - nb)) as usize], curve);
        apply_z_safe(Rx[(0) as usize], Ry[(0) as usize], z, curve);
        uECC_vli_set_safe(result, Rx[(0) as usize], num_words);
        uECC_vli_set_safe((result + num_words), Ry[(0) as usize], num_words);
    }

    pub fn regularize_k_safe(const k: &uECC_word_t, k0: &mut uECC_word_t, k1: &mut uECC_word_t, curve: uECC_Curve) -> uECC_word_t {
        let mut num_n_words: wordcount_t = BITS_TO_WORDS(curve.num_n_bits);
        let mut num_n_bits: bitcount_t = curve.num_n_bits;
        let mut carry: uECC_word_t = (uECC_vli_add(k0, k, curve.n, num_n_words) || (((num_n_bits < (((num_n_words * uECC_WORD_SIZE) * 8))) && uECC_vli_testBit_safe(k0, num_n_bits))));
        uECC_vli_add(k1, k0, curve.n, num_n_words);
        return carry;
    }

    pub fn EccPoint_compute_public_key_safe(result: &mut uECC_word_t, private_key: &mut uECC_word_t, curve: uECC_Curve) -> uECC_word_t {
        let mut tmp1: [uECC_word_t; 1] = [uECC_word_t::default(); 1];
        let mut tmp2: [uECC_word_t; 1] = [uECC_word_t::default(); 1];
        let mut p2[2]: [uECC_word_t; 4096] = [uECC_word_t::default(); 4096];
        let mut carry: uECC_word_t = uECC_word_t::default();
        carry = regularize_k_safe(private_key, tmp1, tmp2, curve);
        EccPoint_mult_safe(result, curve.G, p2[usize::from(((carry) == 0))], 0, (curve.num_n_bits + 1), curve);
        if (EccPoint_isZero_safe(result, curve)) != 0 {
            return 0;
        }
        return 1;
    }

    pub fn uECC_vli_nativeToBytes_safe(bytes: &mut [u8], num_bytes: i32, native: &[u32]) {
        let mut i: wordcount_t = wordcount_t::default();
        i = 0;
        while (i < num_bytes) {
            let mut b: u32 = ((((num_bytes - 1) - i)) as u32);
            bytes[(i) as usize] = (((native[((b / ((uECC_WORD_SIZE) as u32))) as usize]).wrapping_shr((((((8) as u32)).wrapping_mul(((b % ((uECC_WORD_SIZE) as u32)))))) as u32)) as u8);
            i += 1;
        }
    }

    pub fn uECC_vli_bytesToNative_safe(native: &mut [u32], bytes: &[u8], num_bytes: i32) {
        let mut i: wordcount_t = wordcount_t::default();
        uECC_vli_clear_safe(native, (((num_bytes + ((uECC_WORD_SIZE - 1)))) / uECC_WORD_SIZE));
        i = 0;
        while (i < num_bytes) {
            let mut b: u32 = ((((num_bytes - 1) - i)) as u32);
            native[((b / ((uECC_WORD_SIZE) as u32))) as usize] |= (bytes[(i) as usize] << (((((8) as u32)).wrapping_mul(((b % ((uECC_WORD_SIZE) as u32)))))));
            i += 1;
        }
    }

    pub fn uECC_generate_random_int_safe(random: &mut uECC_word_t, top: &uECC_word_t, num_words: wordcount_t) -> i32 {
        let mut mask: uECC_word_t = ((uECC_word_t) - 1);
        let mut tries: uECC_word_t = uECC_word_t::default();
        let mut num_bits: bitcount_t = uECC_vli_numBits_safe(top, num_words);
        if ((g_rng_function) == 0) {
            return 0;
        }
        tries = 0;
        while (tries < uECC_RNG_MAX_TRIES) {
            if ((g_rng_function(random, (num_words * uECC_WORD_SIZE))) == 0) {
                return 0;
            }
            random[((num_words - 1)) as usize] &= (mask >> (((bitcount_t)((((num_words * uECC_WORD_SIZE) * 8) - num_bits)))));
            if (((uECC_vli_isZero_safe(random, num_words)) == 0) && (uECC_vli_cmp_safe(top, random, num_words) == 1)) {
                return 1;
            }
            tries += 1;
        }
        return 0;
    }

    pub fn uECC_valid_point_safe(point: &uECC_word_t, curve: uECC_Curve) -> i32 {
        let mut tmp1: [uECC_word_t; 1] = [uECC_word_t::default(); 1];
        let mut tmp2: [uECC_word_t; 1] = [uECC_word_t::default(); 1];
        let mut num_words: wordcount_t = curve.num_words;
        if (EccPoint_isZero_safe(point, curve)) != 0 {
            return -1;
        }
        if ((uECC_vli_cmp_unsafe_safe(curve.p, point, num_words) != 1) || (uECC_vli_cmp_unsafe_safe(curve.p, (point + num_words), num_words) != 1)) {
            return -2;
        }
        uECC_vli_modSquare_fast(tmp1, (point + num_words), curve);
        curve->x_side(tmp2, point, curve);
        if (uECC_vli_equal_safe(tmp1, tmp2, num_words) != 0) {
            return -3;
        }
        return 0;
    }

    pub fn uECC_valid_public_key_safe(public_key: &[u8], curve: uECC_Curve) -> i32 {
        let mut _public: [uECC_word_t; 2] = [uECC_word_t::default(); 2];
        uECC_vli_bytesToNative_safe(&mut _public, public_key, curve.num_bytes);
        uECC_vli_bytesToNative_safe(&mut { let _tmp = (_public + curve.num_words); _tmp }, &{ let _tmp = (public_key + curve.num_bytes); _tmp }, curve.num_bytes);
        if (uECC_vli_cmp_unsafe_safe(_public, curve.G, (NUM_ECC_WORDS * 2)) == 0) {
            return -4;
        }
        return uECC_valid_point_safe(_public, curve);
    }

    pub fn uECC_compute_public_key_safe(private_key: &[u8], public_key: &mut [u8], curve: uECC_Curve) -> i32 {
        let mut _private: [uECC_word_t; 1] = [uECC_word_t::default(); 1];
        let mut _public: [uECC_word_t; 2] = [uECC_word_t::default(); 2];
        uECC_vli_bytesToNative_safe(&mut _private, private_key, BITS_TO_BYTES(curve.num_n_bits));
        if (uECC_vli_isZero_safe(_private, BITS_TO_WORDS(curve.num_n_bits))) != 0 {
            return 0;
        }
        if (uECC_vli_cmp_safe(curve.n, _private, BITS_TO_WORDS(curve.num_n_bits)) != 1) {
            return 0;
        }
        if ((EccPoint_compute_public_key_safe(_public, _private, curve)) == 0) {
            return 0;
        }
        uECC_vli_nativeToBytes_safe(public_key, curve.num_bytes, &_public);
        uECC_vli_nativeToBytes_safe(&mut { let _tmp = (public_key + curve.num_bytes); _tmp }, curve.num_bytes, &{ let _tmp = (_public + curve.num_words); _tmp });
        return 1;
    }

}

#[no_mangle]
pub extern "C" fn uECC_set_rng(rng_function: uECC_RNG_Function) {
    safe_core::uECC_set_rng_safe(rng_function)
}

#[no_mangle]
pub extern "C" fn uECC_get_rng() -> uECC_RNG_Function {
    safe_core::uECC_get_rng_safe()
}

#[no_mangle]
pub extern "C" fn uECC_curve_private_key_size(curve: uECC_Curve) -> i32 {
    safe_core::uECC_curve_private_key_size_safe(curve)
}

#[no_mangle]
pub extern "C" fn uECC_curve_public_key_size(curve: uECC_Curve) -> i32 {
    safe_core::uECC_curve_public_key_size_safe(curve)
}

/// # Safety
/// Pointer arguments must be valid for the duration of the call.
#[no_mangle]
pub unsafe extern "C" fn uECC_vli_clear(vli: *mut uECC_word_t, num_words: wordcount_t) {
    let vli_ref = unsafe { &mut *vli };
    safe_core::uECC_vli_clear_safe(vli_ref, num_words)
}

/// # Safety
/// Pointer arguments must be valid for the duration of the call.
#[no_mangle]
pub unsafe extern "C" fn uECC_vli_isZero(vli: *const uECC_word_t, num_words: wordcount_t) -> uECC_word_t {
    let vli_ref = unsafe { &*vli };
    safe_core::uECC_vli_isZero_safe(vli_ref, num_words)
}

/// # Safety
/// Pointer arguments must be valid for the duration of the call.
#[no_mangle]
pub unsafe extern "C" fn uECC_vli_testBit(vli: *const uECC_word_t, bit: bitcount_t) -> uECC_word_t {
    let vli_ref = unsafe { &*vli };
    safe_core::uECC_vli_testBit_safe(vli_ref, bit)
}

/// # Safety
/// Pointer arguments must be valid for the duration of the call.
#[no_mangle]
pub unsafe extern "C" fn uECC_vli_numBits(vli: *const uECC_word_t, max_words: wordcount_t) -> bitcount_t {
    let vli_ref = unsafe { &*vli };
    safe_core::uECC_vli_numBits_safe(vli_ref, max_words)
}

/// # Safety
/// Pointer arguments must be valid for the duration of the call.
#[no_mangle]
pub unsafe extern "C" fn uECC_vli_set(dest: *mut uECC_word_t, src: *const uECC_word_t, num_words: wordcount_t) {
    let dest_ref = unsafe { &mut *dest };
    let src_ref = unsafe { &*src };
    safe_core::uECC_vli_set_safe(dest_ref, src_ref, num_words)
}

/// # Safety
/// Pointer arguments must be valid for the duration of the call.
#[no_mangle]
pub unsafe extern "C" fn uECC_vli_cmp_unsafe(left: *const uECC_word_t, right: *const uECC_word_t, num_words: wordcount_t) -> cmpresult_t {
    let left_ref = unsafe { &*left };
    let right_ref = unsafe { &*right };
    safe_core::uECC_vli_cmp_unsafe_safe(left_ref, right_ref, num_words)
}

/// # Safety
/// Pointer arguments must be valid for the duration of the call.
#[no_mangle]
pub unsafe extern "C" fn uECC_vli_equal(left: *const uECC_word_t, right: *const uECC_word_t, num_words: wordcount_t) -> uECC_word_t {
    let left_ref = unsafe { &*left };
    let right_ref = unsafe { &*right };
    safe_core::uECC_vli_equal_safe(left_ref, right_ref, num_words)
}

#[no_mangle]
pub extern "C" fn cond_set(p_true: uECC_word_t, p_false: uECC_word_t, cond: u32) -> uECC_word_t {
    safe_core::cond_set_safe(p_true, p_false, cond)
}

/// # Safety
/// Pointer arguments must be valid for the duration of the call.
#[no_mangle]
pub unsafe extern "C" fn uECC_vli_sub(result: *mut uECC_word_t, left: *const uECC_word_t, right: *const uECC_word_t, num_words: wordcount_t) -> uECC_word_t {
    let result_ref = unsafe { &mut *result };
    let left_ref = unsafe { &*left };
    let right_ref = unsafe { &*right };
    safe_core::uECC_vli_sub_safe(result_ref, left_ref, right_ref, num_words)
}

/// # Safety
/// Pointer arguments must be valid for the duration of the call.
#[no_mangle]
pub unsafe extern "C" fn uECC_vli_cmp(left: *const uECC_word_t, right: *const uECC_word_t, num_words: wordcount_t) -> cmpresult_t {
    let left_ref = unsafe { &*left };
    let right_ref = unsafe { &*right };
    safe_core::uECC_vli_cmp_safe(left_ref, right_ref, num_words)
}

/// # Safety
/// Pointer arguments must be valid for the duration of the call.
#[no_mangle]
pub unsafe extern "C" fn uECC_vli_modAdd(result: *mut uECC_word_t, left: *const uECC_word_t, right: *const uECC_word_t, mod: *const uECC_word_t, num_words: wordcount_t) {
    let result_ref = unsafe { &mut *result };
    let left_ref = unsafe { &*left };
    let right_ref = unsafe { &*right };
    let mod_ref = unsafe { &*mod };
    safe_core::uECC_vli_modAdd_safe(result_ref, left_ref, right_ref, mod_ref, num_words)
}

/// # Safety
/// Pointer arguments must be valid for the duration of the call.
#[no_mangle]
pub unsafe extern "C" fn uECC_vli_modSub(result: *mut uECC_word_t, left: *const uECC_word_t, right: *const uECC_word_t, mod: *const uECC_word_t, num_words: wordcount_t) {
    let result_ref = unsafe { &mut *result };
    let left_ref = unsafe { &*left };
    let right_ref = unsafe { &*right };
    let mod_ref = unsafe { &*mod };
    safe_core::uECC_vli_modSub_safe(result_ref, left_ref, right_ref, mod_ref, num_words)
}

/// # Safety
/// Pointer arguments must be valid for the duration of the call.
#[no_mangle]
pub unsafe extern "C" fn uECC_vli_mmod(result: *mut uECC_word_t, product: *mut uECC_word_t, mod: *const uECC_word_t, num_words: wordcount_t) {
    let result_ref = unsafe { &mut *result };
    let product_ref = unsafe { &mut *product };
    let mod_ref = unsafe { &*mod };
    safe_core::uECC_vli_mmod_safe(result_ref, product_ref, mod_ref, num_words)
}

/// # Safety
/// Pointer arguments must be valid for the duration of the call.
#[no_mangle]
pub unsafe extern "C" fn uECC_vli_modMult(result: *mut uECC_word_t, left: *const uECC_word_t, right: *const uECC_word_t, mod: *const uECC_word_t, num_words: wordcount_t) {
    let result_ref = unsafe { &mut *result };
    let left_ref = unsafe { &*left };
    let right_ref = unsafe { &*right };
    let mod_ref = unsafe { &*mod };
    safe_core::uECC_vli_modMult_safe(result_ref, left_ref, right_ref, mod_ref, num_words)
}

/// # Safety
/// Pointer arguments must be valid for the duration of the call.
#[no_mangle]
pub unsafe extern "C" fn uECC_vli_modMult_fast(result: *mut uECC_word_t, left: *const uECC_word_t, right: *const uECC_word_t, curve: uECC_Curve) {
    let result_ref = unsafe { &mut *result };
    let left_ref = unsafe { &*left };
    let right_ref = unsafe { &*right };
    safe_core::uECC_vli_modMult_fast_safe(result_ref, left_ref, right_ref, curve)
}

/// # Safety
/// Pointer arguments must be valid for the duration of the call.
#[no_mangle]
pub unsafe extern "C" fn uECC_vli_modInv(result: *mut uECC_word_t, input: *const uECC_word_t, mod: *const uECC_word_t, num_words: wordcount_t) {
    let result_ref = unsafe { &mut *result };
    let input_ref = unsafe { &*input };
    let mod_ref = unsafe { &*mod };
    safe_core::uECC_vli_modInv_safe(result_ref, input_ref, mod_ref, num_words)
}

/// # Safety
/// Pointer arguments must be valid for the duration of the call.
#[no_mangle]
pub unsafe extern "C" fn double_jacobian_default(X1: *mut uECC_word_t, Y1: *mut uECC_word_t, Z1: *mut uECC_word_t, curve: uECC_Curve) {
    let X1_ref = unsafe { &mut *X1 };
    let Y1_ref = unsafe { &mut *Y1 };
    let Z1_ref = unsafe { &mut *Z1 };
    safe_core::double_jacobian_default_safe(X1_ref, Y1_ref, Z1_ref, curve)
}

/// # Safety
/// Pointer arguments must be valid for the duration of the call.
#[no_mangle]
pub unsafe extern "C" fn x_side_default(result: *mut uECC_word_t, x: *const uECC_word_t, curve: uECC_Curve) {
    let result_ref = unsafe { &mut *result };
    let x_ref = unsafe { &*x };
    safe_core::x_side_default_safe(result_ref, x_ref, curve)
}

#[no_mangle]
pub extern "C" fn uECC_secp256r1() -> uECC_Curve {
    safe_core::uECC_secp256r1_safe()
}

/// # Safety
/// Pointer arguments must be valid for the duration of the call.
#[no_mangle]
pub unsafe extern "C" fn vli_mmod_fast_secp256r1(result: *mut u32, product: *mut u32) {
    let result_ref = unsafe { &mut *result };
    let product_slice = unsafe { core::slice::from_raw_parts_mut(product, 3usize) };
    safe_core::vli_mmod_fast_secp256r1_safe(result_ref, product_slice)
}

/// # Safety
/// Pointer arguments must be valid for the duration of the call.
#[no_mangle]
pub unsafe extern "C" fn EccPoint_isZero(point: *const uECC_word_t, curve: uECC_Curve) -> uECC_word_t {
    let point_ref = unsafe { &*point };
    safe_core::EccPoint_isZero_safe(point_ref, curve)
}

/// # Safety
/// Pointer arguments must be valid for the duration of the call.
#[no_mangle]
pub unsafe extern "C" fn apply_z(X1: *mut uECC_word_t, Y1: *mut uECC_word_t, const Z: *const uECC_word_t, curve: uECC_Curve) {
    let X1_ref = unsafe { &mut *X1 };
    let Y1_ref = unsafe { &mut *Y1 };
    let const Z_ref = unsafe { &*const Z };
    safe_core::apply_z_safe(X1_ref, Y1_ref, const Z_ref, curve)
}

/// # Safety
/// Pointer arguments must be valid for the duration of the call.
#[no_mangle]
pub unsafe extern "C" fn XYcZ_add(X1: *mut uECC_word_t, Y1: *mut uECC_word_t, X2: *mut uECC_word_t, Y2: *mut uECC_word_t, curve: uECC_Curve) {
    let X1_ref = unsafe { &mut *X1 };
    let Y1_ref = unsafe { &mut *Y1 };
    let X2_ref = unsafe { &mut *X2 };
    let Y2_ref = unsafe { &mut *Y2 };
    safe_core::XYcZ_add_safe(X1_ref, Y1_ref, X2_ref, Y2_ref, curve)
}

/// # Safety
/// Pointer arguments must be valid for the duration of the call.
#[no_mangle]
pub unsafe extern "C" fn EccPoint_mult(result: *mut uECC_word_t, point: *const uECC_word_t, scalar: *const uECC_word_t, initial_Z: *const uECC_word_t, num_bits: bitcount_t, curve: uECC_Curve) {
    let result_ref = unsafe { &mut *result };
    let point_ref = unsafe { &*point };
    let scalar_ref = unsafe { &*scalar };
    let initial_Z_ref = unsafe { &*initial_Z };
    safe_core::EccPoint_mult_safe(result_ref, point_ref, scalar_ref, initial_Z_ref, num_bits, curve)
}

/// # Safety
/// Pointer arguments must be valid for the duration of the call.
#[no_mangle]
pub unsafe extern "C" fn regularize_k(const k: *const uECC_word_t, k0: *mut uECC_word_t, k1: *mut uECC_word_t, curve: uECC_Curve) -> uECC_word_t {
    let const k_ref = unsafe { &*const k };
    let k0_ref = unsafe { &mut *k0 };
    let k1_ref = unsafe { &mut *k1 };
    safe_core::regularize_k_safe(const k_ref, k0_ref, k1_ref, curve)
}

/// # Safety
/// Pointer arguments must be valid for the duration of the call.
#[no_mangle]
pub unsafe extern "C" fn EccPoint_compute_public_key(result: *mut uECC_word_t, private_key: *mut uECC_word_t, curve: uECC_Curve) -> uECC_word_t {
    let result_ref = unsafe { &mut *result };
    let private_key_ref = unsafe { &mut *private_key };
    safe_core::EccPoint_compute_public_key_safe(result_ref, private_key_ref, curve)
}

/// # Safety
/// Pointer arguments must be valid for the duration of the call.
#[no_mangle]
pub unsafe extern "C" fn uECC_vli_nativeToBytes(bytes: *mut u8, num_bytes: i32, native: *const u32) {
    let bytes_slice = unsafe { core::slice::from_raw_parts_mut(bytes, 3usize) };
    let native_slice = unsafe { core::slice::from_raw_parts(native, 3usize) };
    safe_core::uECC_vli_nativeToBytes_safe(bytes_slice, num_bytes, native_slice)
}

/// # Safety
/// Pointer arguments must be valid for the duration of the call.
#[no_mangle]
pub unsafe extern "C" fn uECC_vli_bytesToNative(native: *mut u32, bytes: *const u8, num_bytes: i32) {
    let native_slice = unsafe { core::slice::from_raw_parts_mut(native, 3usize) };
    let bytes_slice = unsafe { core::slice::from_raw_parts(bytes, 3usize) };
    safe_core::uECC_vli_bytesToNative_safe(native_slice, bytes_slice, num_bytes)
}

/// # Safety
/// Pointer arguments must be valid for the duration of the call.
#[no_mangle]
pub unsafe extern "C" fn uECC_generate_random_int(random: *mut uECC_word_t, top: *const uECC_word_t, num_words: wordcount_t) -> i32 {
    let random_ref = unsafe { &mut *random };
    let top_ref = unsafe { &*top };
    safe_core::uECC_generate_random_int_safe(random_ref, top_ref, num_words)
}

/// # Safety
/// Pointer arguments must be valid for the duration of the call.
#[no_mangle]
pub unsafe extern "C" fn uECC_valid_point(point: *const uECC_word_t, curve: uECC_Curve) -> i32 {
    let point_ref = unsafe { &*point };
    safe_core::uECC_valid_point_safe(point_ref, curve)
}

/// # Safety
/// Pointer arguments must be valid for the duration of the call.
#[no_mangle]
pub unsafe extern "C" fn uECC_valid_public_key(public_key: *const u8, curve: uECC_Curve) -> i32 {
    let public_key_slice = unsafe { core::slice::from_raw_parts(public_key, 3usize) };
    safe_core::uECC_valid_public_key_safe(public_key_slice, curve)
}

/// # Safety
/// Pointer arguments must be valid for the duration of the call.
#[no_mangle]
pub unsafe extern "C" fn uECC_compute_public_key(private_key: *const u8, public_key: *mut u8, curve: uECC_Curve) -> i32 {
    let private_key_slice = unsafe { core::slice::from_raw_parts(private_key, 3usize) };
    let public_key_slice = unsafe { core::slice::from_raw_parts_mut(public_key, 3usize) };
    safe_core::uECC_compute_public_key_safe(private_key_slice, public_key_slice, curve)
}

