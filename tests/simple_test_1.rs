use approx;
use geo::algorithm::intersects::Intersects;
use intersect2d::algorithm::{AlgorithmData, SiteEventKey};
#[allow(unused_imports)]
use intersect2d::{intersect, scale_to_coordinate, ulps_eq};
use num_traits::{Float, ToPrimitive};

#[allow(dead_code)]
fn almost_equal<T>(x1: T, x2: T, y1: T, y2: T)
where
    T: Float + approx::UlpsEq + geo::CoordNum + PartialOrd,
    T::Epsilon: Copy,
{
    assert!(ulps_eq(&x1, &x2));
    assert!(ulps_eq(&y1, &y2));
}

/// Convert an array slice into a vec of Line
fn to_lines<U, T>(points: &[[U; 4]]) -> Vec<geo::Line<T>>
where
    U: ToPrimitive + Copy,
    T: Float + approx::UlpsEq + geo::CoordNum + PartialOrd,
    T::Epsilon: Copy,
{
    let mut rv = Vec::with_capacity(points.len());
    for p in points.iter() {
        rv.push(geo::Line::<T>::new(
            geo::Coordinate {
                x: T::from(p[0]).unwrap(),
                y: T::from(p[1]).unwrap(),
            },
            geo::Coordinate {
                x: T::from(p[2]).unwrap(),
                y: T::from(p[3]).unwrap(),
            },
        ));
    }
    rv
}

#[test]
fn two_connected_1() {
    let _l: [[f64; 4]; 2] = [[200., 200., 300., 300.], [400., 200., 300., 300.]];
    let _l = to_lines(&_l);
    let mut ad = AlgorithmData::<f64>::default();
    ad.with_ignore_end_point_intersections(true);
    ad.with_lines(_l.iter());
    ad.compute(false);
    assert!(ad.get_results().is_some());
    let _result = ad.get_results().as_ref().unwrap();
    let mut iter = ad.get_results().as_ref().unwrap().iter();
    //No result!
    assert!(iter.next().is_none());
}

#[test]
fn two_connected_2() {
    let _l: [[f64; 4]; 2] = [[200., 200., 300., 300.], [400., 200., 300., 300.]];
    let _l = to_lines(&_l);
    let mut ad = AlgorithmData::<f64>::default();
    ad.with_ignore_end_point_intersections(false);
    ad.with_lines(_l.iter());
    ad.compute(false);
    assert!(ad.get_results().is_some());
    let _result = ad.get_results().as_ref().unwrap();
    let mut iter = ad.get_results().as_ref().unwrap().iter();
    let (k, i) = iter.next().unwrap();
    let intersection = SiteEventKey::new(300., 300.);
    let lines = [0, 1];
    assert_eq!(&intersection, k);
    assert_eq!(
        i.iter().collect::<Vec<&usize>>().sort(),
        lines.iter().collect::<Vec<&usize>>().sort()
    );
}

//#[ignore]
#[test]
fn connected_3_1() {
    // this is test connected_3_1
    let _l: [[f64; 4]; 3] = [
        [200., 200., 300., 300.],
        [400., 200., 300., 300.],
        [200., 300., 300., 300.],
    ];
    let _l = to_lines(&_l);
    let mut ad = AlgorithmData::<f64>::default();
    ad.with_ignore_end_point_intersections(false);
    ad.with_lines(_l.iter());
    ad.compute(false);
    assert!(ad.get_results().is_some());
    let _result = ad.get_results().as_ref().unwrap();
    let mut iter = ad.get_results().as_ref().unwrap().iter();
    let (k, i) = iter.next().unwrap();
    let intersection = SiteEventKey::new(300., 300.);
    let lines = [0, 1, 2];
    assert_eq!(&intersection, k);
    assert_eq!(
        i.iter().collect::<Vec<&usize>>().sort(),
        lines.iter().collect::<Vec<&usize>>().sort()
    );
}

