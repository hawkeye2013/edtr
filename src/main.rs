
fn main() {

    if let Some((w, h)) = term_size::dimensions() {
        println!("Width: {}\nHeight: {}", w, h);
    } else {
        println!("Unable to get term size :(")
    }
}
