#[allow(unused_imports)]
use approx;
#[allow(unused_imports)]
use geo::algorithm::intersects::Intersects;

#[allow(unused_imports)]
use intersect2d::{intersect, scale_to_coordinate, to_lines, Intersection};

#[allow(unused_imports)]
use num_traits::{Float, ToPrimitive};

#[test]
fn readme_1() {
    use intersect2d::{intersect, Intersection};

    let line1 = geo::Line::<f64>::from([(100.0, 150.), (150.0, 100.)]);
    let line2 = geo::Line::<f64>::from([(100.0, 150.), (150.0, 100.)]);

    let rv = intersect(&line1, &line2);
    match rv {
        Some(Intersection::Intersection(_a)) => panic!("expected an overlap"),
        Some(Intersection::OverLap(a)) => println!("{:?}", a),
        None => panic!("expected an overlap"),
    }
    // you can also get a single intersection point from the Intersection enum.
    // Albeit geometrically incorrect, it makes things easy
    println!("{:?}", rv.unwrap().single());
}

#[test]
fn readme_2() -> Result<(), intersect2d::IntersectError> {
    let lines = vec![
        geo::Line::<f64>::from([(200.0, 200.), (350.0, 300.)]),
        geo::Line::<f64>::from([(400.0, 200.), (250.0, 300.)]),
    ];
    let results = intersect2d::algorithm::AlgorithmData::<f64>::default()
        .with_ignore_end_point_intersections(false)?
        .with_lines(lines.into_iter())?
        .compute()?;
    for (point, line) in results {
        println!("Intersection @{:?} Involved lines:{:?}", point, line);
    }
    Ok(())
}

#[test]
fn readme_3() -> Result<(), intersect2d::IntersectError> {
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

    Ok(())
}

#[test]
fn readme_4() -> Result<(), intersect2d::IntersectError> {
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

    assert!(line_string.self_intersections()?.len() > 0);
    Ok(())
}

#[test]
fn readme_5() -> Result<(), intersect2d::IntersectError> {
    // SelfIntersectingInclusive reports endpoint intersections
    use intersect2d::SelfIntersectingInclusive;
    let lines = vec![
        geo::Line::<f64>::from([(200.0, 200.), (350.0, 300.)]),
        geo::Line::<f64>::from([(400.0, 200.), (250.0, 300.)]),
    ];
    if lines.is_self_intersecting_inclusive()? {
        println!("Intersection detected");
    }
    for intersections in lines.self_intersections_inclusive()? {
        println!("Intersection: {:?}", intersections);
    }

    assert!(lines.self_intersections_inclusive()?.len() > 0);
    Ok(())
}
