use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let collection = caesar_cipher::Arguments::new(&args);
    collection.print();
    let secret = collection.translate();
    println!("{}", secret);
}
