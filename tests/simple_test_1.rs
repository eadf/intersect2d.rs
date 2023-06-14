#[allow(unused_imports)]
use approx;
#[allow(unused_imports)]
use geo::algorithm::intersects::Intersects;
#[allow(unused_imports)]
use intersect2d::algorithm::{AlgorithmData, SiteEventKey};
#[allow(unused_imports)]
use intersect2d::{intersect, scale_to_coordinate, to_lines};
#[allow(unused_imports)]
use num_traits::Float;

#[test]
fn two_connected_1() -> Result<(), intersect2d::IntersectError> {
    let _l: [[f64; 4]; 2] = [[200., 200., 300., 300.], [400., 200., 300., 300.]];
    let mut iter = AlgorithmData::<f64>::default()
        .with_ignore_end_point_intersections(true)?
        .with_ref_lines(to_lines(&_l).iter())?
        .compute()?;
    //No result!
    assert!(iter.next().is_none());
    Ok(())
}

#[test]
fn two_connected_2() -> Result<(), intersect2d::IntersectError> {
    let _l: [[f64; 4]; 2] = [[200., 200., 300., 300.], [400., 200., 300., 300.]];
    let mut iter = AlgorithmData::<f64>::default()
        .with_ignore_end_point_intersections(false)?
        .with_ref_lines(to_lines(&_l).iter())?
        .compute()?;
    let (k, i) = iter.next().unwrap();
    let intersection = geo::Coordinate::from((300., 300.));
    let lines = [0, 1];
    assert_eq!(intersection, k);
    assert_eq!(
        i.iter().collect::<Vec<&usize>>().sort(),
        lines.iter().collect::<Vec<&usize>>().sort()
    );
    Ok(())
}

#[test]
fn two_connected_3() -> Result<(), intersect2d::IntersectError> {
    // README.md example
    // use geo;
    // use intersect2d::algorithm::AlgorithmData;
    let _l = vec![
        geo::Line::new(
            geo::Coordinate { x: 200., y: 200. },
            geo::Coordinate { x: 350., y: 300. },
        ),
        geo::Line::new(
            geo::Coordinate { x: 400., y: 200. },
            geo::Coordinate { x: 250., y: 300. },
        ),
    ];
    let results = AlgorithmData::<f64>::default()
        .with_ignore_end_point_intersections(false)?
        .with_lines(_l.into_iter())?
        .compute()?;
    for (p, l) in results {
        println!("Intersection @{:?} Involved lines:{:?}", p, l);
    }
    Ok(())
}


#[test]
fn two_connected_4() -> Result<(), intersect2d::IntersectError> {
    let intersection = intersect(
            &geo::Line::new(
                geo::Coordinate{ x: 0.0, y: 0.0 },
                geo::Coordinate{ x: 0.0, y: 100.0 },
            ),
            &geo::Line::new(
                geo::Coordinate{ x: 0.0, y: 100.0 },
                geo::Coordinate{ x: 0.0, y: 200.0 },
            ),
        );
    println!("intersection: {:?}", intersection);
    assert!(
        matches!(
            intersection.expect("lines should intersect"),
            intersect2d::Intersection::Intersection(coord)
            if coord == geo::Coordinate{ x: 0.0, y: 100.0 }
        )
    );
    Ok(())
}


//#[ignore]
#[test]
fn connected_3_1() -> Result<(), intersect2d::IntersectError> {
    // this is test connected_3_1
    let _l: [[f64; 4]; 3] = [
        [200., 200., 300., 300.],
        [400., 200., 300., 300.],
        [200., 300., 300., 300.],
    ];
    let mut iter = AlgorithmData::<f64>::default()
        .with_ignore_end_point_intersections(false)?
        .with_lines(to_lines(&_l).into_iter())?
        .compute()?;
    let (k, i) = iter.next().unwrap();
    let intersection = geo::Coordinate::from((300., 300.));
    let lines = [0, 1, 2];
    assert_eq!(intersection, k);
    assert_eq!(
        i.iter().collect::<Vec<&usize>>().sort(),
        lines.iter().collect::<Vec<&usize>>().sort()
    );
    Ok(())
}

