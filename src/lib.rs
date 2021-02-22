use std::collections::HashMap;
use std::vec::Vec;

pub struct WordPath<'a> {
    word: Vec<char>,
    key_layout: &'a HashMap<String, (f64, f64)>,
    no_points: usize,
}

impl<'a> WordPath<'a> {
    pub fn new(key_layout: &'a HashMap<String, (f64, f64)>, word: &str, no_points: usize) -> Self {
        let mut word: Vec<char> = word.chars().collect();
        word.dedup();
        // Ignore the case
        let word: String = word.into_iter().collect::<String>().to_ascii_lowercase();
        let word: Vec<char> = word.chars().collect();
        Self {
            word,
            key_layout,
            no_points,
        }
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
    // Waypoints that are the same are merged into one
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
        path_length: f64,
    ) -> Option<(Vec<(f64, f64)>, f64)> {
        // If there are no waypoints, we can't construct a path
        // This can only happen for an empty string (which should never occur)
        // If there are more waypoints than the number of requested points, we also cannot construct a path
        if waypoints.len() == 0 || self.no_points < waypoints.len() {
            return None;
        }

        if path_length < 0.00000001 {
            //println!("The waypoints are all the same!");
            //println!("Replaced them with just the first point");
            return Some((vec![waypoints[0]; self.no_points], path_length));
        }

        // Interpolate the points of the path
        let mut leg_dist;
        let mut delta_x;
        let mut delta_y;

        let point_dist = path_length / (self.no_points - 1) as f64;
        let mut remainder = 0.0;

        let mut path: Vec<(f64, f64)> = Vec::new();
        let mut waypoints_iter = waypoints.iter().peekable();

        // While there are more waypoints, interpolate points for them
        while let Some(start_point) = waypoints_iter.next() {
            path.push(*start_point);
            if let Some(&end_point) = waypoints_iter.peek() {
                leg_dist = dist(start_point, end_point);
                let no_leg_sections = leg_dist / point_dist + remainder;
                remainder = no_leg_sections.fract();
                let no_leg_sections = no_leg_sections.trunc();

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
        Some((path, path_length))
    }

    pub fn get_path(&self) -> Option<(Vec<(f64, f64)>, f64)> {
        // Get waypoints
        let ideal_path = self.ideal_waypoints();
        // Interpolate the path
        if let Some((ideal_path, path_length)) = ideal_path {
            self.ideal_path_interpolated(ideal_path, path_length)
        } else {
            None
        }
    }
}

fn dist(start: &(f64, f64), end: &(f64, f64)) -> f64 {
    f64::sqrt((start.0 - end.0).powi(2) + (start.1 - end.1).powi(2))
}

// Gets the centers of all default buttons
/*pub fn get_default_buttons_centers() -> HashMap<String, (f64, f64)> {
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
}*/

pub fn get_default_buttons_centers() -> HashMap<String, (f64, f64)> {
    let mut buttons = HashMap::new();
    buttons.insert("h".to_string(), (0.600, 0.15));
    buttons.insert("p".to_string(), (0.950, 0.05));
    buttons.insert("q".to_string(), (0.050, 0.05));
    buttons.insert("d".to_string(), (0.300, 0.15));
    buttons.insert("x".to_string(), (0.250, 0.25));
    buttons.insert("s".to_string(), (0.200, 0.15));
    buttons.insert("y".to_string(), (0.550, 0.05));
    buttons.insert("i".to_string(), (0.750, 0.05));
    buttons.insert("j".to_string(), (0.700, 0.15));
    buttons.insert("w".to_string(), (0.150, 0.05));
    buttons.insert("e".to_string(), (0.250, 0.05));
    buttons.insert("c".to_string(), (0.350, 0.25));
    buttons.insert("m".to_string(), (0.750, 0.25));
    buttons.insert("k".to_string(), (0.800, 0.15));
    buttons.insert("a".to_string(), (0.100, 0.15));
    buttons.insert("o".to_string(), (0.850, 0.05));
    buttons.insert("t".to_string(), (0.450, 0.05));
    buttons.insert("z".to_string(), (0.150, 0.25));
    buttons.insert("b".to_string(), (0.550, 0.25));
    buttons.insert("g".to_string(), (0.500, 0.15));
    buttons.insert("r".to_string(), (0.350, 0.05));
    buttons.insert("l".to_string(), (0.900, 0.15));
    buttons.insert("f".to_string(), (0.400, 0.15));
    buttons.insert("n".to_string(), (0.650, 0.25));
    buttons.insert("v".to_string(), (0.450, 0.25));
    buttons.insert("u".to_string(), (0.650, 0.05));

    buttons
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

fn get_buttons() -> Vec<(String, f64, f64)> {
    vec![
        ("m".to_string(), 0.7250000000000001, 0.125),
        ("m".to_string(), 0.775, 0.125),
        ("f".to_string(), 0.37500000000000006, 0.07500000000000001),
        ("f".to_string(), 0.42500000000000004, 0.07500000000000001),
        ("d".to_string(), 0.275, 0.07500000000000001),
        ("d".to_string(), 0.32500000000000007, 0.07500000000000001),
        ("v".to_string(), 0.42500000000000004, 0.125),
        ("v".to_string(), 0.47500000000000003, 0.125),
        ("h".to_string(), 0.5750000000000001, 0.07500000000000001),
        ("h".to_string(), 0.6250000000000001, 0.07500000000000001),
        ("w".to_string(), 0.125, 0.025),
        ("w".to_string(), 0.17500000000000002, 0.025),
        ("i".to_string(), 0.7250000000000001, 0.025),
        ("i".to_string(), 0.775, 0.025),
        ("y".to_string(), 0.525, 0.025),
        ("y".to_string(), 0.5750000000000001, 0.025),
        ("c".to_string(), 0.32500000000000007, 0.125),
        ("c".to_string(), 0.37500000000000006, 0.125),
        ("u".to_string(), 0.6250000000000001, 0.025),
        ("u".to_string(), 0.675, 0.025),
        ("q".to_string(), 0.025, 0.025),
        ("q".to_string(), 0.07500000000000001, 0.025),
        ("r".to_string(), 0.32500000000000007, 0.025),
        ("r".to_string(), 0.37500000000000006, 0.025),
        ("a".to_string(), 0.07500000000000001, 0.07500000000000001),
        ("a".to_string(), 0.125, 0.07500000000000001),
        ("k".to_string(), 0.775, 0.07500000000000001),
        ("k".to_string(), 0.8250000000000001, 0.07500000000000001),
        ("j".to_string(), 0.675, 0.07500000000000001),
        ("j".to_string(), 0.7250000000000001, 0.07500000000000001),
        ("l".to_string(), 0.8750000000000001, 0.07500000000000001),
        ("l".to_string(), 0.925, 0.07500000000000001),
        ("s".to_string(), 0.17500000000000002, 0.07500000000000001),
        ("s".to_string(), 0.225, 0.07500000000000001),
        ("n".to_string(), 0.6250000000000001, 0.125),
        ("n".to_string(), 0.675, 0.125),
        ("t".to_string(), 0.42500000000000004, 0.025),
        ("t".to_string(), 0.47500000000000003, 0.025),
        ("g".to_string(), 0.47500000000000003, 0.07500000000000001),
        ("g".to_string(), 0.525, 0.07500000000000001),
        ("b".to_string(), 0.525, 0.125),
        ("b".to_string(), 0.5750000000000001, 0.125),
        ("e".to_string(), 0.225, 0.025),
        ("e".to_string(), 0.275, 0.025),
        ("z".to_string(), 0.125, 0.125),
        ("z".to_string(), 0.17500000000000002, 0.125),
        ("x".to_string(), 0.225, 0.125),
        ("x".to_string(), 0.275, 0.125),
        ("o".to_string(), 0.8250000000000001, 0.025),
        ("o".to_string(), 0.8750000000000001, 0.025),
        ("p".to_string(), 0.925, 0.025),
        ("p".to_string(), 0.9750000000000001, 0.025),
        /*

               (".".to_string(), 0.7250000000000001, 0.17500000000000002),
               (".".to_string(), 0.775, 0.17500000000000002),
        (
                   "Return".to_string(),
                   0.8250000000000001,
                   0.17500000000000002,
               ),
               (
                   "Return".to_string(),
                   0.8750000000000001,
                   0.17500000000000002,
               ),
               ("Return".to_string(), 0.925, 0.17500000000000002),
               (
                   "Return".to_string(),
                   0.9750000000000001,
                   0.17500000000000002,
               ),

               ("Shift_L_base".to_string(), 0.025, 0.125),
               ("Shift_L_base".to_string(), 0.07500000000000001, 0.125),
               ("show_numbers".to_string(), 0.025, 0.17500000000000002),
               (
                   "show_numbers".to_string(),
                   0.07500000000000001,
                   0.17500000000000002,
               ),
               (":)".to_string(), 0.225, 0.17500000000000002),
               (":)".to_string(), 0.275, 0.17500000000000002),
               ("BackSpace".to_string(), 0.8250000000000001, 0.125),
               ("BackSpace".to_string(), 0.8750000000000001, 0.125),
               ("BackSpace".to_string(), 0.925, 0.125),
               ("BackSpace".to_string(), 0.9750000000000001, 0.125),
               (
                   "space".to_string(),
                   0.32500000000000007,
                   0.17500000000000002,
               ),
               (
                   "space".to_string(),
                   0.37500000000000006,
                   0.17500000000000002,
               ),
               (
                   "space".to_string(),
                   0.42500000000000004,
                   0.17500000000000002,
               ),
               (
                   "space".to_string(),
                   0.47500000000000003,
                   0.17500000000000002,
               ),
               ("space".to_string(), 0.525, 0.17500000000000002),
               ("space".to_string(), 0.5750000000000001, 0.17500000000000002),
               ("space".to_string(), 0.6250000000000001, 0.17500000000000002),
               ("space".to_string(), 0.675, 0.17500000000000002),
               ("GBA".to_string(), 0.125, 0.17500000000000002),
               ("GBA".to_string(), 0.17500000000000002, 0.17500000000000002),*/
    ]
}
