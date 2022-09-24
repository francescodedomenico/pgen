use clap::Parser;
use rand::{rngs::ThreadRng, Rng};
use serde_json;
use std::env;
use std::fs::File;
use std::io::Write;

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
    #[clap(short, long, value_parser = clap::value_parser!(u8).range(1..5), value_name = "COMPLEXITY")]
    complexity: Option<u8>,
    /// A custom dictionary for the password, if set the complexity parameter is ignored.
    #[clap(short, long, value_parser, value_name = "DICTIONARY")]
    dictionary: Option<String>,

    /// Number of passwords to be generated
    #[clap(short, long, value_parser, value_name = "NUMBER")]
    number: Option<u64>,

    /// Optional: writes the generated password/passwords into the specified file in the current working directory
    #[clap(short, long, value_parser, value_name = "OUTPUT_FILE")]
    output_file: Option<String>,

    /// Optional: passwords output as JSON
    #[clap(short, long, action)]
    json: bool,
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
    if let Some(input_number) = args.number {
        // Multi password branch
        if let Some(input_output_file) = args.output_file.as_deref() {
            // File output branch
            let temp_directory = env::current_dir().unwrap();
            let temp_file = temp_directory.join(input_output_file);
            let mut file = File::create(temp_file).unwrap();
            if args.json {
                let mut passwords: Vec<String> = Vec::new();
                for _i in 0..input_number {
                    let password: String = make_password(&args, &dictionary, &mut rng);
                    passwords.push(password);
                }
                let to_write_buffer: String = serde_json::to_string_pretty(&passwords).unwrap();
                writeln!(&mut file, "{}", to_write_buffer).unwrap();
            } else {
                for i in 0..input_number {
                    let password: String = make_password(&args, &dictionary, &mut rng);
                    writeln!(&mut file, "### PASSWORD {} ###\n{}\n", i + 1, password).unwrap();
                }
            }
        } else {
            if args.json {
                let mut passwords: Vec<String> = Vec::new();
                for _i in 0..input_number {
                    let password: String = make_password(&args, &dictionary, &mut rng);
                    passwords.push(password);
                }
                let to_write_buffer: String = serde_json::to_string_pretty(&passwords).unwrap();
                println!("{}", to_write_buffer);
            } else {
                for i in 0..input_number {
                    let password: String = make_password(&args, &dictionary, &mut rng);
                    println!("### PASSWORD {} ###\n{}\n", i + 1, password);
                }
            }
        }
    } else {
        // Single password branch
        let password: String = make_password(&args, &dictionary, &mut rng);
        if let Some(input_output_file) = args.output_file.as_deref() {
            // File output branch
            let temp_directory = env::temp_dir();
            let temp_file = temp_directory.join(input_output_file);
            let mut file = File::create(temp_file).unwrap();
            if args.json {
                // Json branch
                let mut passwords: Vec<String> = Vec::new();
                let password: String = make_password(&args, &dictionary, &mut rng);
                passwords.push(password);
                let to_write_buffer: String = serde_json::to_string_pretty(&passwords).unwrap();
                writeln!(&mut file, "{}", to_write_buffer).unwrap();
            } else {
                writeln!(&mut file, "### PASSWORD ###\n{}\n", password).unwrap();
            }
        } else {
            println!("### PASSWORD ###\n{}\n", password);
        }
    }
}
