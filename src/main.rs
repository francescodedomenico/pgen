use clap::Parser;
use rand::{RngExt, rngs::ThreadRng};
use std::fs::File;
use std::io::{self, Write};

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
    #[arg(short, long, value_name = "DICTIONARY", value_parser = clap::builder::NonEmptyStringValueParser::new())]
    dictionary: Option<String>,

    /// Number of passwords to be generated
    #[arg(short, long, value_name = "NUMBER")]
    number: Option<u64>,

    /// Optional: writes the generated password/passwords into the specified file (a relative path is resolved from the current working directory)
    #[arg(short, long, value_name = "OUTPUT_FILE")]
    output_file: Option<String>,

    /// Optional: passwords output as JSON
    #[arg(short, long)]
    json: bool,
}

/// The characters a password is drawn from: the custom dictionary as given,
/// or the predefined levels (each a superset of the one below).
fn make_dictionary(args: &Arguments) -> Vec<char> {
    const UPPERCASE: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    const LOWERCASE: &str = "abcdefghijklmnopqrstuvwxyz";
    const NUMBERS: &str = "0123456789";
    const SYMBOLS: &str = "\\|\"$%&/()='?^~";

    if let Some(input_dictionary) = args.dictionary.as_deref() {
        return input_dictionary.chars().collect();
    }
    // No complexity given: the full dictionary.
    let complexity = args.complexity.unwrap_or(4);
    let mut dictionary = String::new();
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
    dictionary.chars().collect()
}

fn make_password(length: usize, dictionary: &[char], rng: &mut ThreadRng) -> String {
    (0..length)
        .map(|_| dictionary[rng.random_range(0..dictionary.len())])
        .collect()
}

fn make_passwords(
    count: u64,
    length: usize,
    dictionary: &[char],
    rng: &mut ThreadRng,
) -> Vec<String> {
    (0..count)
        .map(|_| make_password(length, dictionary, rng))
        .collect()
}

/// The output as written to stdout or the file: a pretty JSON array, or one
/// `### PASSWORD ###` block per password (numbered when `--number` was given),
/// each followed by a blank line.
fn render(passwords: &[String], json: bool, numbered: bool) -> String {
    if json {
        let mut out = serde_json::to_string_pretty(passwords).expect("strings serialize");
        out.push('\n');
        return out;
    }
    passwords
        .iter()
        .enumerate()
        .map(|(i, password)| {
            if numbered {
                format!("### PASSWORD {} ###\n{}\n\n", i + 1, password)
            } else {
                format!("### PASSWORD ###\n{}\n\n", password)
            }
        })
        .collect()
}

fn run(args: &Arguments) -> io::Result<()> {
    let dictionary = make_dictionary(args);
    let mut rng = rand::rng();
    let passwords = make_passwords(args.number.unwrap_or(1), args.length, &dictionary, &mut rng);
    let output = render(&passwords, args.json, args.number.is_some());

    // A relative --output-file lands in the current working directory; an
    // absolute one is written where it says.
    let mut sink: Box<dyn Write> = match args.output_file.as_deref() {
        Some(path) => Box::new(
            File::create(path).map_err(|e| io::Error::new(e.kind(), format!("{path}: {e}")))?,
        ),
        None => Box::new(io::stdout().lock()),
    };
    sink.write_all(output.as_bytes())?;
    sink.flush()
}

fn main() {
    let args = Arguments::parse();
    if let Err(error) = run(&args) {
        eprintln!("pgen: {error}");
        std::process::exit(1);
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

    fn dictionary_string(a: &Arguments) -> String {
        make_dictionary(a).into_iter().collect()
    }

    #[test]
    fn custom_dictionary_wins_over_complexity() {
        assert_eq!(dictionary_string(&args(8, Some(1), Some("xyz 1"))), "xyz 1");
    }

    #[test]
    fn complexity_levels_grow_the_dictionary() {
        let level: Vec<String> = (1..=4)
            .map(|c| dictionary_string(&args(8, Some(c), None)))
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
            dictionary_string(&args(8, None, None)),
            dictionary_string(&args(8, Some(4), None))
        );
    }

    #[test]
    fn password_has_the_requested_length_and_only_dictionary_chars() {
        let dictionary = make_dictionary(&args(64, None, Some("ab1")));
        let password = make_password(64, &dictionary, &mut rand::rng());
        assert_eq!(password.chars().count(), 64);
        assert!(password.chars().all(|c| "ab1".contains(c)));
    }

    #[test]
    fn multibyte_dictionary_chars_are_sampled_whole() {
        let dictionary = make_dictionary(&args(32, None, Some("äöü€")));
        let password = make_password(32, &dictionary, &mut rand::rng());
        assert_eq!(password.chars().count(), 32);
        assert!(password.chars().all(|c| "äöü€".contains(c)));
    }

    #[test]
    fn empty_custom_dictionary_is_refused_at_parse_time() {
        let parsed = Arguments::try_parse_from(["pgen", "-l", "8", "-d", ""]);
        assert!(
            parsed.is_err(),
            "an empty dictionary must be a usage error, not a panic"
        );
    }

    #[test]
    fn passwords_have_the_requested_count() {
        let dictionary = make_dictionary(&args(10, Some(3), None));
        let passwords = make_passwords(5, 10, &dictionary, &mut rand::rng());
        assert_eq!(passwords.len(), 5);
        assert!(passwords.iter().all(|p| p.len() == 10));
    }

    #[test]
    fn text_output_is_one_block_per_password_numbered_only_with_number() {
        let one = vec!["abc".to_string()];
        assert_eq!(render(&one, false, false), "### PASSWORD ###\nabc\n\n");
        assert_eq!(render(&one, false, true), "### PASSWORD 1 ###\nabc\n\n");
        let two = vec!["abc".to_string(), "def".to_string()];
        assert_eq!(
            render(&two, false, true),
            "### PASSWORD 1 ###\nabc\n\n### PASSWORD 2 ###\ndef\n\n"
        );
    }

    #[test]
    fn json_output_is_a_pretty_array_even_for_one_password() {
        let one = vec!["a\"b".to_string()];
        assert_eq!(render(&one, true, false), "[\n  \"a\\\"b\"\n]\n");
        let parsed: Vec<String> = serde_json::from_str(&render(&one, true, true)).unwrap();
        assert_eq!(parsed, one);
    }
}
