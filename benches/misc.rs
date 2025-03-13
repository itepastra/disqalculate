use criterion::{criterion_group, criterion_main, Criterion};
use rand::random;
use tsunami::Color;

pub fn benchmark_random_color(c: &mut Criterion) {
    let mut group = c.benchmark_group("Random Colors");
    group.bench_function("use impl trait", |b| {
        b.iter(|| {
            random::<Color>();
        })
    });
    group.bench_function("use 3 randoms", |b| {
        b.iter(|| Color::RGB24(random(), random(), random()));
    });

    group.finish()
}

criterion_group!(benches, benchmark_random_color);
criterion_main!(benches);