#[test]
fn connected_3_1_linestring() -> Result<(), intersect2d::IntersectError> {
    // README example
    let coords = vec![(200., 200.), (300., 300.), (400., 200.), (200., 300.)];
    let line_string: geo::LineString<f32> = coords.into_iter().collect();

    let result = AlgorithmData::<f32>::default()
        .with_ignore_end_point_intersections(true)?
        .with_stop_at_first_intersection(true)?
        .with_lines(line_string.lines())?
        .compute()?;
    for (p, l) in result {
        println!("Intersection detected @{:?} Involved lines:{:?}", p, l);
    }
    Ok(())
}

//#[ignore]
#[test]
fn connected_3_2() -> Result<(), intersect2d::IntersectError> {
    // this is test connected_3_2
    let _l: [[f64; 4]; 3] = [
        [200., 200., 300., 300.],
        [400., 200., 300., 300.],
        [200., 300., 300., 300.],
    ];
    let mut iter = AlgorithmData::<f64>::default()
        .with_ignore_end_point_intersections(true)?
        .with_ref_lines(to_lines(&_l).iter())?
        .compute()?;
    //No result!
    assert!(iter.next().is_none());
    Ok(())
}

//#[ignore]
#[test]
fn connected_3_3() -> Result<(), intersect2d::IntersectError> {
    // this is test connected_3_3
    let _l: [[f64; 4]; 3] = [
        [200., 200., 300., 300.],
        [400., 200., 300., 300.],
        [200., 300., 400., 300.],
    ];
    let mut iter = AlgorithmData::<f64>::default()
        .with_ignore_end_point_intersections(true)?
        .with_ref_lines(to_lines(&_l).iter())?
        .compute()?;
    let (k, i) = iter.next().unwrap();
    let intersection = geo::Coordinate::from((300., 300.));
    let lines = [0, 1, 2];
    assert_eq!(intersection, k);
    assert_eq!(
        i.iter().collect::<Vec<&usize>>().sort(),
        lines.iter().collect::<Vec<&usize>>().sort()
    );
    Ok(())
}

//#[ignore]
#[test]
fn connected_3_4() -> Result<(), intersect2d::IntersectError> {
    // this is test connected_3_4
    let _l: [[f64; 4]; 3] = [
        [200., 200., 300., 300.],
        [400., 200., 300., 300.],
        [200., 300., 400., 300.],
    ];
    let mut iter = AlgorithmData::<f64>::default()
        .with_ignore_end_point_intersections(false)?
        .with_ref_lines(to_lines(&_l).iter())?
        .compute()?;
    let (k, i) = iter.next().unwrap();
    let intersection = geo::Coordinate::from((300., 300.));
    let lines = [0, 1, 2];
    assert_eq!(intersection, k);
    assert_eq!(
        i.iter().collect::<Vec<&usize>>().sort(),
        lines.iter().collect::<Vec<&usize>>().sort()
    );
    Ok(())
}

#[test]
fn connected_3_5() -> Result<(), intersect2d::IntersectError> {
    // this is test connected_3_5
    let _l: [[f64; 4]; 6] = [
        [200., 200., 300., 300.],
        [400., 200., 300., 300.],
        [200., 300., 400., 300.],
        [200., 200., 300., 300.],
        [400., 200., 300., 300.],
        [200., 300., 400., 300.],
    ];
    let _l = to_lines(&_l);
    let mut iter = AlgorithmData::<f64>::default()
        .with_ignore_end_point_intersections(true)?
        .with_ref_lines(_l.iter())?
        .compute()?;
    let (k, i) = iter.next().unwrap();
    let intersection = geo::Coordinate::from((300., 300.));
    let lines = [0, 1, 2, 3, 4, 5];
    assert_eq!(intersection, k);
    assert_eq!(
        i.iter().collect::<Vec<&usize>>().sort(),
        lines.iter().collect::<Vec<&usize>>().sort()
    );
    for lineid_1 in i.iter().rev().skip(1) {
        for lineid_2 in i.iter().skip(1) {
            if lineid_1 == lineid_2 {
                continue;
            }
            println!("line1:{} line2:{}", lineid_1, lineid_2);
            assert!(_l[*lineid_1].intersects(&_l[*lineid_2]));
        }
    }
    Ok(())
}

