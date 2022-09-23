use clap::Parser;
use rand::{rngs::ThreadRng, Rng};
#[derive(Parser, Default, Debug)]
#[clap(
    author = "Francesco De Domenico",
    version,
    about = "A Very simple Password Generator!"
)]
struct Arguments {
    /// The desided length for the password
    #[clap(short, long, value_parser, value_name = "LENGTH")]
    length: usize,
    /// The complexity of the password. Values allowed: 1 - Lowercase dictionary; 2 - Lowercase and uppercase dictionary; 3 - Lowercase, uppercase and digits dictionary; 4 - lowercase, uppercase, digits and symbols dictionary;
    #[clap(short, long, value_parser = clap::value_parser!(u8).range(1..4), value_name = "COMPLEXITY")]
    complexity: Option<u8>,
    /// A custom dictionary for the password, if set the complexity parameter is ignored.
    #[clap(short, long, value_parser, value_name = "DICTIONARY")]
    dictionary: Option<String>,

    /// Number of passwords to be generated
    #[clap(short, long, value_parser, value_name = "NUMBER")]
    number: Option<u64>,
}

fn make_dictionary(args: &Arguments) -> String {
    const UPPERCASE: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    const LOWERCASE: &str = "abcdefghijklmnopqrstuvwxyz";
    const NUMBERS: &str = "0123456789";
    const SYMBOLS: &str = "\\|\"$%&/()='?^~";
    let mut dictionary: String = "".to_string();

    if let Some(input_dictionary) = args.dictionary.as_deref() {
        dictionary = input_dictionary.to_string();
    } else {
        if let Some(input_complexity) = args.complexity {
            match input_complexity {
                1 => {
                    dictionary.push_str(&LOWERCASE);
                }
                2 => {
                    dictionary.push_str(&UPPERCASE);
                    dictionary.push_str(&LOWERCASE);
                }
                3 => {
                    dictionary.push_str(&UPPERCASE);
                    dictionary.push_str(&LOWERCASE);
                    dictionary.push_str(&NUMBERS);
                }
                4 => {
                    dictionary.push_str(&UPPERCASE);
                    dictionary.push_str(&LOWERCASE);
                    dictionary.push_str(&NUMBERS);
                    dictionary.push_str(&SYMBOLS);
                }
                _ => {}
            }
        } else {
            dictionary.push_str(&UPPERCASE);
            dictionary.push_str(&LOWERCASE);
            dictionary.push_str(&NUMBERS);
            dictionary.push_str(&SYMBOLS);
        }
    }
    dictionary
}

fn make_password(args: &Arguments, dictionary: &String, rng: &mut ThreadRng) -> String {
    let password: String = (0..args.length)
        .map(|_| {
            let idx = rng.gen_range(0..dictionary.len());
            dictionary.chars().nth(idx).unwrap() as char
        })
        .collect();
    password
}
fn main() {
    let args = Arguments::parse();
    let dictionary: String = make_dictionary(&args);
    let mut rng = rand::thread_rng();
    println!("\n");
    if let Some(input_number) = args.number {
        for i in 0..input_number {
            let password: String = make_password(&args, &dictionary, &mut rng);
            println!("{}: {}", i, password);
            println!("\n");
        }
    } else {
        let password: String = make_password(&args, &dictionary, &mut rng);
        println!("{}", password);
    }
}
