// Import from `core` instead of from `std` since we are in no-std mode
use core::result::Result;

// Import CKB syscalls and structures
// https://nervosnetwork.github.io/ckb-std/riscv64imac-unknown-none-elf/doc/ckb_std/index.html
use ckb_std::{ckb_types::core::ScriptHashType, cstr_core::cstr, high_level};

use crate::error::Error;

pub fn main() -> Result<(), Error> {
    let arg1 = cstr!("Hello World");
    let arg2 = cstr!("你好");
    // $ ckb-cli util blake2b --binary-path target/riscv64imac-unknown-none-elf/debug/exec-callee
    // 0xd4b27e70df8fa6922a3675e68b9dc0a319769b5955e49bbeaaf1daef16ce9328

    let code_hash = [
        0xd4, 0xb2, 0x7e, 0x70, 0xdf, 0x8f, 0xa6, 0x92, 0x2a, 0x36, 0x75, 0xe6, 0x8b, 0x9d, 0xc0,
        0xa3, 0x19, 0x76, 0x9b, 0x59, 0x55, 0xe4, 0x9b, 0xbe, 0xaa, 0xf1, 0xda, 0xef, 0x16, 0xce,
        0x93, 0x28,
    ];
    let ret = high_level::exec_cell(
        &code_hash[..],
        ScriptHashType::Data1,
        0,
        0,
        &[arg1, arg2][..],
    )
    .unwrap();
    panic!("exec failed: {}", ret);
}