#[test]
fn connected_3_6() -> Result<(), intersect2d::IntersectError> {
    // this is test connected_3_6
    let _l: [[f64; 4]; 6] = [
        [200., 200., 300., 300.],
        [400., 200., 300., 300.],
        [200., 300., 300., 300.],
        [200., 200., 300., 300.],
        [400., 200., 300., 300.],
        [200., 300., 300., 300.],
    ];
    let mut iter = AlgorithmData::<f64>::default()
        .with_ignore_end_point_intersections(true)?
        .with_ref_lines(to_lines(&_l).iter())?
        .compute()?;
    //No result!
    assert!(iter.next().is_none());
    Ok(())
}

#[test]
fn connected_3_7() -> Result<(), intersect2d::IntersectError> {
    // this is test connected_3_7
    let _l: [[f64; 4]; 6] = [
        [200., 200., 300., 300.],
        [400., 200., 300., 300.],
        [200., 300., 300., 300.],
        [200., 200., 300., 300.],
        [400., 200., 300., 300.],
        [200., 300., 300., 300.],
    ];
    let _l = to_lines(&_l);
    let mut iter = AlgorithmData::<f64>::default()
        .with_ignore_end_point_intersections(false)?
        .with_ref_lines(_l.iter())?
        .compute()?;
    let (k, i) = iter.next().unwrap();
    let intersection = geo::Coordinate::from((200., 200.));
    let lines = [0, 3];
    assert_eq!(intersection, k);
    assert_eq!(
        i.iter().collect::<Vec<&usize>>().sort(),
        lines.iter().collect::<Vec<&usize>>().sort()
    );
    for lineid_1 in i.iter().rev().skip(1) {
        for lineid_2 in i.iter().skip(1) {
            if lineid_1 == lineid_2 {
                continue;
            }
            println!("line1:{} line2:{}", lineid_1, lineid_2);
            assert!(_l[*lineid_1].intersects(&_l[*lineid_2]));
        }
    }
    let (k, i) = iter.next().unwrap();
    let intersection = geo::Coordinate::from((400., 200.));
    let lines = [1, 4];
    assert_eq!(intersection, k);
    assert_eq!(
        i.iter().collect::<Vec<&usize>>().sort(),
        lines.iter().collect::<Vec<&usize>>().sort()
    );
    for lineid_1 in i.iter().rev().skip(1) {
        for lineid_2 in i.iter().skip(1) {
            if lineid_1 == lineid_2 {
                continue;
            }
            println!("line1:{} line2:{}", lineid_1, lineid_2);
            assert!(_l[*lineid_1].intersects(&_l[*lineid_2]));
        }
    }
    let (k, i) = iter.next().unwrap();
    let intersection = geo::Coordinate::from((200., 300.));
    let lines = [2, 5];
    assert_eq!(intersection, k);
    assert_eq!(
        i.iter().collect::<Vec<&usize>>().sort(),
        lines.iter().collect::<Vec<&usize>>().sort()
    );
    for lineid_1 in i.iter().rev().skip(1) {
        for lineid_2 in i.iter().skip(1) {
            if lineid_1 == lineid_2 {
                continue;
            }
            println!("line1:{} line2:{}", lineid_1, lineid_2);
            assert!(_l[*lineid_1].intersects(&_l[*lineid_2]));
        }
    }
    let (k, i) = iter.next().unwrap();
    let intersection = geo::Coordinate::from((300., 300.));
    let lines = [0, 1, 2, 3, 4, 5];
    assert_eq!(intersection, k);
    assert_eq!(
        i.iter().collect::<Vec<&usize>>().sort(),
        lines.iter().collect::<Vec<&usize>>().sort()
    );
    for lineid_1 in i.iter().rev().skip(1) {
        for lineid_2 in i.iter().skip(1) {
            if lineid_1 == lineid_2 {
                continue;
            }
            println!("line1:{} line2:{}", lineid_1, lineid_2);
            assert!(_l[*lineid_1].intersects(&_l[*lineid_2]));
        }
    }
    Ok(())
}

