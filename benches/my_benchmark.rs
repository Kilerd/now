use criterion::{black_box, criterion_group, criterion_main, Criterion};


use chrono::Utc;
use now::Now;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("utc now", |b| b.iter(|| {
        Utc::now();
    }));
    c.bench_function("utc end_of_today", |b| b.iter(|| {
        Utc.end_of_today();
    }));
    c.bench_function("utc end_of_today_inline", |b| b.iter(|| {
        Utc.end_of_today_inline();
    }));
}
criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
