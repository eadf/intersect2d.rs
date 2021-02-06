

[![Crates.io](https://meritbadge.herokuapp.com/intersect2d)](https://crates.io/crates/intersect2d)
[![Documentation](https://docs.rs/intersect2d/badge.svg)](https://docs.rs/intersect2d)
[![Workflow](https://github.com/eadf/intersect2d.rs/workflows/Rust/badge.svg)](https://github.com/eadf/intersect2d.rs/workflows/Rust/badge.svg)
[![Workflow](https://github.com/eadf/intersect2d.rs/workflows/Clippy/badge.svg)](https://github.com/eadf/intersect2d.rs/workflows/Clippy/badge.svg)
[![dependency status](https://deps.rs/crate/intersect2d/0.1.0/status.svg)](https://deps.rs/crate/intersect2d/0.1.0)

# intersection2d.rs
2D line segment intersection detection function.\
Line-sweep algorithm implementation.

Code still in development, not ready for any purpose.

![Rusty voronoi](img.png)

Quick example:
```fish
cargo run --example fltk_gui --features console_trace
```

API example:
```rust
use intersection2d::intersect;

let line1 = geo::Line::<f64>::new(
    geo::Coordinate { x: 100.0, y: 150.0 },
    geo::Coordinate { x: 150.0, y: 100.0 },
);
let line2 = geo::Line::<f64>::new(
    geo::Coordinate { x: 100.0, y: 150.0 },
    geo::Coordinate { x: 150.0, y: 100.0 },
);
let _rv = intersect(&line1, &line2);
match _rv {
    Some(Intersection::Intersection(_a)) => panic!("expected an overlap"),
    Some(Intersection::OverLap(a)) => println!("{:?}", a),
    None =>  panic!("expected an overlap"),
}
// you can also get a single intersection point from the Intersection enum.
// Albeit geometrically incorrect, it makes things easy
if let Some(_rv) =_rv {
    println!("{:?}", _rv.single());
}
```

## Todo
- [ ] Error handling
- [ ] Benchmark and optimize


