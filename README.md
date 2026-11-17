# TinyCrypt Safe Rust (`tinycrypt-safe-rust`)
[![License: BSD 3-Clause](https://img.shields.io/badge/License-BSD_3--Clause-blue.svg)](LICENSE)
[![Safety: Forbid Unsafe](https://img.shields.io/badge/unsafe-forbid-success.svg)](https://github.com/coboliqcom/tinycrypt-safe-rust)
[![no_std](https://img.shields.io/badge/no__std-compatible-blueviolet.svg)](https://github.com/coboliqcom/tinycrypt-safe-rust)
[![NIST KAT: 100% Passed](https://img.shields.io/badge/NIST%20KAT-100%25%20Verified-brightgreen.svg)](tests/kat_tests.rs)

> **100% Safe Rust (`#![forbid(unsafe_code)]`) and `no_std` cryptographic library for constrained microcontrollers, IoT devices, and RTOS environments.**  
> Lifted from Intel's archived `intel/tinycrypt` library by the **DecodeIQ CodeTruth Engine**.

---

## Background

On March 5, 2024, Intel archived the official [intel/tinycrypt](https://github.com/intel/tinycrypt) repository, leaving constrained IoT devices, Zephyr OS deployments, and embedded systems without ongoing maintenance.

Intel's notice invited the community:
> *"Intel has ceased development and contributions... If you have an ongoing need to use this project... please create your own fork of this project."*

While we maintain the C code in **[`coboliqcom/tinycrypt-hardened-c`](https://github.com/coboliqcom/tinycrypt-hardened-c)**, embedded devices running C remain vulnerable to low-level undefined behavior, integer promotions, and stack memory bugs.

**`tinycrypt-safe-rust`** is the memory-safe successor:
- **`#![forbid(unsafe_code)]`**: Absolutely zero `unsafe` blocks. Guaranteed no buffer overflows, no use-after-free, and no undefined behavior.
- **`#![no_std]`**: Zero standard library dependency, zero heap allocations (`no alloc`), deterministic stack memory.
- **NIST KAT Verified**: Tested and validated against official NIST Known Answer Tests (FIPS 180-4, FIPS 197).
- **Target Platforms**: ARM Cortex-M, RISC-V, ESP32, Zephyr-Rust, bare metal embedded systems.

---

## Cryptographic Primitives

| Primitive | Standard | Safety Guarantee | Status |
|---|---|---|---|
| **SHA-256** | FIPS 180-4 | 100% Safe Rust (`no_std`) | Verified KAT |
| **AES-128 (ECB/CBC/CTR)** | FIPS 197 / SP 800-38A | 100% Safe Rust (`no_std`) | Verified KAT |
| **HMAC-SHA256** | RFC 2104 / FIPS 198-1 | 100% Safe Rust (`no_std`) | Verified |
| **AES-CMAC** | NIST SP 800-38B | 100% Safe Rust (`no_std`) | Verified |
| **Constant-Time Utils** | Side-Channel Resistant | 100% Safe Rust (`no_std`) | Verified |

---

## Quick Start

Add to your `Cargo.toml`:
```toml
[dependencies]
tinycrypt-safe-rust = { git = "https://github.com/coboliqcom/tinycrypt-safe-rust.git" }
```

### SHA-256 Example (`no_std`)
```rust
use tinycrypt_safe_rust::Sha256;

let hash = Sha256::digest(b"abc");
assert_eq!(hash[0], 0xba);
```

### AES-128 Encryption Example
```rust
use tinycrypt_safe_rust::aes::{self, AES_ctx};

let key = [0x2bu8; 16];
let mut ctx = AES_ctx::default();
aes::init_ctx_safe(&mut ctx, &key, 128);

let mut block = [0x01u8; 16];
aes::ecb_encrypt_safe(&ctx, &mut block);
```

---

## Verification & Testing

Run all unit tests and NIST Known Answer Tests:
```bash
cargo test
cargo clippy -- -D warnings
```

---

## Companion C Repository

If your embedded firmware requires pure C99 with backward-compatible ABIs:
👉 **[`coboliqcom/tinycrypt-hardened-c`](https://github.com/coboliqcom/tinycrypt-hardened-c)**:
Features automated remediation of C99 §6.5.7p4 signed integer promotion shift UB and CERT-C EXP33 uninitialized variable bugs.

---

## License

BSD 3-Clause License. See [`LICENSE`](LICENSE) for details.  
Synthesized by [CobolIQ / DecodeIQ CodeTruth Engine](https://coboliq.com).
