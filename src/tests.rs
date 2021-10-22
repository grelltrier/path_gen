use super::*;

fn float_cmp(a: f64, b: f64) -> bool {
    (a - b).abs() < 0.000000005
}

#[test]
// Test case B1
fn test_empty_word() {
    let word = "";
    let key_layout = get_default_buttons_centers();
    let word_path = WordPath::new(&key_layout, word);

    // Check the interpolated path
    assert!(word_path.get_path(0.1).is_none());
}

#[test]
// Test case B2
fn test_negative_point_density() {
    let word = "I";
    let key_layout = get_default_buttons_centers();
    let word_path = WordPath::new(&key_layout, word);

    // Check the interpolated path
    assert!(word_path.get_path(-0.1).is_none());
}

#[test]
// Test case B3
fn test_no_path_possible() {
    let word = "ÜÜÜÜÜÜ";
    let key_layout = get_default_buttons_centers();
    let word_path = WordPath::new(&key_layout, word);

    // Check first and last points
    let (first_word, last_word) = word_path.get_first_last_points();
    assert!(first_word.is_none() && last_word.is_none());

    // Check the interpolated path
    assert!(word_path.get_path(-0.1).is_none());
}

#[test]
// Test case B4
fn test_no_path_possible_but_first_last_valid() {
    let word = "hÜÜÜÜo";
    let key_layout = get_default_buttons_centers();
    let word_path = WordPath::new(&key_layout, word);

    // Check first and last points
    if let (Some(&(first_x, first_y)), Some(&(last_x, last_y))) = word_path.get_first_last_points()
    {
        assert!(float_cmp(first_x, 0.600) && float_cmp(first_y, 0.15));
        assert!(float_cmp(last_x, 0.850) && float_cmp(last_y, 0.05));
    } else {
        panic!()
    }

    // Check the interpolated path
    assert!(word_path.get_path(-0.1).is_none());
}

#[test]
// Test case B5
fn test_word_i() {
    let word = "I";
    println!("Path for '{}':", word);

    let correct_path = vec![(0.750, 0.05)];

    let key_layout = get_default_buttons_centers();
    let word_path = WordPath::new(&key_layout, word);

    // Check the first and last points
    if let (Some(&(first_x, first_y)), Some(&(last_x, last_y))) = word_path.get_first_last_points()
    {
        assert!(float_cmp(first_x, 0.750) && float_cmp(first_y, 0.05));
        assert!(float_cmp(last_x, 0.750) && float_cmp(last_y, 0.05));
    } else {
        panic!()
    }

    // Check the interpolated path
    let ideal_path = word_path.get_path(0.1);
    if let Some(ideal_path) = ideal_path {
        assert!(ideal_path.len() == correct_path.len());
        for (idx, &(point_x, point_y)) in ideal_path.iter().enumerate() {
            assert!(
                float_cmp(point_x, correct_path[idx].0) && float_cmp(point_y, correct_path[idx].1)
            );
        }
    } else {
        panic!();
    }
}

#[test]
// Test case B6
fn test_word_hello() {
    let word = "hello";
    println!("Path for '{}':", word);

    let correct_path = vec![
        (0.6, 0.15),
        (0.48333333333333334, 0.11666666666666667),
        (0.3666666666666667, 0.08333333333333333),
        (0.25, 0.05),
        (0.34285714285714286, 0.0642857142857143),
        (0.4357142857142857, 0.07857142857142857),
        (0.5285714285714286, 0.09285714285714286),
        (0.6214285714285714, 0.10714285714285715),
        (0.7142857142857143, 0.12142857142857143),
        (0.8071428571428572, 0.13571428571428573),
        (0.9, 0.15),
        (0.85, 0.05),
    ];

    let key_layout = get_default_buttons_centers();
    let word_path = WordPath::new(&key_layout, word);

    // Check the first and last points
    if let (Some(&(first_x, first_y)), Some(&(last_x, last_y))) = word_path.get_first_last_points()
    {
        assert!(float_cmp(first_x, 0.600) && float_cmp(first_y, 0.15));
        assert!(float_cmp(last_x, 0.850) && float_cmp(last_y, 0.05));
    } else {
        panic!()
    }

    // Check the interpolated path
    let ideal_path = word_path.get_path(0.1);
    if let Some(ideal_path) = ideal_path {
        assert!(ideal_path.len() == correct_path.len());
        for (idx, &(point_x, point_y)) in ideal_path.iter().enumerate() {
            println!("({},{}),", point_x, point_y);
            assert!(
                float_cmp(point_x, correct_path[idx].0) && float_cmp(point_y, correct_path[idx].1)
            );
        }
    } else {
        panic!();
    }
}

