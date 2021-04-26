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

#[test]
fn self_intersection_3() {
    let mut coordinates = Vec::<geo::Coordinate<f32>>::with_capacity(100);
    let mut angle = 0.0_f32;
    let mut radius = 0.1_f32;
    for _i in 0..100 {
        coordinates.push(geo::Coordinate {
            x: angle.cos() * radius,
            y: angle.sin() * radius,
        });
        angle += 0.1;
        radius += 0.2;
    }
    let coordinates = geo::LineString::from(coordinates);
    assert!(!coordinates.is_self_intersecting().unwrap());
}

#[test]
fn self_intersection_4() {
    let mut coordinates = Vec::<geo::Coordinate<f32>>::with_capacity(20);
    let mut angle = 0.0_f32;
    let mut radius = 0.1_f32;
    for _i in 0..20 {
        coordinates.push(geo::Coordinate {
            x: angle.cos() * radius,
            y: angle.sin() * radius,
        });
        angle += 0.1;
        radius += 0.2;
    }
    let coordinates = geo::LineString::from(coordinates);
    assert!(!coordinates.is_self_intersecting().unwrap());
}

#[test]
fn self_intersection_5() {
    let mut coordinates = Vec::<geo::Coordinate<f32>>::with_capacity(100);
    let mut angle = 0.0_f32;
    let mut radius = 0.1_f32;
    for _i in 0..100 {
        coordinates.push(geo::Coordinate {
            x: angle.cos() * radius,
            y: angle.sin() * radius,
        });
        angle += 0.1;
        radius += 0.2;
    }
    let coordinates = geo::LineString::from(coordinates);
    let coordinates: Vec<geo::Line<f32>> = coordinates.lines().collect();
    println!("{:?}", coordinates.is_self_intersecting());
    assert!(!coordinates.is_self_intersecting().unwrap());
}

#[test]
fn self_intersection_6() {
    let mut coordinates = Vec::<geo::Coordinate<f32>>::with_capacity(20);
    let mut angle = 0.0_f32;
    let mut radius = 0.1_f32;
    for _i in 0..20 {
        coordinates.push(geo::Coordinate {
            x: angle.cos() * radius,
            y: angle.sin() * radius,
        });
        angle += 0.1;
        radius += 0.2;
    }
    let coordinates = geo::LineString::from(coordinates);
    let coordinates: Vec<geo::Line<f32>> = coordinates.lines().collect();
    println!("{:?}", coordinates.is_self_intersecting());
    assert!(!coordinates.is_self_intersecting().unwrap());
}

#[test]
fn self_intersection_7() {
    let coordinates = vec![(200., 200.), (300., 300.), (400., 200.), (200., 300.)];
    let line_string = geo::LineString::from(coordinates);

    if line_string.is_self_intersecting().unwrap(){
        println!("Intersection detected");
    }
    assert!(line_string.is_self_intersecting().unwrap());
}