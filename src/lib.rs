use std::collections::HashMap;
use std::vec::Vec;

pub fn get_path(word: &str, no_points: usize) -> Vec<(f64, f64)> {
    println!("Original word '{}':", word);

    // Remove duplicate chars
    let mut word: Vec<char> = word.chars().collect();
    word.dedup();
    let word: String = word.into_iter().collect();

    // Get button centers
    let button_coordinates = get_default_buttons_centers();

    // Get waypoints
    let ideal_path = ideal_waypoints(&word, button_coordinates);
    // Interpolate the path
    ideal_path_interpolated(no_points, ideal_path)
}

// Generates a path by connecting the centers of the keys of the word with straight lines. Only the waypoints are returned, nothing is interpolated
pub fn ideal_waypoints(
    word: &str,
    button_coordinates: HashMap<String, (f64, f64)>,
) -> Vec<(f64, f64)> {
    let mut points = Vec::new();
    for letter in word.chars() {
        let letter_coordinate = button_coordinates.get(&letter.to_string()).unwrap();
        points.push(*letter_coordinate);
    }
    points
}
fn dist(start: &(f64, f64), end: &(f64, f64)) -> f64 {
    f64::sqrt((start.0 - end.0).powi(2) + (start.1 - end.1).powi(2))
}

pub fn ideal_path_interpolated(no_points: usize, waypoints: Vec<(f64, f64)>) -> Vec<(f64, f64)> {
    if no_points < waypoints.len() {
        panic!("There are more waypoints than no_points!");
    }

    let mut path: Vec<(f64, f64)> = Vec::new();
    let mut total_dist = 0.0;

    // Calculate the total distance of the path
    let mut waypoints_iter = waypoints.iter().peekable();
    while let Some(start_point) = waypoints_iter.next() {
        if let Some(end_point) = waypoints_iter.peek() {
            total_dist += dist(start_point, *end_point);
        }
    }
    if (total_dist - 0.0).abs() < 0.00000001 {
        panic!("The waypoints must not be the same");
    }

    // Interpolate the points of the path
    let mut leg_dist;
    let mut delta_x;
    let mut delta_y;

    let mut waypoints_iter = waypoints.iter().peekable();

    // Add the first waypoint
    path.push(waypoints[0]);

    // While there are more waypoints, interpolate points for them
    while let Some(start_point) = waypoints_iter.next() {
        if let Some(end_point) = waypoints_iter.peek() {
            delta_x = end_point.0 - start_point.0;
            delta_y = end_point.1 - start_point.1;
            // Skip legs when their start end end points are equal
            if (delta_x).abs() < 0.00000001 && (delta_y).abs() < 0.00000001 {
                continue;
            }
            leg_dist = dist(start_point, end_point);
            let no_leg_points = (leg_dist / total_dist) * no_points as f64;
            let no_leg_points = no_leg_points.trunc();

            for i in 0..no_leg_points as isize {
                path.push((
                    delta_x * (i as f64 / no_leg_points) + start_point.0,
                    delta_y * (i as f64 / no_leg_points) + start_point.1,
                ));
            }
            path.push(**end_point);
        }
    }

    path
}

// Gets the centers of all default buttons
pub fn get_default_buttons_centers() -> HashMap<String, (f64, f64)> {
    let mut buttons = HashMap::new();
    buttons.insert("u".to_string(), (0.65, 0.125));
    buttons.insert("i".to_string(), (0.75, 0.125));
    buttons.insert("k".to_string(), (0.8, 0.375));
    buttons.insert("e".to_string(), (0.25, 0.125));
    buttons.insert("l".to_string(), (0.9, 0.375));
    buttons.insert("t".to_string(), (0.45, 0.125));
    buttons.insert("x".to_string(), (0.25, 0.625));
    buttons.insert(".".to_string(), (0.75, 0.875));
    buttons.insert("w".to_string(), (0.15, 0.125));
    buttons.insert("d".to_string(), (0.3, 0.375));
    buttons.insert("h".to_string(), (0.6, 0.375));
    buttons.insert("space".to_string(), (0.5, 0.875));
    buttons.insert("v".to_string(), (0.45, 0.625));
    buttons.insert("r".to_string(), (0.35, 0.125));
    buttons.insert("Shift_L_base".to_string(), (0.05, 0.625));
    buttons.insert("q".to_string(), (0.05, 0.125));
    buttons.insert("a".to_string(), (0.1, 0.375));
    buttons.insert("o".to_string(), (0.85, 0.125));
    buttons.insert("c".to_string(), (0.35, 0.625));
    buttons.insert("m".to_string(), (0.75, 0.625));
    buttons.insert("p".to_string(), (0.95, 0.125));
    buttons.insert("n".to_string(), (0.65, 0.625));
    buttons.insert("z".to_string(), (0.15, 0.625));
    buttons.insert(":)".to_string(), (0.25, 0.875));
    buttons.insert("b".to_string(), (0.55, 0.625));
    buttons.insert("Return".to_string(), (0.9, 0.875));
    buttons.insert("y".to_string(), (0.55, 0.125));
    buttons.insert("BackSpace".to_string(), (0.9, 0.625));
    buttons.insert("show_numbers".to_string(), (0.05, 0.875));
    buttons.insert("GBA".to_string(), (0.15, 0.875));
    buttons.insert("s".to_string(), (0.2, 0.375));
    buttons.insert("g".to_string(), (0.5, 0.375));
    buttons.insert("f".to_string(), (0.4, 0.375));
    buttons.insert("j".to_string(), (0.7, 0.375));
    buttons
}
