#[allow(unused_imports)]
use approx;
#[allow(unused_imports)]
use intersect2d::algorithm::{AlgorithmData, SiteEventKey};
#[allow(unused_imports)]
use intersect2d::{intersect, scale_to_coordinate, to_lines, Intersection};
#[allow(unused_imports)]
use num_traits::Float;

//#[ignore]
#[test]
fn large_test_1() -> Result<(), intersect2d::IntersectError> {
    let _l: [[f64; 4]; 52] = [
        [200., 200., 200., 400.],
        [200., 400., 400., 400.],
        [400., 200., 400., 400.],
        [200., 200., 400., 200.],
        [367., 107., 529., 242.],
        [667., 431., 464., 554.],
        [464., 554., 230., 588.],
        [88., 464., 230., 588.],
        [80., 236., 88., 464.],
        [178., 97., 80., 236.],
        [463., 56., 178., 97.],
        [463., 56., 670., 175.],
        [670., 175., 732., 346.],
        [732., 346., 735., 479.],
        [735., 479., 512., 643.],
        [512., 643., 257., 710.],
        [100., 615., 257., 710.],
        [36., 470., 100., 615.],
        [53., 195., 36., 470.],
        [82., 83., 53., 195.],
        [211., 430., 82., 483.],
        [379., 35., 614., 55.],
        [759., 140., 784., 390.],
        [734., 594., 678., 686.],
        [485., 742., 203., 745.],
        [12., 537., 103., 724.],
        [138., 145., 122., 425.],
        [43., 125., 508., 217.],
        [512., 691., 629., 758.],
        [643., 601., 629., 758.],
        [618., 281., 499., 455.],
        [612., 209., 618., 281.],
        [486., 125., 612., 209.],
        [486., 125., 486., 125.],
        [462., 458., 361., 494.],
        [148., 470., 215., 498.],
        [453., 233., 494., 371.],
        [560., 262., 494., 371.],
        [563., 200., 560., 262.],
        [451., 141., 563., 200.],
        [421., 82., 451., 141.],
        [421., 82., 243., 111.],
        [243., 111., 145., 187.],
        [145., 187., 144., 319.],
        [144., 319., 177., 442.],
        [177., 442., 266., 484.],
        [266., 484., 336., 541.],
        [433., 497., 336., 541.],
        [525., 467., 433., 497.],
        [594., 427., 525., 467.],
        [617., 342., 594., 427.],
        [675., 292., 617., 342.],
    ];
    let mut iter = AlgorithmData::<f64>::default()
        .with_ignore_end_point_intersections(true)?
        .with_ref_lines(to_lines(&_l).iter())?
        .compute()?;

    let (k, i) = iter.next().unwrap();
    let intersection = geo::Coordinate::<f64>::from((69.75440198728721, 130.2933440490977));
    let lines = [19, 27];
    assert_eq!(intersection, k);
    assert_eq!(
        i.iter().collect::<Vec<&usize>>().sort(),
        lines.iter().collect::<Vec<&usize>>().sort()
    );
    let (k, i) = iter.next().unwrap();
    let intersection = geo::Coordinate::<f64>::from((144.1495431155042, 145.01238272392771));
    let lines = [9, 27];
    assert_eq!(intersection, k);
    assert_eq!(
        i.iter().collect::<Vec<&usize>>().sort(),
        lines.iter().collect::<Vec<&usize>>().sort()
    );
    let (k, i) = iter.next().unwrap();
    let intersection = geo::Coordinate::<f64>::from((187.96392821715213, 153.68103526016773));
    let lines = [27, 41];
    assert_eq!(intersection, k);
    assert_eq!(
        i.iter().collect::<Vec<&usize>>().sort(),
        lines.iter().collect::<Vec<&usize>>().sort()
    );
    let (k, i) = iter.next().unwrap();
    let intersection = geo::Coordinate::<f64>::from((137.45685279187816, 154.50507614213197));
    let lines = [9, 26];
    assert_eq!(intersection, k);
    assert_eq!(
        i.iter().collect::<Vec<&usize>>().sort(),
        lines.iter().collect::<Vec<&usize>>().sort()
    );
    let (k, i) = iter.next().unwrap();
    let intersection = geo::Coordinate::<f64>::from((496.1979695431472, 214.66497461928935));
    let lines = [4, 27];
    assert_eq!(intersection, k);
    assert_eq!(
        i.iter().collect::<Vec<&usize>>().sort(),
        lines.iter().collect::<Vec<&usize>>().sort()
    );
    let (k, i) = iter.next().unwrap();
    let intersection = geo::Coordinate::<f64>::from((179.23048840651208, 443.0525900345338));
    let lines = [20, 44];
    assert_eq!(intersection, k);
    assert_eq!(
        i.iter().collect::<Vec<&usize>>().sort(),
        lines.iter().collect::<Vec<&usize>>().sort()
    );
    let (k, i) = iter.next().unwrap();
    let intersection = geo::Coordinate::<f64>::from((100.87671116401667, 475.24445200238074));
    let lines = [7, 20];
    assert_eq!(intersection, k);
    assert_eq!(
        i.iter().collect::<Vec<&usize>>().sort(),
        lines.iter().collect::<Vec<&usize>>().sort()
    );
    Ok(())
}

