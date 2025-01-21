use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use wasm_encoder::CodeSection;

fn encode_current() {
    let mut code = CodeSection::new();
    for f in wasm_encoder_performance::current::helpers() {
        code.raw(&f.into_raw_body());
    }
}

fn encode_alternative() {
    let mut code = CodeSection::new();
    for f in wasm_encoder_performance::alternative::helpers() {
        code.raw(&f.into_raw_body());
    }
}

fn bench_encoding(c: &mut Criterion) {
    let mut group = c.benchmark_group("Encoding");
    group.bench_with_input(BenchmarkId::new("Current", "helpers"), &(), |b, _| {
        b.iter(encode_current)
    });
    group.bench_with_input(BenchmarkId::new("Alternative", "helpers"), &(), |b, _| {
        b.iter(encode_alternative)
    });
    group.finish();
}

criterion_group!(benches, bench_encoding);
criterion_main!(benches);
