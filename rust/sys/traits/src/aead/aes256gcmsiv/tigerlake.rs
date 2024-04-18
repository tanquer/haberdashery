// @generated
use haberdashery_sys::aead::aes256gcmsiv::tigerlake::*;

use crate::aead::BenchmarkContext;
use crate::aead::Aead;

impl Aead for Aes256GcmSiv {
    const KEY_LEN: usize = Self::KEY_LEN;
    const NONCE_LEN: usize = Self::NONCE_LEN;
    const TAG_LEN: usize = Self::TAG_LEN;

    const FN_INIT: unsafe extern "C" fn(*mut Self, *const u8, usize) -> i32 =
        haberdashery_aes256gcmsiv_tigerlake_init;
    const FN_ENCRYPT: unsafe extern "C" fn(
        *const Self,
        *const u8,
        usize,
        *const u8,
        usize,
        *const u8,
        usize,
        *mut u8,
        usize,
        *mut u8,
        usize,
    ) -> i32 = haberdashery_aes256gcmsiv_tigerlake_encrypt;
    const FN_DECRYPT: unsafe extern "C" fn(
        *const Self,
        *const u8,
        usize,
        *const u8,
        usize,
        *const u8,
        usize,
        *const u8,
        usize,
        *mut u8,
        usize,
    ) -> i32 = haberdashery_aes256gcmsiv_tigerlake_decrypt;
    const FN_IS_SUPPORTED: unsafe extern "C" fn() -> i32 =
        haberdashery_aes256gcmsiv_tigerlake_is_supported;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::aead::TestVectors;

    #[test]
    fn vectors() {
        let vectors =
            TestVectors::<Aes256GcmSiv>::new(crate::PROJECT_PATH.join("test_vectors/aes256gcmsiv.cozybuf"));
        match vectors.verify() {
            Ok(()) => {}
            Err(_) => {
                vectors.generate().unwrap();
                vectors.verify().unwrap();
            }
        }
    }
}

#[nano_bench::benchmark(
    library:haberdashery,
    primitive:aead,
    algorithm:aes256gcmsiv,
    profile:tigerlake,
)]
fn init(iters: u64, measure: &mut dyn nano_bench::Measure) {
    let Some(mut context) = BenchmarkContext::<Aes256GcmSiv>::new_init() else {
        measure.skip();
        return;
    };
    measure.start();
    for _ in 0..iters {
        context.init();
    }
    measure.stop();
}
#[nano_bench::benchmark(
    library:haberdashery,
    primitive:aead,
    algorithm:aes256gcmsiv,
    profile:tigerlake,
)]
fn aad(length: usize, iters: u64, measure: &mut dyn nano_bench::Measure) {
    let Some(mut context) = BenchmarkContext::<Aes256GcmSiv>::new_aad(length) else {
        measure.skip();
        return;
    };
    measure.start();
    for _ in 0..iters {
        context.encrypt();
    }
    measure.stop();
}
#[nano_bench::benchmark(
    library:haberdashery,
    primitive:aead,
    algorithm:aes256gcmsiv,
    profile:tigerlake,
)]
fn encrypt(length: usize, iters: u64, measure: &mut dyn nano_bench::Measure) {
    let Some(mut context) = BenchmarkContext::<Aes256GcmSiv>::new_crypt(length) else {
        measure.skip();
        return;
    };
    measure.start();
    for _ in 0..iters {
        context.encrypt();
    }
    measure.stop();
}
#[nano_bench::benchmark(
    library:haberdashery,
    primitive:aead,
    algorithm:aes256gcmsiv,
    profile:tigerlake,
)]
fn decrypt(length: usize, iters: u64, measure: &mut dyn nano_bench::Measure) {
    let Some(mut context) = BenchmarkContext::<Aes256GcmSiv>::new_crypt(length) else {
        measure.skip();
        return;
    };
    measure.start();
    for _ in 0..iters {
        context.decrypt();
    }
    measure.stop();
}
