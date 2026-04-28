use rand::Rng;

const LOWERCASE: &str = "abcdefghijklmnopqrstuvwxyz";
const UPPERCASE: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
const NUMBERS: &str = "0123456789";
const SYMBOLS: &str = "!@#$%^&*()-_=+[]{};:,.<>?";

pub fn generate_password(length: usize, numbers: bool, symbols: bool) -> String {
    let mut charset = String::new();

    charset.push_str(LOWERCASE);
    charset.push_str(UPPERCASE);

    if numbers {
        charset.push_str(NUMBERS);
    }

    if symbols {
        charset.push_str(SYMBOLS);
    }

    let chars: Vec<char> = charset.chars().collect();
    let mut rng = rand::thread_rng();

    (0..length)
        .map(|_| {
            let index = rng.gen_range(0..chars.len());
            chars[index]
        })
        .collect()
}