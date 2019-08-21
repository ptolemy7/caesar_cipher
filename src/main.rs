use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let collection = Arguments::new(&args);
    collection.print();
}

struct Arguments {
    encode_or_decode: String,
    key: i8,
    message: String,
}

impl Arguments {
    fn new(args: &[String]) -> Arguments {
        let encode_or_decode = args[1].clone();
        let key = args[2].clone().parse::<i8>().expect("Not a number");
        let message = args[3].clone();
        Arguments {
            encode_or_decode,
            key,
            message,
        }
    }
    fn print(&self) {
        println!("The setting is {:?}", self.encode_or_decode);
        println!("The key is {}", self.key);
        println!("The message is {}", self.message);
    }
}
