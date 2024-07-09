// Copyright (c) Meta Platforms, Inc. and affiliates.
//
// This source code is dual-licensed under either the MIT license found in the
// LICENSE-MIT file in the root directory of this source tree or the Apache
// License, Version 2.0 found in the LICENSE-APACHE file in the root directory
// of this source tree. You may select, at your option, one of the above-listed licenses.

mod c89;
mod descriptor;
mod openssl_evp;
mod rust_bench;
mod rust_lib;
mod rust_sys;
pub(crate) mod write_generated;

use std::path::Path;

pub use descriptor::Case;
pub use descriptor::Descriptor;
pub use descriptor::Descriptors;

sflags::define! {
    --descriptor_path: <String as AsRef<Path>> = "";
    --bindings_path: <String as AsRef<Path>> = "";
}

fn get_descriptors_from_flag(primitive: &str) -> Descriptors {
    let path = DESCRIPTOR_PATH.join(primitive);
    Descriptors::try_from(path.as_ref())
        .unwrap_or_else(|e| panic!("Couldn't parse descriptors in {path:?}: {e}"))
}

pub fn openssl_evp() {
    openssl_evp::bindings();
}

pub fn rust_sys() {
    rust_sys::bindings();
}

pub fn rust_lib() {
    rust_lib::bindings();
}

pub fn rust_bench() {
    rust_bench::bindings();
}

pub fn c89() {
    c89::bindings();
}
