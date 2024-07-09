// Copyright (c) Meta Platforms, Inc. and affiliates.
//
// This source code is dual-licensed under either the MIT license found in the
// LICENSE-MIT file in the root directory of this source tree or the Apache
// License, Version 2.0 found in the LICENSE-APACHE file in the root directory
// of this source tree. You may select, at your option, one of the above-listed licenses.

use haberdashery_asm_gen::ffi::mac::Mac;
use haberdashery_asm_gen::ffi::reader::Reader;
use haberdashery_asm_gen::ffi::writer::Writer;
use haberdashery_asm_gen::is_supported::is_supported;
use haberdashery_asm_gen::sivmac::SivMacKey as Base;

#[repr(C)]
pub struct SivMac(Base<8>);
#[bindings_proc::mac(
    algorithm: siv_mac,
    prefix: haberdashery,
    profile: haswell,
    profile: broadwell,
    profile: skylake,
    profile: skylakex,
)]
impl Mac for SivMac {
    const KEY_LEN: usize = 32;
    const TAG_LEN: usize = 16;
    const STRUCT_SIZE: usize = 368;
    const STRUCT_ALIGNMENT: usize = 16;
    #[inline(always)]
    fn init(&mut self, key: &[u8]) -> bool {
        self.0.init(key)
    }
    #[inline(always)]
    fn sign(&self, message: Reader, tag: Writer) -> bool {
        self.0.sign(message, tag)
    }
    #[inline(always)]
    fn verify(&self, message: Reader, tag: Reader) -> bool {
        self.0.verify(message, tag)
    }
    #[inline(always)]
    fn is_supported() -> bool {
        is_supported()
    }
}
#[bindings_proc::mac(
    algorithm: siv_mac,
    prefix: haberdashery,
    profile: tigerlake,
)]
impl Mac for SivMac {
    const KEY_LEN: usize = 32;
    const TAG_LEN: usize = 16;
    const STRUCT_SIZE: usize = 384;
    const STRUCT_ALIGNMENT: usize = 16;
    #[inline(always)]
    fn init(&mut self, key: &[u8]) -> bool {
        self.0.init(key)
    }
    #[inline(always)]
    fn sign(&self, message: Reader, tag: Writer) -> bool {
        self.0.sign(message, tag)
    }
    #[inline(always)]
    fn verify(&self, message: Reader, tag: Reader) -> bool {
        self.0.verify(message, tag)
    }
    #[inline(always)]
    fn is_supported() -> bool {
        is_supported()
    }
}