//#[ignore]
#[test]
fn connected_3_2() {
    // this is test connected_3_2
    let _l: [[f64; 4]; 3] = [
        [200., 200., 300., 300.],
        [400., 200., 300., 300.],
        [200., 300., 300., 300.],
    ];
    let _l = to_lines(&_l);
    let mut ad = AlgorithmData::<f64>::default();
    ad.with_ignore_end_point_intersections(true);
    ad.with_lines(_l.iter());
    ad.compute(false);
    assert!(ad.get_results().is_some());
    let _result = ad.get_results().as_ref().unwrap();
    let mut iter = ad.get_results().as_ref().unwrap().iter();
    //No result!
    assert!(iter.next().is_none());
}

//#[ignore]
#[test]
fn connected_3_3() {
    // this is test connected_3_3
    let _l: [[f64; 4]; 3] = [
        [200., 200., 300., 300.],
        [400., 200., 300., 300.],
        [200., 300., 400., 300.],
    ];
    let _l = to_lines(&_l);
    let mut ad = AlgorithmData::<f64>::default();
    ad.with_ignore_end_point_intersections(true);
    ad.with_lines(_l.iter());
    ad.compute(false);
    assert!(ad.get_results().is_some());
    let _result = ad.get_results().as_ref().unwrap();
    let mut iter = ad.get_results().as_ref().unwrap().iter();
    let (k, i) = iter.next().unwrap();
    let intersection = SiteEventKey::new(300., 300.);
    let lines = [0, 1, 2];
    assert_eq!(&intersection, k);
    assert_eq!(
        i.iter().collect::<Vec<&usize>>().sort(),
        lines.iter().collect::<Vec<&usize>>().sort()
    );
}

//#[ignore]
#[test]
fn connected_3_4() {
    // this is test connected_3_4
    let _l: [[f64; 4]; 3] = [
        [200., 200., 300., 300.],
        [400., 200., 300., 300.],
        [200., 300., 400., 300.],
    ];
    let _l = to_lines(&_l);
    let mut ad = AlgorithmData::<f64>::default();
    ad.with_ignore_end_point_intersections(false);
    ad.with_lines(_l.iter());
    ad.compute(false);
    assert!(ad.get_results().is_some());
    let _result = ad.get_results().as_ref().unwrap();
    let mut iter = ad.get_results().as_ref().unwrap().iter();
    let (k, i) = iter.next().unwrap();
    let intersection = SiteEventKey::new(300., 300.);
    let lines = [0, 1, 2];
    assert_eq!(&intersection, k);
    assert_eq!(
        i.iter().collect::<Vec<&usize>>().sort(),
        lines.iter().collect::<Vec<&usize>>().sort()
    );
}

#[test]
fn connected_3_5() {
    // this is test connected_3_5
    let _l: [[f64; 4]; 6] = [
        [200., 200., 300., 300.],
        [400., 200., 300., 300.],
        [200., 300., 400., 300.],
        [200., 200., 300., 300.],
        [400., 200., 300., 300.],
        [200., 300., 400., 300.],
    ];
    let mut ad = AlgorithmData::<f64>::default();
    ad.with_ignore_end_point_intersections(true);
    let _l = to_lines(&_l);
    ad.with_lines(_l.iter());
    ad.compute(false);
    assert!(ad.get_results().is_some());
    let _result = ad.get_results().as_ref().unwrap();
    let mut iter = ad.get_results().as_ref().unwrap().iter();
    let (k, i) = iter.next().unwrap();
    let intersection = SiteEventKey::new(300., 300.);
    let lines = [0, 1, 2, 3, 4, 5];
    assert_eq!(&intersection, k);
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
}

