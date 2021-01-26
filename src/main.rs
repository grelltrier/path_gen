fn main() {
    let word = "top";
    let ideal_path = path_gen::get_path(word, 100);

    // Print the path for debugging
    println!("Path for '{}':", word);
    for (i, point) in ideal_path.iter().enumerate() {
        println!("NO: {}, x: {:.3}, y: {:.3}", i, point.0, point.1);
    }
}
