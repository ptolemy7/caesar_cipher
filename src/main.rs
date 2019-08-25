use caesar_cipher as cipher;
use std::env;

fn main() {
    //Collect the input stream
    let args: Vec<String> = env::args().collect();
    //Pass the args into new
    let collection = cipher::Arguments::new(&args).unwrap();
    //Run the translation(the rest of the logic is in lib.rs)
    let secret = collection.run();
    println!(
        "The message is:
'{}'",
        secret
    );
}
