use std::collections::HashMap;
use std::vec::Vec;

pub struct WordPath<'a> {
    word: Vec<char>,
    key_layout: &'a HashMap<String, (f64, f64)>,
}

impl<'a> WordPath<'a> {
    pub fn new(key_layout: &'a HashMap<String, (f64, f64)>, word: &str) -> Self {
        let mut word: Vec<char> = word.chars().collect();
        word.dedup();
        // Ignore the case
        let word: String = word.into_iter().collect::<String>().to_ascii_lowercase();
        let word: Vec<char> = word.chars().collect();
        Self { word, key_layout }
    }

    pub fn get_first_last_points(&self) -> (Option<&(f64, f64)>, Option<&(f64, f64)>) {
        let coordinate_first_key = if self.word.len() > 0 {
            let first_char = self.word[0];
            self.key_layout.get(&first_char.to_string())
        } else {
            None
        };

        let coordinate_last_key = if self.word.len() > 1 {
            let last_char = self.word[self.word.len() - 1];
            self.key_layout.get(&last_char.to_string())
        } else {
            None
        };

        (coordinate_first_key, coordinate_last_key)
    }

    // Generates a path by connecting the centers of the keys of the word with straight lines. Only the waypoints are returned, nothing is interpolated
    // Waypoints that are the same are merged into one. This happens when multiple characters are sharing the same key
    // The resulting waypoints and the total length of the path is returned
    fn ideal_waypoints(&self) -> Option<(Vec<(f64, f64)>, f64)> {
        let mut points = Vec::new();
        let mut prev_point = None;
        let mut total_length = 0.0;
        let mut leg_length;
        for letter in &self.word {
            if let Some(&(x, y)) = self.key_layout.get(&letter.to_string()) {
                if let Some(prev_point) = prev_point {
                    leg_length = dist(&(x, y), &prev_point);
                    if leg_length < 0.0000001 {
                        continue;
                    }
                    total_length += leg_length
                }
                prev_point = Some((x, y));
                points.push((x, y));
            } else {
                //println!("No key found for letter {}, NO PATH POSSIBLE", letter);
                return None;
            }
        }
        Some((points, total_length))
    }

    pub fn ideal_path_interpolated(
        &self,
        waypoints: Vec<(f64, f64)>,
        desired_point_density: f64,
    ) -> Option<Vec<(f64, f64)>> {
        // If there are no waypoints, we can't construct a path
        // This can only happen for an empty string (which should never occur)
        // If the requested desired_point_density is negative or zero, we also can not construct a valid path
        if waypoints.len() == 0 || desired_point_density <= 0.0 {
            return None;
        }

        // If there is only one waypoint, we can also not construct intermediate points
        if waypoints.len() == 1 {
            return Some(vec![waypoints[0]; 1]);
        }

        // Interpolate the points of the path
        let mut leg_dist;
        let mut delta_x;
        let mut delta_y;

        //let point_dist = path_length / (no_points - 1) as f64;
        let point_dist = desired_point_density;
        let mut remainder = 0.0;

        let mut path: Vec<(f64, f64)> = Vec::new();
        let mut waypoints_iter = waypoints.iter().peekable();
        let mut no_leg_sections;
        // While there are more waypoints, interpolate points for them
        while let Some(start_point) = waypoints_iter.next() {
            path.push(*start_point);
            if let Some(&end_point) = waypoints_iter.peek() {
                leg_dist = dist(start_point, end_point);
                no_leg_sections = leg_dist / point_dist + remainder;
                remainder = no_leg_sections.fract();
                no_leg_sections = no_leg_sections.trunc();

                // Calculate the delta and divide it by the number of points
                // This can also be interpreted as the slope of the linear function connecting the start and the end point
                delta_x = (end_point.0 - start_point.0) / no_leg_sections;
                delta_y = (end_point.1 - start_point.1) / no_leg_sections;

                for i in 1..no_leg_sections as isize {
                    path.push((
                        delta_x * i as f64 + start_point.0,
                        delta_y * i as f64 + start_point.1,
                    ));
                }
            }
        }
        Some(path)
    }

    pub fn get_path(&self, desired_point_density: f64) -> Option<Vec<(f64, f64)>> {
        // Get waypoints
        let ideal_path = self.ideal_waypoints();
        // Interpolate the path
        if let Some((ideal_path, _)) = ideal_path {
            self.ideal_path_interpolated(ideal_path, desired_point_density)
        } else {
            None
        }
    }
}

pub fn get_button_centers() -> HashMap<std::string::String, (f64, f64)> {
    let buttons_raw = get_buttons();
    // Get sum up the coordinates of the buttons
    let mut buttons_coordinates = HashMap::new();
    for (button_id, x, y) in buttons_raw.into_iter() {
        let button_entry = buttons_coordinates
            .entry(button_id)
            .or_insert((0.0, 0.0, 0));
        let (button_x, button_y, button_count) = button_entry;
        *button_x += x;
        *button_y += y;
        *button_count += 1;
    }

    // Divide the coordinates by the number of occurences of the buttons to get their center.
    let mut buttons_coordinates_normalized = HashMap::new();
    for (button_id, (button_x, button_y, button_count)) in buttons_coordinates.into_iter() {
        let button_x = button_x / button_count as f64;
        let button_y = button_y / button_count as f64;
        buttons_coordinates_normalized.insert(button_id.to_string(), (button_x, button_y));
    }
    for (button_name, (x, y)) in buttons_coordinates_normalized.iter() {
        //buttons.push((".".to_string(), 0.775, 0.875));
        println!(
            "buttons.push((\"{}\".to_string(), {:.3}, {:.3}));",
            button_name, x, y,
        );
    }

    buttons_coordinates_normalized
}