//#[ignore]
#[test]
fn chevron_1() -> Result<(), intersect2d::IntersectError> {
    // this is chevron_1
    let _l: [[f64; 4]; 5] = [
        [200., 200., 300., 300.],
        [400., 200., 300., 300.],
        [200., 250., 310., 350.],
        [400., 250., 290., 350.],
        [200., 250., 300., 400.],
    ];
    let mut iter = AlgorithmData::<f64>::default()
        .with_ignore_end_point_intersections(true)?
        .with_ref_lines(to_lines(&_l).iter())?
        .compute()?;
    let (k, i) = iter.next().unwrap();
    let intersection = geo::Coordinate::from((300., 340.9090909090909));
    let lines = [2, 3];
    assert_eq!(intersection, k);
    assert_eq!(
        i.iter().collect::<Vec<&usize>>().sort(),
        lines.iter().collect::<Vec<&usize>>().sort()
    );
    Ok(())
}

//#[ignore]
#[test]
fn chevron_2() -> Result<(), intersect2d::IntersectError> {
    // this is chevron_2
    let _l: [[f64; 4]; 5] = [
        [200., 200., 300., 300.],
        [400., 200., 300., 300.],
        [200., 250., 310., 350.],
        [400., 250., 290., 350.],
        [200., 250., 300., 400.],
    ];
    let mut iter = AlgorithmData::<f64>::default()
        .with_ignore_end_point_intersections(false)?
        .with_ref_lines(to_lines(&_l).iter())?
        .compute()?;
    let (k, i) = iter.next().unwrap();
    let intersection = geo::Coordinate::from((200., 250.));
    let lines = [2, 4];
    assert_eq!(intersection, k);
    assert_eq!(
        i.iter().collect::<Vec<&usize>>().sort(),
        lines.iter().collect::<Vec<&usize>>().sort()
    );
    let (k, i) = iter.next().unwrap();
    let intersection = geo::Coordinate::from((300., 300.));
    let lines = [0, 1];
    assert_eq!(intersection, k);
    assert_eq!(
        i.iter().collect::<Vec<&usize>>().sort(),
        lines.iter().collect::<Vec<&usize>>().sort()
    );
    let (k, i) = iter.next().unwrap();
    let intersection = geo::Coordinate::from((300., 340.9090909090909));
    let lines = [2, 3];
    assert_eq!(intersection, k);
    assert_eq!(
        i.iter().collect::<Vec<&usize>>().sort(),
        lines.iter().collect::<Vec<&usize>>().sort()
    );
    Ok(())
}

//#[ignore]
#[test]
fn chevron_3() -> Result<(), intersect2d::IntersectError> {
    // this is chevron_3
    let _l: [[f64; 4]; 10] = [
        [200., 200., 300., 300.],
        [400., 200., 300., 300.],
        [200., 250., 310., 350.],
        [400., 250., 290., 350.],
        [200., 250., 300., 400.],
        [200., 200., 300., 300.],
        [400., 200., 300., 300.],
        [200., 250., 310., 350.],
        [400., 250., 290., 350.],
        [200., 250., 300., 400.],
    ];
    let _l = to_lines(&_l);
    let mut iter = AlgorithmData::<f64>::default()
        .with_ignore_end_point_intersections(true)?
        .with_ref_lines(_l.iter())?
        .compute()?;
    let (k, i) = iter.next().unwrap();
    let intersection = geo::Coordinate::from((300., 340.9090909090909));
    let lines = [2, 3, 7, 8];
    assert_eq!(intersection, k);
    assert_eq!(
        i.iter().collect::<Vec<&usize>>().sort(),
        lines.iter().collect::<Vec<&usize>>().sort()
    );
    for lineid_1 in i.iter().rev().skip(1) {
        for lineid_2 in i.iter().skip(1) {
            if lineid_1 == lineid_2 {
                continue;
            }
            println!("line1:{} line2:{}", lineid_1, lineid_2);
            assert!(_l[*lineid_1].intersects(&_l[*lineid_2]));
        }
    }
    Ok(())
}

