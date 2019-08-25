use caesar_cipher as cipher;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let collection = cipher::Arguments::new(&args);
    //collection.print();
    let secret = collection.run();
    println!(
        "The message is:
'{}'",
        secret
    );
}
