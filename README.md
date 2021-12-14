[![crates.io](https://img.shields.io/crates/v/intersect2d.svg)](https://crates.io/crates/intersect2d)
[![Documentation](https://docs.rs/intersect2d/badge.svg)](https://docs.rs/intersect2d)
[![Workflow](https://github.com/eadf/intersect2d.rs/workflows/Rust/badge.svg)](https://github.com/eadf/intersect2d.rs/workflows/Rust/badge.svg)
[![Workflow](https://github.com/eadf/intersect2d.rs/workflows/Clippy/badge.svg)](https://github.com/eadf/intersect2d.rs/workflows/Clippy/badge.svg)
[![dependency status](https://deps.rs/crate/intersect2d/0.4.2/status.svg)](https://deps.rs/crate/intersect2d/0.4.2)
![license](https://img.shields.io/crates/l/intersect2d)

# intersect2d
After watching [Philipp Kindermann's](https://www.youtube.com/watch?v=I9EsN2DTnN8) excellent sweep-line 
videos I think I finally understand how this algorithm works.

This is my humble take on an implementation of the segment line 
intersection sweep-line algorithm.

The library crate also contains a [line intersection function](https://stackoverflow.com/a/565282).

This crate is educational, not ready for any production purpose. It has been adopted as a [cgmath](https://crates.io/crates/cgmath) library [here](https://crates.io/crates/linestring).

![Rusty voronoi](img.png)

Interactive step-by-step example:
```fish
cargo run --example fltk_gui --features console_trace
```

Intersection function API example:
```rust
use intersect2d::{intersect, Intersection};

let line1 = geo::Line::<f64>::from([(100.0,150.),(150.0,100.)]);
let line2 = geo::Line::<f64>::from([(100.0,150.),(150.0,100.)]);

let rv = intersect(&line1, &line2);
match rv {
    Some(Intersection::Intersection(_a)) => panic!("expected an overlap"),
    Some(Intersection::OverLap(a)) => println!("{:?}", a),
    None =>  panic!("expected an overlap"),
}
// you can also get a single intersection point from the Intersection enum.
// Albeit geometrically incorrect, it makes things easy
println!("{:?}", rv.unwrap().single());
```

Sweep-line API example:
```rust
let lines = vec![
    geo::Line::<f64>::from([(200.0,200.),(350.0,300.)]),
    geo::Line::<f64>::from([(400.0,200.),(250.0,300.)]),
];
let results = intersect2d::algorithm::AlgorithmData::<f64>::default()
    .with_ignore_end_point_intersections(false)?
    .with_lines(lines.into_iter())?
    .compute()?;
for (point, line) in results {
    println!("Intersection @{:?} Involved lines:{:?}", point, line);
}
```

Detection of self-intersecting geo::LineString:
```rust
let coordinates = vec![(200., 200.), (300., 300.), (400., 200.), (200., 300.)];
let line_string = geo::LineString::<f32>::from(coordinates);

// Obviously this example only makes sense for LinesStrings with many points.
// A simple brute force O(n²) intersection test will be faster than this O(nlog(n)+k)
// sweep-line algorithm if n is small enough.
let result = intersect2d::algorithm::AlgorithmData::<f32>::default()
    .with_ignore_end_point_intersections(true)?
    .with_stop_at_first_intersection(true)?
    .with_lines(line_string.lines())?
    .compute()?;
for (p, l) in result {
    println!("Intersection detected @{:?} Involved lines:{:?}", p, l);
}
```
or using the `SelfIntersectingExclusive` trait:
```rust
// SelfIntersectingExclusive does not report endpoint intersections
use intersect2d::SelfIntersectingExclusive;

let coordinates = vec![(200., 200.), (300., 300.), (400., 200.), (200., 300.)];
let line_string = geo::LineString::from(coordinates);

if line_string.is_self_intersecting()? {
    println!("Intersection detected");
}

for intersections in line_string.self_intersections()? {
    println!("Intersection: {:?}", intersections);
}
```

You can also check a bunch of `geo::Line` for self intersections using the `SelfIntersectingInclusive` trait:
```rust
// SelfIntersectingInclusive reports endpoint intersections
use intersect2d::SelfIntersectingInclusive;
let lines = vec![
    geo::Line::<f64>::from([(200.0,200.),(350.0,300.)]),
    geo::Line::<f64>::from([(400.0,200.),(250.0,300.)]),
];
if lines.is_self_intersecting_inclusive()? {
    println!("Intersection detected");
}
for intersections in lines.self_intersections_inclusive()? {
    println!("Intersection: {:?}", intersections);
}
```

## Todo
- [ ] Benchmark and optimize
- [ ] Stable overlapping co-linear line detection

