use std::collections::HashMap;

// Some buttons are wider than one cell
// To deal with this, the average of their coordinates is assumed to be their center.
// This relies on the assumption that buttons are always adjacent if there are multiple occurences
pub fn get_button_centers() -> HashMap<std::string::String, Point> {
    let buttons_raw = get_buttons();
    // Get sum up the coordinates of the buttons
    let mut buttons_coordinates = HashMap::new();
    for (button_id, x, y) in buttons_raw.iter() {
        let button_entry = buttons_coordinates.entry(button_id).or_insert((0, 0, 0));
        let (button_x, button_y, button_count) = button_entry;
        *button_x += x;
        *button_y += y;
        *button_count += 1;
    }

    // Divide the coordinates by the number of occurences of the buttons to get their center.
    let mut buttons_coordinates_normalized = HashMap::new();
    for (button_id, (button_x, button_y, button_count)) in buttons_coordinates.iter() {
        let button_x = button_x / button_count;
        let button_y = button_y / button_count;
        buttons_coordinates_normalized.insert(
            button_id.to_string(),
            Point {
                x: button_x,
                y: button_y,
            },
        );
    }
    for (button_name, button) in buttons_coordinates_normalized.iter() {
        //buttons.push((".".to_string(), 0.775, 0.875));
        println!(
            "buttons.push((\"{}\".to_string(), {}, {}));",
            button_name,
            button.x as f64 / 1000.0,
            button.y as f64 / 1000.0,
        );
    }

    buttons_coordinates_normalized
}

fn get_buttons() -> [(String, i32, i32); 78] {
    let array: [(String, i32, i32); 78] = [
        (".".to_string(), 775, 875),
        ("GBA".to_string(), 125, 875),
        ("g".to_string(), 525, 375),
        ("o".to_string(), 825, 125),
        ("r".to_string(), 375, 125),
        ("l".to_string(), 925, 375),
        ("d".to_string(), 325, 375),
        ("k".to_string(), 825, 375),
        ("q".to_string(), 75, 125),
        ("Shift_L_base".to_string(), 25, 625),
        (":)".to_string(), 275, 875),
        ("space".to_string(), 525, 875),
        ("i".to_string(), 775, 125),
        ("k".to_string(), 775, 375),
        ("BackSpace".to_string(), 975, 625),
        ("BackSpace".to_string(), 875, 625),
        ("b".to_string(), 575, 625),
        ("Return".to_string(), 825, 875),
        ("space".to_string(), 475, 875),
        ("space".to_string(), 375, 875),
        ("t".to_string(), 425, 125),
        ("w".to_string(), 175, 125),
        ("v".to_string(), 425, 625),
        ("j".to_string(), 725, 375),
        ("i".to_string(), 725, 125),
        ("y".to_string(), 575, 125),
        ("j".to_string(), 675, 375),
        ("f".to_string(), 375, 375),
        ("n".to_string(), 625, 625),
        ("space".to_string(), 575, 875),
        ("show_numbers".to_string(), 75, 875),
        ("z".to_string(), 125, 625),
        ("w".to_string(), 125, 125),
        ("Shift_L_base".to_string(), 75, 625),
        (".".to_string(), 725, 875),
        ("space".to_string(), 325, 875),
        ("s".to_string(), 175, 375),
        ("q".to_string(), 25, 125),
        ("g".to_string(), 475, 375),
        ("u".to_string(), 625, 125),
        ("c".to_string(), 375, 625),
        ("x".to_string(), 275, 625),
        ("space".to_string(), 425, 875),
        ("n".to_string(), 675, 625),
        ("BackSpace".to_string(), 925, 625),
        ("o".to_string(), 875, 125),
        ("d".to_string(), 275, 375),
        ("GBA".to_string(), 175, 875),
        ("y".to_string(), 525, 125),
        ("Return".to_string(), 875, 875),
        ("h".to_string(), 625, 375),
        ("a".to_string(), 125, 375),
        ("p".to_string(), 925, 125),
        ("l".to_string(), 875, 375),
        ("t".to_string(), 475, 125),
        ("space".to_string(), 625, 875),
        ("BackSpace".to_string(), 825, 625),
        ("Return".to_string(), 975, 875),
        ("z".to_string(), 175, 625),
        ("show_numbers".to_string(), 25, 875),
        ("c".to_string(), 325, 625),
        ("a".to_string(), 75, 375),
        ("e".to_string(), 225, 125),
        ("b".to_string(), 525, 625),
        ("space".to_string(), 675, 875),
        ("u".to_string(), 675, 125),
        ("x".to_string(), 225, 625),
        ("p".to_string(), 975, 125),
        ("e".to_string(), 275, 125),
        ("h".to_string(), 575, 375),
        ("m".to_string(), 725, 625),
        ("m".to_string(), 775, 625),
        ("f".to_string(), 425, 375),
        ("s".to_string(), 225, 375),
        (":)".to_string(), 225, 875),
        ("Return".to_string(), 925, 875),
        ("v".to_string(), 475, 625),
        ("r".to_string(), 325, 125),
    ];
    array
}