#[test]
// Test case B7
fn test_lowercasing() {
    let key_layout = get_default_buttons_centers();

    let word = "hello";
    let word_a = "HeLLo";
    let word_b = "HELLO";
    let word_c = "hELlO";

    let word_path = WordPath::new(&key_layout, word);
    let word_path_a = WordPath::new(&key_layout, word_a);
    let word_path_b = WordPath::new(&key_layout, word_b);
    let word_path_c = WordPath::new(&key_layout, word_c);
    let word_paths_cased = vec![word_path_a, word_path_b, word_path_c];

    // Check the first and last points
    let (first_point, last_point) = word_path.get_first_last_points();
    for cased_word in &word_paths_cased {
        let (first_point_cased, last_point_cased) = cased_word.get_first_last_points();
        assert!(first_point == first_point_cased && last_point == last_point_cased);
    }

    // Check the interpolated path
    let ideal_path = word_path.get_path(0.1);
    for cased_word in &word_paths_cased {
        let path_cased = cased_word.get_path(0.1);
        assert!(ideal_path == path_cased);
    }
}

#[test]
// Test case B8
fn test_word_hello_dense() {
    let word = "hello";
    println!("Path for '{}':", word);

    let correct_path = vec![
        (0.6, 0.15),
        (0.5902777777777778, 0.14722222222222223),
        (0.5805555555555555, 0.14444444444444443),
        (0.5708333333333333, 0.14166666666666666),
        (0.5611111111111111, 0.1388888888888889),
        (0.5513888888888888, 0.1361111111111111),
        (0.5416666666666666, 0.13333333333333333),
        (0.5319444444444444, 0.13055555555555556),
        (0.5222222222222221, 0.12777777777777777),
        (0.5125, 0.125),
        (0.5027777777777778, 0.12222222222222222),
        (0.4930555555555555, 0.11944444444444444),
        (0.4833333333333333, 0.11666666666666667),
        (0.4736111111111111, 0.11388888888888889),
        (0.46388888888888885, 0.1111111111111111),
        (0.4541666666666666, 0.10833333333333334),
        (0.4444444444444444, 0.10555555555555556),
        (0.43472222222222223, 0.10277777777777777),
        (0.425, 0.1),
        (0.41527777777777775, 0.09722222222222222),
        (0.40555555555555556, 0.09444444444444444),
        (0.3958333333333333, 0.09166666666666667),
        (0.38611111111111107, 0.08888888888888889),
        (0.3763888888888889, 0.08611111111111111),
        (0.36666666666666664, 0.08333333333333334),
        (0.3569444444444444, 0.08055555555555556),
        (0.3472222222222222, 0.07777777777777778),
        (0.33749999999999997, 0.075),
        (0.3277777777777777, 0.07222222222222223),
        (0.31805555555555554, 0.06944444444444445),
        (0.3083333333333333, 0.06666666666666667),
        (0.2986111111111111, 0.0638888888888889),
        (0.28888888888888886, 0.061111111111111116),
        (0.2791666666666666, 0.058333333333333334),
        (0.26944444444444443, 0.055555555555555566),
        (0.2597222222222222, 0.052777777777777785),
        (0.25, 0.05),
        (0.25984848484848483, 0.05151515151515152),
        (0.2696969696969697, 0.05303030303030303),
        (0.27954545454545454, 0.05454545454545455),
        (0.2893939393939394, 0.05606060606060606),
        (0.29924242424242425, 0.05757575757575758),
        (0.3090909090909091, 0.0590909090909091),
        (0.31893939393939397, 0.06060606060606061),
        (0.3287878787878788, 0.06212121212121212),
        (0.3386363636363636, 0.06363636363636363),
        (0.3484848484848485, 0.06515151515151515),
        (0.35833333333333334, 0.06666666666666667),
        (0.36818181818181817, 0.06818181818181818),
        (0.37803030303030305, 0.0696969696969697),
        (0.3878787878787879, 0.07121212121212121),
        (0.3977272727272727, 0.07272727272727272),
        (0.4075757575757576, 0.07424242424242425),
        (0.4174242424242425, 0.07575757575757576),
        (0.4272727272727273, 0.07727272727272727),
        (0.43712121212121213, 0.07878787878787878),
        (0.446969696969697, 0.08030303030303029),
        (0.45681818181818185, 0.08181818181818182),
        (0.4666666666666667, 0.08333333333333333),
        (0.47651515151515156, 0.08484848484848484),
        (0.4863636363636364, 0.08636363636363636),
        (0.4962121212121212, 0.08787878787878788),
        (0.5060606060606061, 0.08939393939393939),
        (0.5159090909090909, 0.09090909090909091),
        (0.5257575757575758, 0.09242424242424242),
        (0.5356060606060606, 0.09393939393939393),
        (0.5454545454545454, 0.09545454545454546),
        (0.5553030303030304, 0.09696969696969696),
        (0.5651515151515152, 0.09848484848484848),
        (0.575, 0.1),
        (0.584848484848485, 0.1015151515151515),
        (0.5946969696969697, 0.10303030303030303),
        (0.6045454545454546, 0.10454545454545454),
        (0.6143939393939395, 0.10606060606060605),
        (0.6242424242424243, 0.10757575757575757),
        (0.6340909090909091, 0.10909090909090909),
        (0.643939393939394, 0.1106060606060606),
        (0.6537878787878788, 0.11212121212121212),
        (0.6636363636363637, 0.11363636363636363),
        (0.6734848484848486, 0.11515151515151514),
        (0.6833333333333333, 0.11666666666666665),
        (0.6931818181818182, 0.11818181818181818),
        (0.7030303030303031, 0.11969696969696969),
        (0.7128787878787879, 0.1212121212121212),
        (0.7227272727272728, 0.12272727272727273),
        (0.7325757575757577, 0.12424242424242424),
        (0.7424242424242424, 0.12575757575757573),
        (0.7522727272727273, 0.12727272727272726),
        (0.7621212121212122, 0.12878787878787878),
        (0.771969696969697, 0.1303030303030303),
        (0.7818181818181819, 0.1318181818181818),
        (0.7916666666666667, 0.1333333333333333),
        (0.8015151515151515, 0.13484848484848483),
        (0.8113636363636364, 0.13636363636363635),
        (0.8212121212121213, 0.13787878787878788),
        (0.8310606060606062, 0.1393939393939394),
        (0.8409090909090909, 0.1409090909090909),
        (0.8507575757575758, 0.1424242424242424),
        (0.8606060606060607, 0.14393939393939392),
        (0.8704545454545455, 0.14545454545454545),
        (0.8803030303030304, 0.14696969696969697),
        (0.8901515151515152, 0.14848484848484847),
        (0.9, 0.15),
        (0.8954545454545455, 0.1409090909090909),
        (0.8909090909090909, 0.1318181818181818),
        (0.8863636363636364, 0.12272727272727273),
        (0.8818181818181818, 0.11363636363636363),
        (0.8772727272727273, 0.10454545454545454),
        (0.8727272727272727, 0.09545454545454546),
        (0.8681818181818182, 0.08636363636363636),
        (0.8636363636363636, 0.07727272727272727),
        (0.8590909090909091, 0.06818181818181818),
        (0.8545454545454545, 0.05909090909090908),
        (0.85, 0.05),
    ];

    let key_layout = get_default_buttons_centers();
    let word_path = WordPath::new(&key_layout, word);

    // Check the first and last points
    if let (Some(&(first_x, first_y)), Some(&(last_x, last_y))) = word_path.get_first_last_points()
    {
        assert!(float_cmp(first_x, 0.600) && float_cmp(first_y, 0.15));
        assert!(float_cmp(last_x, 0.850) && float_cmp(last_y, 0.05));
    } else {
        panic!()
    }

    // Check the interpolated path
    let ideal_path = word_path.get_path(0.01);
    if let Some(ideal_path) = ideal_path {
        assert!(ideal_path.len() == correct_path.len());
        for (idx, &(point_x, point_y)) in ideal_path.iter().enumerate() {
            println!("({},{}),", point_x, point_y);
            assert!(
                float_cmp(point_x, correct_path[idx].0) && float_cmp(point_y, correct_path[idx].1)
            );
        }
    } else {
        panic!();
    }
}