fn dist(start: &(f64, f64), end: &(f64, f64)) -> f64 {
    f64::sqrt((start.0 - end.0).powi(2) + (start.1 - end.1).powi(2))
}

pub fn get_default_buttons_centers() -> HashMap<String, (f64, f64)> {
    let mut buttons = HashMap::new();
    buttons.insert("a".to_string(), (0.100, 0.15));
    buttons.insert("b".to_string(), (0.550, 0.25));
    buttons.insert("c".to_string(), (0.350, 0.25));
    buttons.insert("d".to_string(), (0.300, 0.15));
    buttons.insert("e".to_string(), (0.250, 0.05));
    buttons.insert("f".to_string(), (0.400, 0.15));
    buttons.insert("g".to_string(), (0.500, 0.15));
    buttons.insert("h".to_string(), (0.600, 0.15));
    buttons.insert("i".to_string(), (0.750, 0.05));
    buttons.insert("j".to_string(), (0.700, 0.15));
    buttons.insert("k".to_string(), (0.800, 0.15));
    buttons.insert("l".to_string(), (0.900, 0.15));
    buttons.insert("m".to_string(), (0.750, 0.25));
    buttons.insert("n".to_string(), (0.650, 0.25));
    buttons.insert("o".to_string(), (0.850, 0.05));
    buttons.insert("p".to_string(), (0.950, 0.05));
    buttons.insert("q".to_string(), (0.050, 0.05));
    buttons.insert("r".to_string(), (0.350, 0.05));
    buttons.insert("s".to_string(), (0.200, 0.15));
    buttons.insert("t".to_string(), (0.450, 0.05));
    buttons.insert("u".to_string(), (0.650, 0.05));
    buttons.insert("v".to_string(), (0.450, 0.25));
    buttons.insert("w".to_string(), (0.150, 0.05));
    buttons.insert("x".to_string(), (0.250, 0.25));
    buttons.insert("y".to_string(), (0.550, 0.05));
    buttons.insert("z".to_string(), (0.150, 0.25));
    buttons
}

fn get_buttons() -> Vec<(String, f64, f64)> {
    vec![
        ("m".to_string(), 0.725, 0.125),
        ("m".to_string(), 0.775, 0.125),
        ("f".to_string(), 0.375, 0.075),
        ("f".to_string(), 0.425, 0.075),
        ("d".to_string(), 0.275, 0.075),
        ("d".to_string(), 0.325, 0.075),
        ("v".to_string(), 0.425, 0.125),
        ("v".to_string(), 0.475, 0.125),
        ("h".to_string(), 0.575, 0.075),
        ("h".to_string(), 0.625, 0.075),
        ("w".to_string(), 0.125, 0.025),
        ("w".to_string(), 0.175, 0.025),
        ("i".to_string(), 0.725, 0.025),
        ("i".to_string(), 0.775, 0.025),
        ("y".to_string(), 0.525, 0.025),
        ("y".to_string(), 0.575, 0.025),
        ("c".to_string(), 0.325, 0.125),
        ("c".to_string(), 0.375, 0.125),
        ("u".to_string(), 0.625, 0.025),
        ("u".to_string(), 0.675, 0.025),
        ("q".to_string(), 0.025, 0.025),
        ("q".to_string(), 0.075, 0.025),
        ("r".to_string(), 0.325, 0.025),
        ("r".to_string(), 0.375, 0.025),
        ("a".to_string(), 0.075, 0.075),
        ("a".to_string(), 0.125, 0.075),
        ("k".to_string(), 0.775, 0.075),
        ("k".to_string(), 0.825, 0.075),
        ("j".to_string(), 0.675, 0.075),
        ("j".to_string(), 0.725, 0.075),
        ("l".to_string(), 0.875, 0.075),
        ("l".to_string(), 0.925, 0.075),
        ("s".to_string(), 0.175, 0.075),
        ("s".to_string(), 0.225, 0.075),
        ("n".to_string(), 0.625, 0.125),
        ("n".to_string(), 0.675, 0.125),
        ("t".to_string(), 0.425, 0.025),
        ("t".to_string(), 0.475, 0.025),
        ("g".to_string(), 0.475, 0.075),
        ("g".to_string(), 0.525, 0.075),
        ("b".to_string(), 0.525, 0.125),
        ("b".to_string(), 0.575, 0.125),
        ("e".to_string(), 0.225, 0.025),
        ("e".to_string(), 0.275, 0.025),
        ("z".to_string(), 0.125, 0.125),
        ("z".to_string(), 0.175, 0.125),
        ("x".to_string(), 0.225, 0.125),
        ("x".to_string(), 0.275, 0.125),
        ("o".to_string(), 0.825, 0.025),
        ("o".to_string(), 0.875, 0.025),
        ("p".to_string(), 0.925, 0.025),
        ("p".to_string(), 0.975, 0.025),
    ]
}