#[test]
fn connected_3_6() {
    // this is test connected_3_6
    let _l: [[f64; 4]; 6] = [
        [200., 200., 300., 300.],
        [400., 200., 300., 300.],
        [200., 300., 300., 300.],
        [200., 200., 300., 300.],
        [400., 200., 300., 300.],
        [200., 300., 300., 300.],
    ];
    let mut ad = AlgorithmData::<f64>::default();
    ad.with_ignore_end_point_intersections(true);
    let _l = to_lines(&_l);
    ad.with_lines(_l.iter());
    ad.compute(false);
    assert!(ad.get_results().is_some());
    let _result = ad.get_results().as_ref().unwrap();
    let mut iter = ad.get_results().as_ref().unwrap().iter();
    //No result!
    assert!(iter.next().is_none());
}

#[test]
fn connected_3_7() {
    // this is test connected_3_7
    let _l: [[f64; 4]; 6] = [
        [200., 200., 300., 300.],
        [400., 200., 300., 300.],
        [200., 300., 300., 300.],
        [200., 200., 300., 300.],
        [400., 200., 300., 300.],
        [200., 300., 300., 300.],
    ];
    let mut ad = AlgorithmData::<f64>::default();
    ad.with_ignore_end_point_intersections(false);
    let _l = to_lines(&_l);
    ad.with_lines(_l.iter());
    ad.compute(false);
    assert!(ad.get_results().is_some());
    let _result = ad.get_results().as_ref().unwrap();
    let mut iter = ad.get_results().as_ref().unwrap().iter();
    let (k, i) = iter.next().unwrap();
    let intersection = SiteEventKey::new(200., 200.);
    let lines = [0, 3];
    assert_eq!(&intersection, k);
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
    let intersection = SiteEventKey::new(400., 200.);
    let lines = [1, 4];
    assert_eq!(&intersection, k);
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
    let intersection = SiteEventKey::new(200., 300.);
    let lines = [2, 5];
    assert_eq!(&intersection, k);
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
    let intersection = SiteEventKey::new(300., 300.);
    let lines = [0, 1, 2, 3, 4, 5];
    assert_eq!(&intersection, k);
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
}

//#[ignore]
#[test]
fn chevron_1() {
    // this is chevron_1
    let _l: [[f64; 4]; 5] = [
        [200., 200., 300., 300.],
        [400., 200., 300., 300.],
        [200., 250., 310., 350.],
        [400., 250., 290., 350.],
        [200., 250., 300., 400.],
    ];
    let mut ad = AlgorithmData::<f64>::default();
    ad.with_ignore_end_point_intersections(true);
    let _l = to_lines(&_l);
    ad.with_lines(_l.iter());
    ad.compute(false);
    assert!(ad.get_results().is_some());
    let _result = ad.get_results().as_ref().unwrap();
    let mut iter = ad.get_results().as_ref().unwrap().iter();
    let (k, i) = iter.next().unwrap();
    let intersection = SiteEventKey::new(300., 340.9090909090909);
    let lines = [2, 3];
    assert_eq!(&intersection, k);
    assert_eq!(
        i.iter().collect::<Vec<&usize>>().sort(),
        lines.iter().collect::<Vec<&usize>>().sort()
    );
}

//#[ignore]
#[test]
fn chevron_2() {
    // this is chevron_2
    let _l: [[f64; 4]; 5] = [
        [200., 200., 300., 300.],
        [400., 200., 300., 300.],
        [200., 250., 310., 350.],
        [400., 250., 290., 350.],
        [200., 250., 300., 400.],
    ];
    let mut ad = AlgorithmData::<f64>::default();
    ad.with_ignore_end_point_intersections(false);
    let _l = to_lines(&_l);
    ad.with_lines(_l.iter());
    ad.compute(false);
    assert!(ad.get_results().is_some());
    let _result = ad.get_results().as_ref().unwrap();
    let mut iter = ad.get_results().as_ref().unwrap().iter();
    let (k, i) = iter.next().unwrap();
    let intersection = SiteEventKey::new(200., 250.);
    let lines = [2, 4];
    assert_eq!(&intersection, k);
    assert_eq!(
        i.iter().collect::<Vec<&usize>>().sort(),
        lines.iter().collect::<Vec<&usize>>().sort()
    );
    let (k, i) = iter.next().unwrap();
    let intersection = SiteEventKey::new(300., 300.);
    let lines = [0, 1];
    assert_eq!(&intersection, k);
    assert_eq!(
        i.iter().collect::<Vec<&usize>>().sort(),
        lines.iter().collect::<Vec<&usize>>().sort()
    );
    let (k, i) = iter.next().unwrap();
    let intersection = SiteEventKey::new(300., 340.9090909090909);
    let lines = [2, 3];
    assert_eq!(&intersection, k);
    assert_eq!(
        i.iter().collect::<Vec<&usize>>().sort(),
        lines.iter().collect::<Vec<&usize>>().sort()
    );
}

