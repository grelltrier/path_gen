use path_gen::{get_default_buttons_centers, WordPath};

fn main() {
    /*let word = "doing";
    let key_layout = get_default_buttons_centers();
    let word_path = WordPath::new(&key_layout, word);
    let ideal_path = word_path.get_path();

    if let Some(ideal_path) = ideal_path {
        // Print the path for debugging
        println!("Path for '{}':", word);
        for (i, point) in ideal_path.iter().enumerate() {
            println!("NO: {}, x: {:.3}, y: {:.3}", i, point.0, point.1);
        }
    } else {
        println!("There is no valid path possible");
    } */

    path_gen::get_button_centers();
}
