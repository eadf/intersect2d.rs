use approx;
use geo::algorithm::intersects::Intersects;

#[allow(unused_imports)]
use intersect2d::{intersect, scale_to_coordinate, to_lines, ulps_eq, Intersection};
//use intersections_2d::algorithm::{AlgorithmData, SiteEventKey};
#[allow(unused_imports)]
use num_traits::{Float, ToPrimitive};

#[allow(dead_code)]
/// Test verification
pub fn brute_force<T>(lines: &[geo::Line<T>]) -> Vec<(usize, Vec<usize>)>
where
    T: Float + approx::UlpsEq + geo::GeoNum + PartialOrd,
    T::Epsilon: Copy,
{
    let mut rv: Vec<(usize, Vec<usize>)> = Vec::new();
    for i in 0..lines.len() {
        // should be i+1, but that messes up the count
        let mut line = (i, Vec::<usize>::new());
        for j in 0..lines.len() {
            if i == j {
                continue;
            }
            let li = &lines[i];
            let lj = &lines[j];
            let int1 = li.intersects(lj);
            let int2 = intersect(li, lj).is_some();
            assert_eq!(int1, int2);
            if int1 {
                line.1.push(j);
                //rv += 1;
            }
        }
        if !line.1.is_empty() {
            rv.push(line);
        }
    }
    rv
}

#[allow(dead_code)]
fn almost_equal<T>(x1: T, x2: T, y1: T, y2: T)
where
    T: Float + approx::UlpsEq + geo::CoordNum + PartialOrd,
    T::Epsilon: Copy,
{
    assert!(ulps_eq(&x1, &x2));
    assert!(ulps_eq(&y1, &y2));
}

/// draws a line pivoting around (x,y) with 'angle' in degrees
/// l1 & l2 are lengths
#[allow(dead_code)]
fn pivot(x: f64, y: f64, l1: f64, l2: f64, angle: f64) -> geo::Line<f64> {
    let l = geo::Line {
        start: geo::Coordinate {
            x: x + angle.to_radians().cos() * l1,
            y: y + angle.to_radians().sin() * l1,
        },
        end: geo::Coordinate {
            x: x + (angle + 180.0).to_radians().cos() * l2,
            y: y + (angle + 180.0).to_radians().sin() * l2,
        },
    };
    // nudge the result so that it really pivots at the (x,y) point
    let v = l.end - l.start;
    let v1 = -v * (l1 / (l1 + l2));
    let v2 = v * (l2 / (l1 + l2));

    geo::Line {
        start: geo::Coordinate {
            x: x + v1.x,
            y: y + v1.y,
        },
        end: geo::Coordinate {
            x: x + v2.x,
            y: y + v2.y,
        },
    }
}

#[test]
fn intersection_1() {
    let _l: [[f64; 4]; 2] = [[200., 200., 300., 300.], [400., 200., 300., 300.]];
    let _l = to_lines(&_l);
    let rv = intersect(&_l[0], &_l[1]).unwrap().single();
    almost_equal(rv.x, 300.0, rv.y, 300.0);
}

#[test]
fn intersection_2() {
    let _l: [[f64; 4]; 2] = [[200., 200., 300., 400.], [400., 200., 300., 400.]];
    let _l = to_lines(&_l);
    let rv = intersect(&_l[0], &_l[1]).unwrap().single();
    almost_equal(rv.x, 300.0, rv.y, 400.0);
}

#[test]
fn intersection_3() {
    // line to point detection
    let _l: [[f64; 4]; 2] = [[200., 200., 300., 300.], [250., 250., 250., 250.]];
    let _l = to_lines(&_l);
    let rv = intersect(&_l[0], &_l[1]).unwrap().single();
    almost_equal(rv.x, 250.0, rv.y, 250.0);
}

#[test]
fn intersection_4() {
    // line to point detection
    let _l: [[f64; 4]; 2] = [[300., 300., 200., 200.], [250., 250., 250., 250.]];
    let _l = to_lines(&_l);
    let rv = intersect(&_l[0], &_l[1]).unwrap().single();
    almost_equal(rv.x, 250.0, rv.y, 250.0);
}

