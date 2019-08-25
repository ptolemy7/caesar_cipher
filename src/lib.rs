pub struct Arguments {
    //the -d or -e flag
    encode_or_decode: String,
    //This is usize so it can iterate an array
    key: usize,
    message: String,
}

pub struct Alnum(usize, char, char);
//Here there are 2 char feilds:
//one for lower case And one for upper case

impl Arguments {
    //use crate is_valid::*;
    //extern crate is_valid::*;
    pub fn new(args: &[String]) -> Result<Arguments, &'static str> {
        /*These if statements are the best way I have right now of checking
        the number of argumetns to be correct, we'll see how long they
        stay aroung*/
        if args.len() > 1 {
            //first checks to make sure there are arguments
            let encode_or_decode = is_valid::string_1(&args[1]).unwrap();
            /*This call of is_valid::string_1 will check and take care
            of -h and --help if they are passed*/
            if args.len() == 4 {
                /*If and only if there number of arguemtns is exactly
                 4 (because the first one is always the name does
                the cipher proceed*/
                let key = is_valid::number(&args[2]);
                let message = args[3].clone();
                Ok(Arguments {
                    encode_or_decode,
                    key,
                    message,
                })
            } else {
                if args.len() < 4 {
                    Err("Not enough arguments, use --help for more")
                } else {
                    Err("Too many arguments, use --help for more")
                }
            }
        } else {
            Err("No arguments passed, use --help for more")
        }
    }
    pub fn print(&self) {
        //Purely for debug purposes, just making sure that everything
        //is collecting correctly
        println!("The setting is {:?}", self.encode_or_decode);
        println!("The key is {}", self.key);
        println!("The message is {}", self.message);
    }
    pub fn run(&self) -> String {
        let mut encoded_message = String::new();
        let encryption_matrix: [Alnum; 27] = [
            //If one wishes to use a non latin script, this should be the
            //only place needing changed
            Alnum(0, 'a', 'A'),
            Alnum(1, 'b', 'B'),
            Alnum(2, 'c', 'C'),
            Alnum(3, 'd', 'D'),
            Alnum(4, 'e', 'E'),
            Alnum(5, 'f', 'F'),
            Alnum(6, 'g', 'G'),
            Alnum(7, 'h', 'H'),
            Alnum(8, 'i', 'I'),
            Alnum(9, 'j', 'J'),
            Alnum(10, 'k', 'K'),
            Alnum(11, 'l', 'L'),
            Alnum(12, 'm', 'M'),
            Alnum(13, 'n', 'N'),
            Alnum(14, 'o', 'O'),
            Alnum(15, 'p', 'P'),
            Alnum(16, 'q', 'Q'),
            Alnum(17, 'r', 'R'),
            Alnum(18, 's', 'S'),
            Alnum(19, 't', 'T'),
            Alnum(20, 'u', 'U'),
            Alnum(21, 'v', 'V'),
            Alnum(22, 'w', 'W'),
            Alnum(23, 'x', 'X'),
            Alnum(24, 'y', 'Y'),
            Alnum(25, 'z', 'S'),
            Alnum(26, ' ', ' '),
        ];
        for i in self.message.chars() {
            let index = lookup(i, &encryption_matrix);
            encoded_message
                .push(encryption_matrix[(index + to_encode_or_decode(&self).unwrap()) % 27].1);
        }
        encoded_message
    }
}

pub mod is_valid {
    use crate::print_help;
    use std::process;
    pub fn string_1(arg: &String) -> Result<String, &'static str> {
        //If -h is selected, then just display help and exit
        let return_arg = arg.clone();
        if (return_arg.as_str() == "-h") | (return_arg.as_str() == "--help") {
            print_help();
            process::exit(0);
        }
        //match the possible valid options, return false if invalid
        match arg.as_str() {
            "-e" => Ok(return_arg),
            "-d" => Ok(return_arg),
            "--encode" => Ok(return_arg),
            "--decode" => Ok(return_arg),
            _ => Err("Invalid arguemnts passed, pass -h to view options"),
        }
    }
    pub fn number(arg: &String) -> usize {
        //extracted for neetness mostly
        let new_arg = arg
            .clone()
            .parse::<usize>()
            .expect("Error, not a positive number!");
        new_arg
    }

}

pub fn lookup(letter: char, encryption_matrix: &[Alnum; 27]) -> usize {
    //This takes a char and sticks it to an index value in the array defined prior
    let mut letter_index: usize = 26;
    for _i in 0..encryption_matrix.len() {
        if (letter == encryption_matrix[_i].1) | (letter == encryption_matrix[_i].2) {
            letter_index = encryption_matrix[_i].0;
        }
    }
    letter_index
}

pub fn to_encode_or_decode(message: &Arguments) -> Result<usize, &'static str> {
    //variable initialized here purely for ease of shorter writing
    let actual_key = message.key % 27;
    match message.encode_or_decode.as_str() {
        /*there really isn't a way this will fail but
        I might as well avoid it if at all possible*/
        "-e" => Ok(actual_key),
        "-d" => Ok(27 - actual_key),
        _ => Err("Error, invalid key or option, use -h or --help for more"),
    }
}

pub fn print_help() {
    println!(
        "Useage: caesar_cipher <option>  <key> [message]
Warning: ALl messages will be encoded into all lower-case,
case sinsitivity is a work in progress
options:
    -e to encode a message w/ the specific key
    -d to decode a message w/ a specific key
    -h to display this screen"
    );
}