//#[ignore]
#[test]
fn connected_5_1() -> Result<(), intersect2d::IntersectError> {
    // this is connected_5_1
    let _l: [[f64; 4]; 5] = [
        [300., 300., 500., 300.],
        [
            306.0307379214091,
            315.7979856674331,
            775.8770483143635,
            486.8080573302675,
        ],
        [
            323.3955556881022,
            285.72123903134604,
            706.4177772475912,
            607.1150438746158,
        ],
        [350., 263.39745962155615, 600., 696.4101615137754],
        [
            382.635182233307,
            251.5192246987792,
            469.45927106677215,
            743.9231012048832,
        ],
    ];
    let mut iter = AlgorithmData::<f64>::default()
        .with_ignore_end_point_intersections(true)?
        .with_lines(to_lines(&_l).into_iter())?
        .compute()?;
    let (k, i) = iter.next().unwrap();
    let intersection = geo::Coordinate::from((340.41232037028954, 300.));
    let lines = [0, 2];
    assert_eq!(intersection, k);
    assert_eq!(
        i.iter().collect::<Vec<&usize>>().sort(),
        lines.iter().collect::<Vec<&usize>>().sort()
    );
    let (k, i) = iter.next().unwrap();
    let intersection = geo::Coordinate::from((371.1324865405187, 300.));
    let lines = [0, 3];
    assert_eq!(intersection, k);
    assert_eq!(
        i.iter().collect::<Vec<&usize>>().sort(),
        lines.iter().collect::<Vec<&usize>>().sort()
    );
    let (k, i) = iter.next().unwrap();
    let intersection = geo::Coordinate::from((391.18365096457677, 300.));
    let lines = [0, 4];
    assert_eq!(intersection, k);
    assert_eq!(
        i.iter().collect::<Vec<&usize>>().sort(),
        lines.iter().collect::<Vec<&usize>>().sort()
    );
    let (k, i) = iter.next().unwrap();
    let intersection = geo::Coordinate::from((400., 350.));
    let lines = [1, 2, 3, 4];
    assert_eq!(intersection, k);
    assert_eq!(
        i.iter().collect::<Vec<&usize>>().sort(),
        lines.iter().collect::<Vec<&usize>>().sort()
    );
    Ok(())
}

//#[ignore]
#[test]
fn connected_7_1() -> Result<(), intersect2d::IntersectError> {
    // this is connected_7_1
    let _l: [[f64; 4]; 5] = [
        [200., 200., 300., 300.],
        [400., 200., 300., 300.],
        [200., 300., 200., 400.],
        [400., 250., 400., 450.],
        [
            446.98463103929544,
            282.89899283371653,
            165.0768448035229,
            385.5050358314172,
        ],
    ];
    let mut iter = AlgorithmData::<f64>::default()
        .with_ignore_end_point_intersections(false)?
        .with_ref_lines(to_lines(&_l).iter())?
        .compute()?;
    let (k, i) = iter.next().unwrap();
    let intersection = geo::Coordinate::from((300., 300.));
    let lines = [0, 1];
    assert_eq!(intersection, k);
    assert_eq!(
        i.iter().collect::<Vec<&usize>>().sort(),
        lines.iter().collect::<Vec<&usize>>().sort()
    );
    let (k, i) = iter.next().unwrap();
    let intersection = geo::Coordinate::from((400., 300.));
    let lines = [3, 4];
    assert_eq!(intersection, k);
    assert_eq!(
        i.iter().collect::<Vec<&usize>>().sort(),
        lines.iter().collect::<Vec<&usize>>().sort()
    );
    let (k, i) = iter.next().unwrap();
    let intersection = geo::Coordinate::from((200., 372.7940468532405));
    let lines = [2, 4];
    assert_eq!(intersection, k);
    assert_eq!(
        i.iter().collect::<Vec<&usize>>().sort(),
        lines.iter().collect::<Vec<&usize>>().sort()
    );
    Ok(())
}

