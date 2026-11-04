use clap::Parser;
use rand::{rngs::ThreadRng, RngExt};
use std::env;
use std::fs::File;
use std::io::Write;

#[derive(Parser, Default, Debug)]
#[command(
    author = "Francesco De Domenico",
    version,
    about = "A Very simple Password Generator!"
)]
struct Arguments {
    /// The desired length for the password
    #[arg(short, long, value_name = "LENGTH")]
    length: usize,

    /// The complexity of the password. Values allowed: 1 - Lowercase dictionary; 2 - Lowercase and uppercase dictionary; 3 - Lowercase, uppercase and digits dictionary; 4 - lowercase, uppercase, digits and symbols dictionary;
    #[arg(short, long, value_parser = clap::value_parser!(u8).range(1..5), value_name = "COMPLEXITY")]
    complexity: Option<u8>,
    /// A custom dictionary for the password, if set the complexity parameter is ignored. In case you would like to use whitespaces in your dictionary ' ' use double quotes: -d "aAb _1"
    #[arg(short, long, value_name = "DICTIONARY")]
    dictionary: Option<String>,

    /// Number of passwords to be generated
    #[arg(short, long, value_name = "NUMBER")]
    number: Option<u64>,

    /// Optional: writes the generated password/passwords into the specified file in the current working directory
    #[arg(short, long, value_name = "OUTPUT_FILE")]
    output_file: Option<String>,

    /// Optional: passwords output as JSON
    #[arg(short, long)]
    json: bool,
}

fn make_dictionary(args: &Arguments) -> String {
    const UPPERCASE: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    const LOWERCASE: &str = "abcdefghijklmnopqrstuvwxyz";
    const NUMBERS: &str = "0123456789";
    const SYMBOLS: &str = "\\|\"$%&/()='?^~";
    let mut dictionary = String::new();

    if let Some(input_dictionary) = args.dictionary.as_deref() {
        dictionary.push_str(input_dictionary);
    } else {
        // No complexity given: the full dictionary.
        let complexity = args.complexity.unwrap_or(4);
        if complexity >= 2 {
            dictionary.push_str(UPPERCASE);
        }
        dictionary.push_str(LOWERCASE);
        if complexity >= 3 {
            dictionary.push_str(NUMBERS);
        }
        if complexity >= 4 {
            dictionary.push_str(SYMBOLS);
        }
    }
    dictionary
}

fn make_password(args: &Arguments, dictionary: &str, rng: &mut ThreadRng) -> String {
    // Sample whole characters, so a dictionary with multibyte characters never
    // indexes into the middle of one.
    let chars: Vec<char> = dictionary.chars().collect();
    (0..args.length)
        .map(|_| chars[rng.random_range(0..chars.len())])
        .collect()
}

fn make_passwords_vector(
    input_number: u64,
    args: &Arguments,
    dictionary: &str,
    rng: &mut ThreadRng,
) -> Vec<String> {
    (0..input_number)
        .map(|_| make_password(args, dictionary, rng))
        .collect()
}

fn make_file_from_cwd(file_name: &str) -> File {
    let temp_directory = env::current_dir().unwrap();
    let temp_file = temp_directory.join(file_name);
    File::create(temp_file).unwrap()
}

fn main() {
    let args = Arguments::parse();
    let dictionary: String = make_dictionary(&args);
    let mut rng = rand::rng();
    if let Some(input_number) = args.number {
        // Multi password branch
        if let Some(input_output_file) = args.output_file.as_deref() {
            // File output branch
            let mut file = make_file_from_cwd(input_output_file);
            if args.json {
                let passwords: Vec<String> =
                    make_passwords_vector(input_number, &args, &dictionary, &mut rng);
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
                let passwords: Vec<String> =
                    make_passwords_vector(input_number, &args, &dictionary, &mut rng);
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
            let mut file = make_file_from_cwd(input_output_file);
            if args.json {
                // Json branch
                let passwords: Vec<String> = make_passwords_vector(1, &args, &dictionary, &mut rng);
                let to_write_buffer: String = serde_json::to_string_pretty(&passwords).unwrap();
                writeln!(&mut file, "{}", to_write_buffer).unwrap();
            } else {
                writeln!(&mut file, "### PASSWORD ###\n{}\n", password).unwrap();
            }
        } else {
            if args.json {
                // Json branch
                let passwords: Vec<String> = make_passwords_vector(1, &args, &dictionary, &mut rng);
                let to_write_buffer: String = serde_json::to_string_pretty(&passwords).unwrap();
                println!("{}", to_write_buffer);
            } else {
                println!("### PASSWORD ###\n{}\n", password);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn args(length: usize, complexity: Option<u8>, dictionary: Option<&str>) -> Arguments {
        Arguments {
            length,
            complexity,
            dictionary: dictionary.map(str::to_string),
            number: None,
            output_file: None,
            json: false,
        }
    }

    #[test]
    fn custom_dictionary_wins_over_complexity() {
        let dictionary = make_dictionary(&args(8, Some(1), Some("xyz 1")));
        assert_eq!(dictionary, "xyz 1");
    }

    #[test]
    fn complexity_levels_grow_the_dictionary() {
        let level: Vec<String> = (1..=4)
            .map(|c| make_dictionary(&args(8, Some(c), None)))
            .collect();
        assert_eq!(level[0], "abcdefghijklmnopqrstuvwxyz");
        assert!(level[1].contains('A') && level[1].contains('a') && !level[1].contains('0'));
        assert!(level[2].contains('0') && !level[2].contains('$'));
        assert!(level[3].contains('$') && level[3].contains('0') && level[3].contains('a'));
        assert!(level[0].len() < level[1].len() && level[1].len() < level[2].len());
        assert!(level[2].len() < level[3].len());
    }

    #[test]
    fn no_complexity_means_the_full_dictionary() {
        assert_eq!(
            make_dictionary(&args(8, None, None)),
            make_dictionary(&args(8, Some(4), None))
        );
    }

    #[test]
    fn password_has_the_requested_length_and_only_dictionary_chars() {
        let a = args(64, None, Some("ab1"));
        let dictionary = make_dictionary(&a);
        let mut rng = rand::rng();
        let password = make_password(&a, &dictionary, &mut rng);
        assert_eq!(password.chars().count(), 64);
        assert!(password.chars().all(|c| "ab1".contains(c)));
    }

    #[test]
    fn multibyte_dictionary_chars_are_sampled_whole() {
        let a = args(32, None, Some("äöü€"));
        let dictionary = make_dictionary(&a);
        let mut rng = rand::rng();
        let password = make_password(&a, &dictionary, &mut rng);
        assert_eq!(password.chars().count(), 32);
        assert!(password.chars().all(|c| "äöü€".contains(c)));
    }

    #[test]
    fn passwords_vector_has_the_requested_count() {
        let a = args(10, Some(3), None);
        let dictionary = make_dictionary(&a);
        let mut rng = rand::rng();
        let passwords = make_passwords_vector(5, &a, &dictionary, &mut rng);
        assert_eq!(passwords.len(), 5);
        assert!(passwords.iter().all(|p| p.len() == 10));
    }
}
