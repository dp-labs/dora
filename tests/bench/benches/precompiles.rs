use criterion::{black_box, criterion_group, criterion_main, Criterion};
use dora_primitives::Bytes;
use dora_runtime::precompiles::{blake2f, ecrecover, identity, modexp, ripemd_160, sha2_256};

/// Benchmark for `ecrecover`
fn benchmark_ecrecover(c: &mut Criterion) {
    let calldata = Bytes::from(&[0u8; 128][..]);
    let gas_limit = 5000;

    c.bench_function("ecrecover", |b| {
        b.iter(|| ecrecover(black_box(&calldata), gas_limit).unwrap())
    });
}

/// Benchmark for `identity`
fn benchmark_identity(c: &mut Criterion) {
    let calldata = Bytes::from(&[0u8; 128][..]);
    let gas_limit = 5000;

    c.bench_function("identity", |b| {
        b.iter(|| identity(black_box(&calldata), gas_limit).unwrap())
    });
}

/// Benchmark for `sha2_256`
fn benchmark_sha2_256(c: &mut Criterion) {
    let calldata = Bytes::from(&[0u8; 128][..]);
    let gas_limit = 5000;

    c.bench_function("sha2_256", |b| {
        b.iter(|| sha2_256(black_box(&calldata), gas_limit).unwrap())
    });
}

/// Benchmark for `ripemd_160`
fn benchmark_ripemd_160(c: &mut Criterion) {
    let calldata = Bytes::from(&[0u8; 128][..]);
    let gas_limit = 5000;

    c.bench_function("ripemd_160", |b| {
        b.iter(|| ripemd_160(black_box(&calldata), gas_limit).unwrap())
    });
}

/// Benchmark for `modexp`
fn benchmark_modexp(c: &mut Criterion) {
    let calldata = Bytes::from(&[0u8; 128][..]);
    let gas_limit = 5000;

    c.bench_function("modexp", |b| {
        b.iter(|| modexp(black_box(&calldata), gas_limit).unwrap())
    });
}

/// Benchmark for `blake2f`
fn benchmark_blake2f(c: &mut Criterion) {
    let calldata = hex::decode("0000000c48c9bdf267e6096a3ba7ca8485ae67bb2bf894fe72f36e3cf1361d5f3af54fa5d182e6ad7f520e511f6c3e2b8c68059b6bbd41fbabd9831f79217e1319cde05b61626300000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000300000000000000000000000000000001").unwrap();
    let calldata = Bytes::from(calldata);
    let gas_limit = 5000;

    c.bench_function("blake2f", |b| {
        b.iter(|| blake2f(black_box(&calldata), gas_limit).unwrap())
    });
}

criterion_group!(
    benches,
    benchmark_ecrecover,
    benchmark_identity,
    benchmark_sha2_256,
    benchmark_ripemd_160,
    benchmark_modexp,
    benchmark_blake2f,
);
criterion_main!(benches);