//#[ignore]
#[test]
fn chevron_3() {
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
    let mut ad = AlgorithmData::<f64>::default();
    ad.with_ignore_end_point_intersections(true);
    let _l = to_lines(&_l);
    ad.with_lines(_l.iter());
    ad.compute(false);
    assert!(ad.get_results().is_some());
    let _result = ad.get_results().as_ref().unwrap();
    let mut iter = ad.get_results().as_ref().unwrap().iter();
    let (k, i) = iter.next().unwrap();
    let intersection = SiteEventKey::new(300., 340.9090909090909);
    let lines = [2, 3, 7, 8];
    assert_eq!(&intersection, k);
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
}

//#[ignore]
#[test]
fn connected_5_1() {
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
    let mut ad = AlgorithmData::<f64>::default();
    ad.with_ignore_end_point_intersections(true);
    let _l = to_lines(&_l);
    ad.with_lines(_l.iter());
    ad.compute(false);
    assert!(ad.get_results().is_some());
    let _result = ad.get_results().as_ref().unwrap();
    let mut iter = ad.get_results().as_ref().unwrap().iter();
    let (k, i) = iter.next().unwrap();
    let intersection = SiteEventKey::new(340.41232037028954, 300.);
    let lines = [0, 2];
    assert_eq!(&intersection, k);
    assert_eq!(
        i.iter().collect::<Vec<&usize>>().sort(),
        lines.iter().collect::<Vec<&usize>>().sort()
    );
    let (k, i) = iter.next().unwrap();
    let intersection = SiteEventKey::new(371.1324865405187, 300.);
    let lines = [0, 3];
    assert_eq!(&intersection, k);
    assert_eq!(
        i.iter().collect::<Vec<&usize>>().sort(),
        lines.iter().collect::<Vec<&usize>>().sort()
    );
    let (k, i) = iter.next().unwrap();
    let intersection = SiteEventKey::new(391.18365096457677, 300.);
    let lines = [0, 4];
    assert_eq!(&intersection, k);
    assert_eq!(
        i.iter().collect::<Vec<&usize>>().sort(),
        lines.iter().collect::<Vec<&usize>>().sort()
    );
    let (k, i) = iter.next().unwrap();
    let intersection = SiteEventKey::new(400., 350.);
    let lines = [1, 2, 3, 4];
    assert_eq!(&intersection, k);
    assert_eq!(
        i.iter().collect::<Vec<&usize>>().sort(),
        lines.iter().collect::<Vec<&usize>>().sort()
    );
}

