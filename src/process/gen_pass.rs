use anyhow::Result;
use rand::{rngs::ThreadRng, seq::SliceRandom};
use std::cmp::max;
use zxcvbn::zxcvbn;

const UPPERS: &[u8] = b"ABCDEFGHJKLMNPQRSTUVWXYZ";
const LOWERS: &[u8] = b"abcdefghijkmnopqrstuvwxyz";
const NUMBERS: &[u8] = b"123456789";
const SYMBOLS: &[u8] = b"!@#$%^&*_";

pub fn process_genpass(
    length: u8,
    upper: bool,
    lower: bool,
    numbers: bool,
    symbol: bool,
) -> Result<String> {
    let mut password = Vec::new();
    let mut rng = rand::thread_rng();
    let mut chars = Vec::new();

    if upper {
        chars.extend_from_slice(UPPERS);
        password.push(get_random_char_slice(UPPERS, &mut rng));
    }

    if lower {
        chars.extend_from_slice(LOWERS);
        password.push(get_random_char_slice(LOWERS, &mut rng));
    }

    if numbers {
        chars.extend_from_slice(NUMBERS);
        password.push(get_random_char_slice(NUMBERS, &mut rng));
    }

    if symbol {
        chars.extend_from_slice(SYMBOLS);
        password.push(get_random_char_slice(SYMBOLS, &mut rng));
    }

    for _ in 0..max(0, length - password.len() as u8) {
        let c = chars.choose(&mut rng).expect("chars won't be empty");
        password.push(*c);
    }
    password.shuffle(&mut rng);

    let password = String::from_utf8(password)?;
    let estimate = zxcvbn(password.as_str(), &[]).unwrap();

    eprintln!("Estimated strength: {}", estimate.score());
    // output

    Ok(password)
}

fn get_random_char_slice(chars: &'static [u8], rng: &mut ThreadRng) -> u8 {
    *chars.choose(rng).expect("chars won't be empty")
}
