use intersect2d::SelfIntersectingExclusive;

#[test]
fn self_intersection_1() {
    let line_string = geo::LineString::from(vec![
        (100., 100.),
        (200., 100.),
        (200., 200.),
        (100., 200.),
        (100., 100.),
    ]);
    assert!(!line_string.is_self_intersecting().unwrap());
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
    assert!(line_string.is_self_intersecting().unwrap());
}
