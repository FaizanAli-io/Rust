fn main() {
    for c in "hello WORLD".char_indices() {
        println!("{} -> {}", c.0, c.1);
    }
}
