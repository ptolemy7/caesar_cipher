pub struct Arguments {
    encode_or_decode: String,
    key: i8,
    message: String,
}

impl Arguments {
    pub fn
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
