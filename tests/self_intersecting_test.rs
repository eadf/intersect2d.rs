use intersect2d::SelfIntersecting;

#[test]
fn self_intersection_1() {
    let line_string = geo::LineString::from(vec![
        (100., 100.),
        (200., 100.),
        (200., 200.),
        (100., 200.),
        (100., 100.),
    ]);
    println!("{:?}", line_string.is_self_intersecting(true));
    assert!(!line_string.is_self_intersecting(true).unwrap());
}

#[test]
fn self_intersection_2() {
    let line_string = geo::LineString::from(vec![
        (100., 100.),
        (200., 100.),
        (200., 200.),
        (150., 50.),
        (100., 200.),
        (100., 100.),
    ]);
    println!("{:?}", line_string.is_self_intersecting(true));
    assert!(line_string.is_self_intersecting(true).unwrap());
}
