use criterion::{criterion_group, criterion_main, Criterion};
use geo::Coordinate;
use geo::LineString;
use intersect2d::SelfIntersecting;

#[cfg(test)]
fn bench_1(c: &mut Criterion) {
    let mut coordinates = Vec::<Coordinate<f32>>::with_capacity(10000);
    let mut angle = 0.0_f32;
    let mut radius = 0.1_f32;
    for _i in 0..10000 {
        coordinates.push(geo::Coordinate {
            x: angle.cos() * radius,
            y: angle.sin() * radius,
        });
        angle += 0.1;
        radius += 0.2;
    }
    let coordinates = LineString::from(coordinates);

    c.bench_function("bench1", |b| {
        b.iter(|| {
            assert!(!coordinates.is_self_intersecting(true).unwrap_or(false));
        })
    });
}

criterion_group!(benches1, bench_1);
criterion_main!(benches1);