#[test]
// Test case B9
fn test_word_spaceship() {
    let word = "spaceship";
    println!("Path for '{}':", word);

    let correct_path = vec![
        (0.2, 0.15),
        (0.30714285714285716, 0.1357142857142857),
        (0.41428571428571426, 0.12142857142857143),
        (0.5214285714285714, 0.10714285714285714),
        (0.6285714285714286, 0.09285714285714286),
        (0.7357142857142858, 0.07857142857142857),
        (0.8428571428571427, 0.06428571428571428),
        (0.95, 0.05),
        (0.8555555555555555, 0.061111111111111116),
        (0.7611111111111111, 0.07222222222222222),
        (0.6666666666666666, 0.08333333333333333),
        (0.5722222222222222, 0.09444444444444444),
        (0.47777777777777775, 0.10555555555555556),
        (0.3833333333333333, 0.11666666666666665),
        (0.28888888888888886, 0.12777777777777777),
        (0.19444444444444442, 0.1388888888888889),
        (0.1, 0.15),
        (0.22499999999999998, 0.2),
        (0.35, 0.25),
        (0.31666666666666665, 0.18333333333333335),
        (0.2833333333333333, 0.11666666666666667),
        (0.25, 0.05),
        (0.2, 0.15),
        (0.3, 0.15),
        (0.4, 0.15),
        (0.5, 0.15),
        (0.6, 0.15),
        (0.75, 0.05),
        (0.85, 0.05),
        (0.95, 0.05),
    ];

    let key_layout = get_default_buttons_centers();
    let word_path = WordPath::new(&key_layout, word);

    // Check the first and last points
    if let (Some(&(first_x, first_y)), Some(&(last_x, last_y))) = word_path.get_first_last_points()
    {
        assert!(float_cmp(first_x, 0.200) && float_cmp(first_y, 0.15));
        assert!(float_cmp(last_x, 0.950) && float_cmp(last_y, 0.05));
    } else {
        panic!()
    }

    // Check the interpolated path
    let ideal_path = word_path.get_path(0.1);
    if let Some(ideal_path) = ideal_path {
        //assert!(ideal_path.len() == correct_path.len());
        for (idx, &(point_x, point_y)) in ideal_path.iter().enumerate() {
            println!("({},{}),", point_x, point_y);
            assert!(
                float_cmp(point_x, correct_path[idx].0) && float_cmp(point_y, correct_path[idx].1)
            );
        }
    } else {
        panic!();
    }
}