//#[ignore]
#[test]
fn connected_7_2() -> Result<(), intersect2d::IntersectError> {
    // this is connected_7_2
    let _l: [[f64; 4]; 7] = [
        [200., 200., 300., 300.],
        [400., 200., 300., 300.],
        [200., 250., 310., 350.],
        [400., 250., 290., 350.],
        [200., 250., 300., 400.],
        [400., 250., 400., 450.],
        [
            446.98463103929544,
            282.89899283371653,
            165.0768448035229,
            385.5050358314172,
        ],
    ];
    let mut iter = AlgorithmData::<f64>::default()
        .with_ignore_end_point_intersections(true)?
        .with_ref_lines(to_lines(&_l).iter())?
        .compute()?;
    let (k, i) = iter.next().unwrap();
    let intersection = geo::Coordinate::from((400., 300.));
    let lines = [5, 6];
    assert_eq!(intersection, k);
    assert_eq!(
        i.iter().collect::<Vec<&usize>>().sort(),
        lines.iter().collect::<Vec<&usize>>().sort()
    );
    let (k, i) = iter.next().unwrap();
    let intersection = geo::Coordinate::from((308.277190154128, 333.3843725871564));
    let lines = [3, 6];
    assert_eq!(intersection, k);
    assert_eq!(
        i.iter().collect::<Vec<&usize>>().sort(),
        lines.iter().collect::<Vec<&usize>>().sort()
    );
    let (k, i) = iter.next().unwrap();
    let intersection = geo::Coordinate::from((296.45573387734373, 337.6870307975852));
    let lines = [2, 6];
    assert_eq!(intersection, k);
    assert_eq!(
        i.iter().collect::<Vec<&usize>>().sort(),
        lines.iter().collect::<Vec<&usize>>().sort()
    );
    let (k, i) = iter.next().unwrap();
    let intersection = geo::Coordinate::from((300., 340.9090909090909));
    let lines = [2, 3];
    assert_eq!(intersection, k);
    assert_eq!(
        i.iter().collect::<Vec<&usize>>().sort(),
        lines.iter().collect::<Vec<&usize>>().sort()
    );
    let (k, i) = iter.next().unwrap();
    let intersection = geo::Coordinate::from((265.8776865616534, 348.81652984248007));
    let lines = [4, 6];
    assert_eq!(intersection, k);
    assert_eq!(
        i.iter().collect::<Vec<&usize>>().sort(),
        lines.iter().collect::<Vec<&usize>>().sort()
    );
    Ok(())
}