#[test]
fn intersection_5() {
    // line to point detection
    for r in (0..360).step_by(5) {
        let line = pivot(100.0, 100.0, 50.0, 50.0, r as f64);
        let vector = line.end - line.start;
        let point = scale_to_coordinate(&line.start, &vector, 0.45);
        println!("line:{:?}", line);
        println!("point:{:?}", point);
        let rv = intersect(&line, &geo::Line::new(point, point))
            .unwrap()
            .single();
        println!("rv:{:?}", rv);
        println!();
        almost_equal(rv.x, point.x, rv.y, point.y);
    }
}

#[test]
fn intersection_6() {
    let line1: geo::Line<f64> = [(100.0, 150.), (150.0, 100.)].into();
    let line2: geo::Line<f64> = [(150.0, 100.), (160.0, 150.)].into();
    let _rv = intersect(&line1, &line2);
}

#[test]
fn intersection_7() {
    // README.md example
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
        None => panic!("expected an overlap"),
    }
    // you can also get a single intersection point from the Intersection enum.
    // Albeit geometrically incorrect, it makes things easy
    if let Some(_rv) = _rv {
        println!("{:?}", _rv.single());
    }
}

#[ignore]
#[test]
fn intersection_8() {
    // line to point detection
    for r in (0..360).step_by(3) {
        let line1 = pivot(200.0, 200.0, 50.0, 1.0, r as f64);
        let line2 = pivot(200.0, 200.0, 50.0, 1.0, r as f64);

        println!(
            "line1:{:?} slope:{}",
            line1,
            (line1.start.x - line1.end.x) / (line1.start.y - line1.end.y)
        );
        println!(
            "line2:{:?} slope:{}",
            line2,
            (line2.start.x - line2.end.x) / (line2.start.y - line2.end.y)
        );

        let rv = intersect(&line1, &line2);
        match rv {
            Some(Intersection::Intersection(_a)) => panic!("expected an overlap, got {:?}", _a),
            Some(Intersection::OverLap(_a)) => println!("{:?}", _a),
            _ => panic!("expected an overlap, got None"),
        }
    }
}

//#[ignore]
#[test]
fn intersection_9() {
    // overlapping lines
    for r in (0..360).step_by(3) {
        let line1 = pivot(200.0, 200.0, 50.0, 1.0, r as f64);

        println!(
            "line1:{:?} slope:{}",
            line1,
            (line1.start.x - line1.end.x) / (line1.start.y - line1.end.y)
        );

        let rv = intersect(&line1, &line1);
        match rv {
            Some(Intersection::Intersection(_a)) => panic!("expected an overlap, got {:?}", _a),
            Some(Intersection::OverLap(_a)) => {
                println!("{:?}", _a);
                assert_eq!(_a.start, line1.start);
                assert_eq!(_a.end, line1.end);
            }
            _ => panic!("expected an overlap, got None"),
        }
    }
}

//#[ignore]
#[test]
fn intersection_10() {
    // overlapping lines
    for r in (0..360).step_by(3) {
        let line1 = pivot(200.0, 200.0, 50.0, 1.0, r as f64);
        let line2 = geo::Line {
            start: line1.end,
            end: line1.start,
        };

        println!(
            "line1:{:?} slope:{}",
            line1,
            (line1.start.x - line1.end.x) / (line1.start.y - line1.end.y)
        );
        println!(
            "line2:{:?} slope:{}",
            line2,
            (line2.start.x - line2.end.x) / (line2.start.y - line2.end.y)
        );

        let rv = intersect(&line1, &line1);
        match rv {
            Some(Intersection::Intersection(_a)) => panic!("expected an overlap, got {:?}", _a),
            Some(Intersection::OverLap(_a)) => {
                println!("{:?}", _a);
                assert_eq!(_a.start, line1.start);
                assert_eq!(_a.end, line1.end);
            }
            _ => panic!("expected an overlap, got None"),
        }
    }
}