//#[ignore]
#[test]
fn connected_7_1() {
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
    let mut ad = AlgorithmData::<f64>::default();
    ad.with_ignore_end_point_intersections(false);
    let _l = to_lines(&_l);
    ad.with_lines(_l.iter());
    ad.compute(false);
    assert!(ad.get_results().is_some());
    let _result = ad.get_results().as_ref().unwrap();
    let mut iter = ad.get_results().as_ref().unwrap().iter();
    let (k, i) = iter.next().unwrap();
    let intersection = SiteEventKey::new(300., 300.);
    let lines = [0, 1];
    assert_eq!(&intersection, k);
    assert_eq!(
        i.iter().collect::<Vec<&usize>>().sort(),
        lines.iter().collect::<Vec<&usize>>().sort()
    );
    let (k, i) = iter.next().unwrap();
    let intersection = SiteEventKey::new(400., 300.);
    let lines = [3, 4];
    assert_eq!(&intersection, k);
    assert_eq!(
        i.iter().collect::<Vec<&usize>>().sort(),
        lines.iter().collect::<Vec<&usize>>().sort()
    );
    let (k, i) = iter.next().unwrap();
    let intersection = SiteEventKey::new(200., 372.7940468532405);
    let lines = [2, 4];
    assert_eq!(&intersection, k);
    assert_eq!(
        i.iter().collect::<Vec<&usize>>().sort(),
        lines.iter().collect::<Vec<&usize>>().sort()
    );
}

//#[ignore]
#[test]
fn connected_7_2() {
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
    let mut ad = AlgorithmData::<f64>::default();
    ad.with_ignore_end_point_intersections(true);
    let _l = to_lines(&_l);
    ad.with_lines(_l.iter());
    ad.compute(false);
    assert!(ad.get_results().is_some());
    let _result = ad.get_results().as_ref().unwrap();
    let mut iter = ad.get_results().as_ref().unwrap().iter();
    let (k, i) = iter.next().unwrap();
    let intersection = SiteEventKey::new(400., 300.);
    let lines = [5, 6];
    assert_eq!(&intersection, k);
    assert_eq!(
        i.iter().collect::<Vec<&usize>>().sort(),
        lines.iter().collect::<Vec<&usize>>().sort()
    );
    let (k, i) = iter.next().unwrap();
    let intersection = SiteEventKey::new(308.277190154128, 333.3843725871564);
    let lines = [3, 6];
    assert_eq!(&intersection, k);
    assert_eq!(
        i.iter().collect::<Vec<&usize>>().sort(),
        lines.iter().collect::<Vec<&usize>>().sort()
    );
    let (k, i) = iter.next().unwrap();
    let intersection = SiteEventKey::new(296.45573387734373, 337.6870307975852);
    let lines = [2, 6];
    assert_eq!(&intersection, k);
    assert_eq!(
        i.iter().collect::<Vec<&usize>>().sort(),
        lines.iter().collect::<Vec<&usize>>().sort()
    );
    let (k, i) = iter.next().unwrap();
    let intersection = SiteEventKey::new(300., 340.9090909090909);
    let lines = [2, 3];
    assert_eq!(&intersection, k);
    assert_eq!(
        i.iter().collect::<Vec<&usize>>().sort(),
        lines.iter().collect::<Vec<&usize>>().sort()
    );
    let (k, i) = iter.next().unwrap();
    let intersection = SiteEventKey::new(265.8776865616534, 348.81652984248007);
    let lines = [4, 6];
    assert_eq!(&intersection, k);
    assert_eq!(
        i.iter().collect::<Vec<&usize>>().sort(),
        lines.iter().collect::<Vec<&usize>>().sort()
    );
}

