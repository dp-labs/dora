use bytes::Bytes;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use dora_runtime::precompiles::{blake2f, ecrecover, identity, modexp, ripemd_160, sha2_256};

/// Benchmark for `ecrecover`
fn benchmark_ecrecover(c: &mut Criterion) {
    let calldata = Bytes::from(&[0u8; 128][..]); // Example calldata
    let gas_limit = 5000;
    let mut consumed_gas = 0;

    c.bench_function("ecrecover", |b| {
        b.iter(|| ecrecover(black_box(&calldata), gas_limit, &mut consumed_gas).unwrap())
    });
}

/// Benchmark for `identity`
fn benchmark_identity(c: &mut Criterion) {
    let calldata = Bytes::from(&[0u8; 128][..]); // Example calldata
    let gas_limit = 5000;
    let mut consumed_gas = 0;

    c.bench_function("identity", |b| {
        b.iter(|| identity(black_box(&calldata), gas_limit, &mut consumed_gas))
    });
}

/// Benchmark for `sha2_256`
fn benchmark_sha2_256(c: &mut Criterion) {
    let calldata = Bytes::from(&[0u8; 128][..]); // Example calldata
    let gas_limit = 5000;
    let mut consumed_gas = 0;

    c.bench_function("sha2_256", |b| {
        b.iter(|| sha2_256(black_box(&calldata), gas_limit, &mut consumed_gas))
    });
}

/// Benchmark for `ripemd_160`
fn benchmark_ripemd_160(c: &mut Criterion) {
    let calldata = Bytes::from(&[0u8; 128][..]); // Example calldata
    let gas_limit = 5000;
    let mut consumed_gas = 0;

    c.bench_function("ripemd_160", |b| {
        b.iter(|| ripemd_160(black_box(&calldata), gas_limit, &mut consumed_gas))
    });
}

/// Benchmark for `modexp`
fn benchmark_modexp(c: &mut Criterion) {
    let calldata = Bytes::from(&[0u8; 128][..]); // Example calldata
    let gas_limit = 5000;
    let mut consumed_gas = 0;

    c.bench_function("modexp", |b| {
        b.iter(|| modexp(black_box(&calldata), gas_limit, &mut consumed_gas))
    });
}

/// Benchmark for `blake2f`
fn benchmark_blake2f(c: &mut Criterion) {
    let calldata = Bytes::from(&[0u8; 128][..]); // Example calldata
    let gas_limit = 5000;
    let mut consumed_gas = 0;

    c.bench_function("blake2f", |b| {
        b.iter(|| blake2f(black_box(&calldata), gas_limit, &mut consumed_gas).unwrap())
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
