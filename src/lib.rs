pub struct Arguments {
    encode_or_decode: String,
    key: i8,
    message: String,
}

impl Arguments {
    pub fn new(args: &[String]) -> Arguments {
        if is_flag_valid(args[1].clone) {
            let encode_or_decode = args[1].clone;
            let key = args[2].clone().parse()::<i8>().expect("Error, not a number");
            let message = args[3];
            Arguments { encode_or_decode,key,message }
        }
        else {
            print_help();
        }
    }
}

pub fn print_help() {
    println!("Unknown command\n")
}
pub fn is_flag_valid(flag: String) -> bool {
    let mut is_valid: bool;
    match flag {
        "-e" => is_valid = true,
        "-d" => is_valid = true,
        _ => is_valid = false,
    }
    is_valid
}
