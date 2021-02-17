use std::collections::HashMap;
use std::vec::Vec;

const RESOLUTION: f64 = 30.0;

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

    pub fn get_path(&self) -> Option<Vec<(f64, f64)>> {
        // Get waypoints
        let ideal_path = self.ideal_waypoints();
        // Interpolate the path
        if let Some(ideal_path) = ideal_path {
            self.ideal_path_interpolated(ideal_path)
        } else {
            None
        }
    }

    // Generates a path by connecting the centers of the keys of the word with straight lines. Only the waypoints are returned, nothing is interpolated
    pub fn ideal_waypoints(&self) -> Option<Vec<(f64, f64)>> {
        let mut points = Vec::new();
        for letter in &self.word {
            let letter_coordinate = self.key_layout.get(&letter.to_string());
            if let Some(letter_coordinate) = letter_coordinate {
                //println!("letter {}, coordinate {:?}", letter, letter_coordinate);
                points.push(*letter_coordinate);
            } else {
                //println!("No key found for letter {}, NO PATH POSSIBLE", letter);
                return None;
            }
        }
        Some(points)
    }

    pub fn ideal_path_interpolated(&self, waypoints: Vec<(f64, f64)>) -> Option<Vec<(f64, f64)>> {
        if waypoints.is_empty() {
            //println!("There must at least be one waypoint");
            return None;
        }

        let mut path: Vec<(f64, f64)> = Vec::new();
        let mut total_dist = 0.0;

        // Add the first waypoint
        path.push(waypoints[0]);

        // Calculate the total distance of the path
        let mut waypoints_iter = waypoints.iter().peekable();
        while let Some(start_point) = waypoints_iter.next() {
            if let Some(&end_point) = waypoints_iter.peek() {
                total_dist += dist(start_point, end_point);
            }
        }
        if (total_dist - 0.0).abs() < 0.00000001 {
            println!("The waypoints are all the same!");
            println!("Replaced them with just the first point");
            return Some(path);
        }

        // Interpolate the points of the path
        let mut leg_dist;
        let mut delta_x;
        let mut delta_y;

        let mut waypoints_iter = waypoints.iter().peekable();

        // While there are more waypoints, interpolate points for them
        while let Some(start_point) = waypoints_iter.next() {
            if let Some(&end_point) = waypoints_iter.peek() {
                leg_dist = dist(start_point, end_point);
                // Skip legs when their start and end points are equal
                if leg_dist < 0.00000001 {
                    continue;
                }
                let no_leg_points = leg_dist * RESOLUTION;
                let no_leg_points = no_leg_points.trunc();

                // println!("No of leg_points: {}", no_leg_points);

                // Calculate the delta and divide it by the number of points
                // This can also be interpreted as the slope of the linear function connecting the start and the end point
                delta_x = (end_point.0 - start_point.0) / no_leg_points;
                delta_y = (end_point.1 - start_point.1) / no_leg_points;
                for i in 1..no_leg_points as isize {
                    path.push((
                        delta_x * i as f64 + start_point.0,
                        delta_y * i as f64 + start_point.1,
                    ));
                }
                path.push(*end_point);
            }
        }

        Some(path)
    }
}

fn dist(start: &(f64, f64), end: &(f64, f64)) -> f64 {
    f64::sqrt((start.0 - end.0).powi(2) + (start.1 - end.1).powi(2))
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