#[test]
fn complex_1() {
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
    let mut ad = AlgorithmData::<f64>::default();
    ad.with_ignore_end_point_intersections(true);
    let _l = to_lines(&_l);
    ad.with_lines(_l.iter());
    ad.compute(false);
    assert!(ad.get_results().is_some());
    let _result = ad.get_results().as_ref().unwrap();
    let mut iter = ad.get_results().as_ref().unwrap().iter();
    let (k, i) = iter.next().unwrap();
    let intersection = SiteEventKey::new(36.946208836282665, 54.39458572600492);
    let lines = [2, 3];
    assert_eq!(&intersection, k);
    assert_eq!(
        i.iter().collect::<Vec<&usize>>().sort(),
        lines.iter().collect::<Vec<&usize>>().sort()
    );
    let (k, i) = iter.next().unwrap();
    let intersection = SiteEventKey::new(250., 250.);
    let lines = [5, 6, 9];
    assert_eq!(&intersection, k);
    assert_eq!(
        i.iter().collect::<Vec<&usize>>().sort(),
        lines.iter().collect::<Vec<&usize>>().sort()
    );
    let (k, i) = iter.next().unwrap();
    let intersection = SiteEventKey::new(209.50863723608447, 290.49136276391556);
    let lines = [2, 6];
    assert_eq!(&intersection, k);
    assert_eq!(
        i.iter().collect::<Vec<&usize>>().sort(),
        lines.iter().collect::<Vec<&usize>>().sort()
    );
    let (k, i) = iter.next().unwrap();
    let intersection = SiteEventKey::new(250., 350.);
    let lines = [7, 9];
    assert_eq!(&intersection, k);
    assert_eq!(
        i.iter().collect::<Vec<&usize>>().sort(),
        lines.iter().collect::<Vec<&usize>>().sort()
    );
    let (k, i) = iter.next().unwrap();
    let intersection = SiteEventKey::new(350., 350.);
    let lines = [8, 10];
    assert_eq!(&intersection, k);
    assert_eq!(
        i.iter().collect::<Vec<&usize>>().sort(),
        lines.iter().collect::<Vec<&usize>>().sort()
    );
    let (k, i) = iter.next().unwrap();
    let intersection = SiteEventKey::new(378., 350.);
    let lines = [8, 11];
    assert_eq!(&intersection, k);
    assert_eq!(
        i.iter().collect::<Vec<&usize>>().sort(),
        lines.iter().collect::<Vec<&usize>>().sort()
    );
    let (k, i) = iter.next().unwrap();
    let intersection = SiteEventKey::new(406., 350.);
    let lines = [8, 12];
    assert_eq!(&intersection, k);
    assert_eq!(
        i.iter().collect::<Vec<&usize>>().sort(),
        lines.iter().collect::<Vec<&usize>>().sort()
    );
    let (k, i) = iter.next().unwrap();
    let intersection = SiteEventKey::new(434., 350.);
    let lines = [8, 13];
    assert_eq!(&intersection, k);
    assert_eq!(
        i.iter().collect::<Vec<&usize>>().sort(),
        lines.iter().collect::<Vec<&usize>>().sort()
    );
    let (k, i) = iter.next().unwrap();
    let intersection = SiteEventKey::new(462., 350.);
    let lines = [8, 14];
    assert_eq!(&intersection, k);
    assert_eq!(
        i.iter().collect::<Vec<&usize>>().sort(),
        lines.iter().collect::<Vec<&usize>>().sort()
    );
    let (k, i) = iter.next().unwrap();
    let intersection = SiteEventKey::new(248.75121951219512, 351.2487804878049);
    let lines = [4, 7];
    assert_eq!(&intersection, k);
    assert_eq!(
        i.iter().collect::<Vec<&usize>>().sort(),
        lines.iter().collect::<Vec<&usize>>().sort()
    );
    let (k, i) = iter.next().unwrap();
    let intersection = SiteEventKey::new(250., 353.55555555555554);
    let lines = [4, 9];
    assert_eq!(&intersection, k);
    assert_eq!(
        i.iter().collect::<Vec<&usize>>().sort(),
        lines.iter().collect::<Vec<&usize>>().sort()
    );
    let (k, i) = iter.next().unwrap();
    let intersection = SiteEventKey::new(420., 420.);
    let lines = [10, 11, 12, 13, 14];
    assert_eq!(&intersection, k);
    assert_eq!(
        i.iter().collect::<Vec<&usize>>().sort(),
        lines.iter().collect::<Vec<&usize>>().sort()
    );

    // uses a true n^2 'algorithm'
    //let _bf = brute_force(&_l);
}
