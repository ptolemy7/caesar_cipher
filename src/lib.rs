pub struct Arguments {
    encode_or_decode: String,
    key: usize,
    message: String,
}

pub struct Alnum(usize, char);

impl Arguments {
    pub fn new(args: &[String]) -> Arguments {
        let encode_or_decode = args[1].clone();
        let key = args[2].clone().parse::<usize>().expect("Not a number");
        let message = args[3].clone();
        Arguments {
            encode_or_decode,
            key,
            message,
        }
    }
    pub fn print(&self) {
        println!("The setting is {:?}", self.encode_or_decode);
        println!("The key is {}", self.key);
        println!("The message is {}", self.message);
    }
    pub fn translate(&self) -> String {
        let mut encoded_message = String::new();
        let encryption_matrix: [Alnum; 27] = [
            //If one wishes to use a non latin script, this should be the
            //only place needing changed
            Alnum(0, 'a'),
            Alnum(1, 'b'),
            Alnum(2, 'c'),
            Alnum(3, 'd'),
            Alnum(4, 'e'),
            Alnum(5, 'f'),
            Alnum(6, 'g'),
            Alnum(7, 'h'),
            Alnum(8, 'i'),
            Alnum(9, 'j'),
            Alnum(10, 'k'),
            Alnum(11, 'l'),
            Alnum(12, 'm'),
            Alnum(13, 'n'),
            Alnum(14, 'o'),
            Alnum(15, 'p'),
            Alnum(16, 'q'),
            Alnum(17, 'r'),
            Alnum(18, 's'),
            Alnum(19, 't'),
            Alnum(20, 'u'),
            Alnum(21, 'v'),
            Alnum(22, 'w'),
            Alnum(23, 'x'),
            Alnum(24, 'y'),
            Alnum(25, 'z'),
            Alnum(26, ' '),
        ];
        for i in self.message.chars() {
            let index = lookup(i, &encryption_matrix);
            encoded_message.push(encryption_matrix[(index + self.key) % 27].1);
        }
        encoded_message
    }
}

pub fn lookup(letter: char, encryption_matrix: &[Alnum; 27]) -> usize {
    //This takes a char and sticks it to an index value in the array defined prior
    let mut letter_index: usize = 26;
    for _i in 0..encryption_matrix.len() {
        if letter == encryption_matrix[_i].1 {
            letter_index = encryption_matrix[_i].0;
        }
    }
    return letter_index;
}