#[test]
fn complex_1() -> Result<(), intersect2d::IntersectError> {
    // this is complex_1
    let _l: [[f64; 4]; 15] = [
        [10., 5., 78., 12.],
        [45., 14., 55., 23.],
        [14., 23., 234., 324.],
        [44., 43., 31., 64.],
        [234., 324., 450., 723.],
        [200., 200., 300., 300.],
        [300., 200., 200., 300.],
        [300., 300., 200., 400.],
        [300., 350., 600., 350.],
        [250., 50., 250., 400.],
        [320., 320., 520., 520.],
        [360., 320., 480., 520.],
        [400., 320., 440., 520.],
        [440., 320., 400., 520.],
        [480., 320., 360., 520.],
    ];
    let mut iter = AlgorithmData::<f64>::default()
        .with_ignore_end_point_intersections(true)?
        .with_ref_lines(to_lines(&_l).iter())?
        .compute()?;
    let (k, i) = iter.next().unwrap();
    let intersection = geo::Coordinate::from((36.946208836282665, 54.39458572600492));
    let lines = [2, 3];
    assert_eq!(intersection, k);
    assert_eq!(
        i.iter().collect::<Vec<&usize>>().sort(),
        lines.iter().collect::<Vec<&usize>>().sort()
    );
    let (k, i) = iter.next().unwrap();
    let intersection = geo::Coordinate::from((250., 250.));
    let lines = [5, 6, 9];
    assert_eq!(intersection, k);
    assert_eq!(
        i.iter().collect::<Vec<&usize>>().sort(),
        lines.iter().collect::<Vec<&usize>>().sort()
    );
    let (k, i) = iter.next().unwrap();
    let intersection = geo::Coordinate::from((209.50863723608447, 290.49136276391556));
    let lines = [2, 6];
    assert_eq!(intersection, k);
    assert_eq!(
        i.iter().collect::<Vec<&usize>>().sort(),
        lines.iter().collect::<Vec<&usize>>().sort()
    );
    let (k, i) = iter.next().unwrap();
    let intersection = geo::Coordinate::from((250., 350.));
    let lines = [7, 9];
    assert_eq!(intersection, k);
    assert_eq!(
        i.iter().collect::<Vec<&usize>>().sort(),
        lines.iter().collect::<Vec<&usize>>().sort()
    );
    let (k, i) = iter.next().unwrap();
    let intersection = geo::Coordinate::from((350., 350.));
    let lines = [8, 10];
    assert_eq!(intersection, k);
    assert_eq!(
        i.iter().collect::<Vec<&usize>>().sort(),
        lines.iter().collect::<Vec<&usize>>().sort()
    );
    let (k, i) = iter.next().unwrap();
    let intersection = geo::Coordinate::from((378., 350.));
    let lines = [8, 11];
    assert_eq!(intersection, k);
    assert_eq!(
        i.iter().collect::<Vec<&usize>>().sort(),
        lines.iter().collect::<Vec<&usize>>().sort()
    );
    let (k, i) = iter.next().unwrap();
    let intersection = geo::Coordinate::from((406., 350.));
    let lines = [8, 12];
    assert_eq!(intersection, k);
    assert_eq!(
        i.iter().collect::<Vec<&usize>>().sort(),
        lines.iter().collect::<Vec<&usize>>().sort()
    );
    let (k, i) = iter.next().unwrap();
    let intersection = geo::Coordinate::from((434., 350.));
    let lines = [8, 13];
    assert_eq!(intersection, k);
    assert_eq!(
        i.iter().collect::<Vec<&usize>>().sort(),
        lines.iter().collect::<Vec<&usize>>().sort()
    );
    let (k, i) = iter.next().unwrap();
    let intersection = geo::Coordinate::from((462., 350.));
    let lines = [8, 14];
    assert_eq!(intersection, k);
    assert_eq!(
        i.iter().collect::<Vec<&usize>>().sort(),
        lines.iter().collect::<Vec<&usize>>().sort()
    );
    let (k, i) = iter.next().unwrap();
    let intersection = geo::Coordinate::from((248.75121951219512, 351.2487804878049));
    let lines = [4, 7];
    assert_eq!(intersection, k);
    assert_eq!(
        i.iter().collect::<Vec<&usize>>().sort(),
        lines.iter().collect::<Vec<&usize>>().sort()
    );
    let (k, i) = iter.next().unwrap();
    let intersection = geo::Coordinate::from((250., 353.55555555555554));
    let lines = [4, 9];
    assert_eq!(intersection, k);
    assert_eq!(
        i.iter().collect::<Vec<&usize>>().sort(),
        lines.iter().collect::<Vec<&usize>>().sort()
    );
    let (k, i) = iter.next().unwrap();
    let intersection = geo::Coordinate::from((420., 420.));
    let lines = [10, 11, 12, 13, 14];
    assert_eq!(intersection, k);
    assert_eq!(
        i.iter().collect::<Vec<&usize>>().sort(),
        lines.iter().collect::<Vec<&usize>>().sort()
    );

    // uses a true n^2 'algorithm'
    //let _bf = brute_force(&_l);
    Ok(())
}

//#[ignore]
#[test]
fn a_test() -> Result<(), intersect2d::IntersectError> {
    let _l: [[f32; 4]; 6] = [
        [651.3134, 410.21536, 335.7384, 544.54614],
        [335.7384, 544.54614, 154.29922, 363.10654],
        [154.29922, 363.10654, 425.06284, 255.50153],
        [425.06284, 255.50153, 651.1434, 387.16595],
        [651.1434, 387.16595, 250.0, 300.0],
        [250.0, 300.0, 651.3134, 410.21536],
    ];
    let _l: Vec<[i32; 4]> = _l
        .iter()
        .map(|x| [x[0] as i32, x[1] as i32, x[2] as i32, x[3] as i32])
        .collect();

    let result = AlgorithmData::<f64>::default()
        .with_ignore_end_point_intersections(true)?
        .with_ref_lines(to_lines(&_l).iter())?
        .compute()?;

    assert_eq!(result.count(), 2);
    Ok(())
}