//#[ignore]
#[test]
fn large_test_2() -> Result<(), intersect2d::IntersectError> {
    let _l: [[f64; 4]; 356] = [
        [395., 20., 402., 20.],
        [402., 20., 408., 23.],
        [469., 26., 476., 27.],
        [328., 26., 322., 28.],
        [328., 26., 335., 29.],
        [476., 27., 481., 33.],
        [322., 28., 318., 33.],
        [257., 47., 264., 49.],
        [540., 47., 548., 50.],
        [257., 47., 250., 51.],
        [548., 50., 552., 56.],
        [395., 20., 370., 57.],
        [408., 23., 429., 57.],
        [335., 29., 362., 58.],
        [469., 26., 438., 58.],
        [370., 57., 362., 58.],
        [429., 57., 438., 58.],
        [481., 33., 495., 69.],
        [318., 33., 305., 69.],
        [264., 49., 295., 71.],
        [540., 47., 504., 71.],
        [495., 69., 504., 71.],
        [305., 69., 295., 71.],
        [191., 82., 198., 82.],
        [607., 81., 613., 85.],
        [191., 82., 185., 87.],
        [393., 87., 404., 87.],
        [393., 87., 382., 93.],
        [613., 85., 616., 92.],
        [404., 87., 415., 94.],
        [552., 56., 558., 94.],
        [250., 51., 241., 94.],
        [241., 94., 233., 98.],
        [558., 94., 566., 98.],
        [198., 82., 233., 98.],
        [607., 81., 566., 98.],
        [382., 93., 377., 103.],
        [415., 94., 420., 104.],
        [616., 92., 615., 107.],
        [377., 103., 377., 115.],
        [420., 104., 420., 115.],
        [615., 107., 615., 120.],
        [377., 115., 382., 125.],
        [420., 115., 414., 125.],
        [451., 124., 471., 128.],
        [348., 124., 318., 131.],
        [140., 128., 133., 129.],
        [660., 128., 667., 130.],
        [185., 87., 185., 131.],
        [615., 120., 614., 131.],
        [382., 125., 392., 131.],
        [414., 125., 403., 131.],
        [392., 131., 403., 131.],
        [667., 130., 670., 134.],
        [133., 129., 128., 134.],
        [471., 128., 510., 141.],
        [185., 131., 177., 136.],
        [614., 131., 622., 136.],
        [318., 131., 289., 142.],
        [140., 128., 177., 136.],
        [660., 128., 622., 136.],
        [670., 134., 671., 140.],
        [289., 142., 262., 155.],
        [510., 141., 545., 160.],
        [348., 124., 384., 162.],
        [262., 155., 237., 172.],
        [451., 124., 405., 165.],
        [384., 162., 395., 167.],
        [405., 165., 395., 167.],
        [545., 160., 577., 183.],
        [128., 134., 136., 177.],
        [671., 140., 663., 177.],
        [237., 172., 213., 191.],
        [663., 177., 668., 185.],
        [136., 177., 131., 185.],
        [92., 183., 131., 185.],
        [707., 183., 668., 185.],
        [707., 183., 713., 186.],
        [92., 183., 85., 186.],
        [713., 186., 717., 191.],
        [85., 186., 82., 191.],
        [577., 183., 606., 210.],
        [717., 191., 717., 198.],
        [82., 191., 82., 198.],
        [213., 191., 192., 211.],
        [192., 211., 490., 211.],
        [490., 211., 501., 213.],
        [606., 210., 630., 242.],
        [717., 198., 701., 233.],
        [82., 198., 98., 233.],
        [501., 213., 529., 222.],
        [701., 233., 705., 241.],
        [98., 233., 94., 241.],
        [630., 242., 641., 259.],
        [94., 241., 56., 248.],
        [705., 241., 743., 248.],
        [56., 248., 50., 250.],
        [743., 248., 749., 250.],
        [50., 250., 47., 257.],
        [749., 250., 752., 258.],
        [47., 257., 49., 264.],
        [752., 258., 750., 264.],
        [556., 238., 584., 271.],
        [132., 291., 119., 293.],
        [666., 292., 677., 294.],
        [132., 291., 142., 295.],
        [750., 264., 728., 295.],
        [49., 264., 71., 295.],
        [666., 292., 654., 296.],
        [728., 295., 728., 296.],
        [172., 296., 211., 296.],
        [584., 271., 590., 305.],
        [211., 296., 211., 474.],
        [641., 259., 622., 304.],
        [172., 296., 173., 299.],
        [337., 298., 337., 350.],
        [337., 298., 432., 298.],
        [432., 298., 435., 298.],
        [119., 293., 111., 301.],
        [677., 294., 686., 302.],
        [71., 295., 69., 301.],
        [69., 301., 69., 302.],
        [142., 295., 150., 305.],
        [654., 296., 647., 305.],
        [69., 302., 69., 305.],
        [728., 296., 730., 305.],
        [435., 298., 457., 308.],
        [111., 301., 106., 311.],
        [686., 302., 690., 312.],
        [150., 305., 152., 316.],
        [647., 305., 646., 317.],
        [69., 305., 33., 318.],
        [730., 305., 767., 318.],
        [622., 304., 613., 322.],
        [106., 311., 108., 323.],
        [457., 308., 465., 325.],
        [690., 312., 688., 324.],
        [33., 318., 27., 323.],
        [767., 318., 772., 324.],
        [173., 299., 184., 326.],
        [152., 316., 147., 326.],
        [646., 317., 649., 328.],
        [772., 324., 773., 329.],
        [27., 323., 26., 330.],
        [590., 305., 582., 337.],
        [108., 323., 115., 332.],
        [613., 322., 613., 333.],
        [688., 324., 681., 333.],
        [147., 326., 138., 334.],
        [649., 328., 659., 335.],
        [773., 329., 770., 335.],
        [184., 326., 181., 337.],
        [115., 332., 126., 336.],
        [138., 334., 126., 336.],
        [681., 333., 670., 337.],
        [465., 325., 458., 340.],
        [659., 335., 670., 337.],
        [613., 333., 618., 342.],
        [181., 337., 174., 344.],
        [174., 344., 123., 367.],
        [458., 340., 438., 348.],
        [438., 348., 428., 350.],
        [337., 350., 428., 350.],
        [26., 330., 58., 362.],
        [770., 335., 742., 362.],
        [618., 342., 675., 369.],
        [742., 362., 742., 369.],
        [582., 337., 548., 374.],
        [675., 369., 676., 369.],
        [742., 369., 742., 370.],
        [58., 362., 57., 370.],
        [548., 374., 533., 383.],
        [123., 367., 121., 387.],
        [57., 370., 23., 391.],
        [742., 370., 776., 391.],
        [533., 383., 550., 397.],
        [776., 391., 780., 396.],
        [23., 391., 20., 397.],
        [550., 397., 558., 405.],
        [780., 396., 780., 402.],
        [20., 397., 20., 404.],
        [558., 405., 562., 410.],
        [780., 402., 776., 408.],
        [562., 410., 565., 416.],
        [676., 369., 677., 418.],
        [565., 416., 569., 422.],
        [121., 387., 123., 423.],
        [647., 418., 677., 418.],
        [647., 418., 645., 420.],
        [337., 425., 411., 425.],
        [337., 425., 337., 476.],
        [411., 425., 417., 426.],
        [776., 408., 742., 429.],
        [20., 404., 57., 429.],
        [569., 422., 574., 438.],
        [417., 426., 434., 434.],
        [645., 420., 644., 442.],
        [742., 429., 742., 438.],
        [57., 429., 58., 438.],
        [123., 423., 126., 449.],
        [434., 434., 450., 456.],
        [450., 456., 453., 463.],
        [644., 442., 637., 463.],
        [574., 438., 580., 462.],
        [58., 438., 29., 464.],
        [742., 438., 770., 464.],
        [126., 449., 132., 474.],
        [580., 462., 588., 470.],
        [770., 464., 773., 471.],
        [29., 464., 26., 471.],
        [637., 463., 623., 473.],
        [132., 474., 211., 474.],
        [588., 470., 605., 475.],
        [623., 473., 605., 475.],
        [337., 476., 406., 476.],
        [773., 471., 771., 477.],
        [26., 471., 28., 478.],
        [406., 476., 407., 478.],
        [771., 477., 766., 481.],
        [766., 481., 730., 495.],
        [28., 478., 69., 495.],
        [730., 495., 728., 504.],
        [69., 495., 71., 504.],
        [453., 463., 464., 515.],
        [464., 515., 467., 527.],
        [71., 504., 49., 535.],
        [728., 504., 752., 540.],
        [49., 535., 47., 541.],
        [47., 541., 50., 548.],
        [752., 540., 749., 548.],
        [467., 527., 485., 552.],
        [50., 548., 56., 552.],
        [749., 548., 743., 552.],
        [743., 552., 705., 558.],
        [56., 552., 94., 558.],
        [407., 478., 407., 560.],
        [485., 552., 502., 562.],
        [407., 560., 407., 561.],
        [502., 562., 506., 562.],
        [407., 561., 405., 562.],
        [621., 562., 624., 562.],
        [175., 562., 405., 562.],
        [506., 562., 621., 562.],
        [94., 558., 98., 566.],
        [705., 558., 701., 566.],
        [255., 581., 200., 590.],
        [548., 581., 537., 583.],
        [701., 566., 711., 588.],
        [255., 581., 265., 585.],
        [624., 562., 600., 590.],
        [537., 583., 529., 590.],
        [175., 562., 200., 590.],
        [600., 590., 598., 592.],
        [265., 585., 270., 593.],
        [548., 581., 598., 592.],
        [98., 566., 82., 601.],
        [711., 588., 717., 601.],
        [717., 601., 717., 609.],
        [82., 601., 82., 609.],
        [227., 609., 238., 610.],
        [571., 610., 558., 611.],
        [717., 609., 714., 613.],
        [82., 609., 86., 613.],
        [227., 609., 216., 615.],
        [571., 610., 581., 616.],
        [238., 610., 248., 617.],
        [86., 613., 92., 616.],
        [714., 613., 707., 616.],
        [131., 614., 92., 616.],
        [668., 614., 707., 616.],
        [558., 611., 549., 618.],
        [668., 614., 666., 617.],
        [131., 614., 136., 622.],
        [666., 617., 663., 622.],
        [216., 615., 210., 625.],
        [581., 616., 587., 626.],
        [248., 617., 253., 628.],
        [549., 618., 544., 628.],
        [210., 625., 209., 637.],
        [587., 626., 588., 638.],
        [270., 593., 281., 641.],
        [253., 628., 252., 639.],
        [544., 628., 544., 640.],
        [209., 637., 214., 647.],
        [588., 638., 582., 648.],
        [252., 639., 247., 648.],
        [544., 640., 551., 649.],
        [281., 641., 283., 649.],
        [283., 649., 291., 653.],
        [529., 590., 514., 650.],
        [214., 647., 224., 653.],
        [247., 648., 236., 654.],
        [582., 648., 572., 654.],
        [551., 649., 561., 655.],
        [224., 653., 236., 654.],
        [572., 654., 561., 655.],
        [136., 622., 128., 659.],
        [514., 650., 478., 664.],
        [291., 653., 329., 666.],
        [663., 622., 671., 665.],
        [128., 659., 129., 666.],
        [622., 663., 614., 668.],
        [177., 663., 185., 668.],
        [671., 665., 666., 670.],
        [129., 666., 134., 671.],
        [478., 664., 439., 672.],
        [622., 663., 659., 671.],
        [666., 670., 659., 671.],
        [177., 663., 134., 671.],
        [329., 666., 368., 673.],
        [439., 672., 419., 674.],
        [368., 673., 389., 675.],
        [419., 674., 389., 675.],
        [566., 701., 558., 705.],
        [233., 701., 241., 705.],
        [185., 668., 183., 707.],
        [614., 668., 614., 712.],
        [183., 707., 186., 713.],
        [233., 701., 198., 717.],
        [566., 701., 601., 717.],
        [614., 712., 609., 717.],
        [186., 713., 191., 718.],
        [198., 717., 191., 718.],
        [601., 717., 609., 717.],
        [504., 728., 495., 730.],
        [295., 728., 305., 730.],
        [241., 705., 248., 743.],
        [558., 705., 552., 743.],
        [362., 742., 370., 742.],
        [429., 742., 438., 742.],
        [552., 743., 549., 749.],
        [248., 743., 251., 749.],
        [504., 728., 535., 750.],
        [549., 749., 542., 752.],
        [251., 749., 259., 752.],
        [295., 728., 259., 752.],
        [535., 750., 542., 752.],
        [305., 730., 318., 766.],
        [495., 730., 481., 766.],
        [438., 742., 464., 770.],
        [362., 742., 334., 771.],
        [318., 766., 322., 771.],
        [481., 766., 476., 772.],
        [464., 770., 470., 773.],
        [322., 771., 327., 773.],
        [334., 771., 327., 773.],
        [476., 772., 470., 773.],
        [370., 742., 391., 776.],
        [429., 742., 404., 779.],
        [391., 776., 397., 780.],
        [404., 779., 397., 780.],
        [529., 222., 556., 238.],
        [134., 520., 423., 706.],
        [735., 105., 423., 706.],
        [735., 105., 415., 586.],
        [134., 520., 415., 586.],
    ];
    let mut iter = AlgorithmData::<f64>::default()
        .with_ignore_end_point_intersections(true)?
        .with_ref_lines(to_lines(&_l).iter())?
        .compute()?;

    let (k, i) = iter.next().unwrap();
    let intersection = geo::Coordinate::<f64>::from((694.1658119658119, 183.6581634889327));
    let lines = [76, 353];
    assert_eq!(intersection, k);
    assert_eq!(
        i.iter().collect::<Vec<&usize>>().sort(),
        lines.iter().collect::<Vec<&usize>>().sort()
    );
    let (k, i) = iter.next().unwrap();
    let intersection = geo::Coordinate::<f64>::from((682.2641978034108, 184.26850267674814));
    let lines = [76, 354];
    assert_eq!(intersection, k);
    assert_eq!(
        i.iter().collect::<Vec<&usize>>().sort(),
        lines.iter().collect::<Vec<&usize>>().sort()
    );
    let (k, i) = iter.next().unwrap();
    let intersection = geo::Coordinate::<f64>::from((636.8320752958717, 252.55866182089275));
    let lines = [93, 354];
    assert_eq!(intersection, k);
    assert_eq!(
        i.iter().collect::<Vec<&usize>>().sort(),
        lines.iter().collect::<Vec<&usize>>().sort()
    );
    let (k, i) = iter.next().unwrap();
    let intersection = geo::Coordinate::<f64>::from((582.8097622027534, 333.76095118898627));
    let lines = [144, 354];
    assert_eq!(intersection, k);
    assert_eq!(
        i.iter().collect::<Vec<&usize>>().sort(),
        lines.iter().collect::<Vec<&usize>>().sort()
    );
    let (k, i) = iter.next().unwrap();
    let intersection = geo::Coordinate::<f64>::from((614.8802683640117, 336.38448305522104));
    let lines = [157, 353];
    assert_eq!(intersection, k);
    assert_eq!(
        i.iter().collect::<Vec<&usize>>().sort(),
        lines.iter().collect::<Vec<&usize>>().sort()
    );
    let (k, i) = iter.next().unwrap();
    let intersection = geo::Coordinate::<f64>::from((577.1267168808154, 342.3032786885244));
    let lines = [167, 354];
    assert_eq!(intersection, k);
    assert_eq!(
        i.iter().collect::<Vec<&usize>>().sort(),
        lines.iter().collect::<Vec<&usize>>().sort()
    );
    let (k, i) = iter.next().unwrap();
    let intersection = geo::Coordinate::<f64>::from((544.0163545863949, 392.07229201232525));
    let lines = [175, 354];
    assert_eq!(intersection, k);
    assert_eq!(
        i.iter().collect::<Vec<&usize>>().sort(),
        lines.iter().collect::<Vec<&usize>>().sort()
    );
    let (k, i) = iter.next().unwrap();
    let intersection = geo::Coordinate::<f64>::from((569.5389521070401, 423.7246467425284));
    let lines = [194, 353];
    assert_eq!(intersection, k);
    assert_eq!(
        i.iter().collect::<Vec<&usize>>().sort(),
        lines.iter().collect::<Vec<&usize>>().sort()
    );
    let (k, i) = iter.next().unwrap();
    let intersection = geo::Coordinate::<f64>::from((463.5741644247869, 512.9869590989922));
    let lines = [223, 354];
    assert_eq!(intersection, k);
    assert_eq!(
        i.iter().collect::<Vec<&usize>>().sort(),
        lines.iter().collect::<Vec<&usize>>().sort()
    );
    let (k, i) = iter.next().unwrap();
    let intersection = geo::Coordinate::<f64>::from((498.7483691984704, 560.0872759991003));
    let lines = [236, 353];
    assert_eq!(intersection, k);
    assert_eq!(
        i.iter().collect::<Vec<&usize>>().sort(),
        lines.iter().collect::<Vec<&usize>>().sort()
    );
    let (k, i) = iter.next().unwrap();
    let intersection = geo::Coordinate::<f64>::from((199.25806451612902, 562.));
    let lines = [241, 352];
    assert_eq!(intersection, k);
    assert_eq!(
        i.iter().collect::<Vec<&usize>>().sort(),
        lines.iter().collect::<Vec<&usize>>().sort()
    );
    let (k, i) = iter.next().unwrap();
    let intersection = geo::Coordinate::<f64>::from((312.8181818181818, 562.));
    let lines = [241, 355];
    assert_eq!(intersection, k);
    assert_eq!(
        i.iter().collect::<Vec<&usize>>().sort(),
        lines.iter().collect::<Vec<&usize>>().sort()
    );
    let (k, i) = iter.next().unwrap();
    let intersection = geo::Coordinate::<f64>::from((234.0947704777492, 584.4208557400046));
    let lines = [245, 352];
    assert_eq!(intersection, k);
    assert_eq!(
        i.iter().collect::<Vec<&usize>>().sort(),
        lines.iter().collect::<Vec<&usize>>().sort()
    );
    let (k, i) = iter.next().unwrap();
    let intersection = geo::Coordinate::<f64>::from((273.9057162185016, 610.043125317098));
    let lines = [280, 352];
    assert_eq!(intersection, k);
    assert_eq!(
        i.iter().collect::<Vec<&usize>>().sort(),
        lines.iter().collect::<Vec<&usize>>().sort()
    );
    let (k, i) = iter.next().unwrap();
    let intersection = geo::Coordinate::<f64>::from((440.84729981378024, 671.6210667048657));
    let lines = [305, 353];
    assert_eq!(intersection, k);
    assert_eq!(
        i.iter().collect::<Vec<&usize>>().sort(),
        lines.iter().collect::<Vec<&usize>>().sort()
    );
    let (k, i) = iter.next().unwrap();
    let intersection = geo::Coordinate::<f64>::from((372.37289663461536, 673.4164663461538));
    let lines = [311, 352];
    assert_eq!(intersection, k);
    assert_eq!(
        i.iter().collect::<Vec<&usize>>().sort(),
        lines.iter().collect::<Vec<&usize>>().sort()
    );
    Ok(())
}
