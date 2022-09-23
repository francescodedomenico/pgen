use clap::Parser;
use rand::Rng;
#[derive(Parser,Default,Debug)]
#[clap(author="Francesco De Domenico", version, about="A Very simple Password Generator!")]
struct Arguments {
    /// The desided length for the password
    #[clap(short, long, value_parser, value_name = "LENGTH")]
    length: usize,
    /// The complexity of the password. Values allowed: 1 - Lowercase dictionary; 2 - Lowercase and uppercase dictionary; 3 - Lowercase, uppercase and digits dictionary; 4 - lowercase, uppercase, digits and symbols dictionary;
    #[clap(short, long, value_parser = clap::value_parser!(u8).range(1..4), value_name = "COMPLEXITY")]
    complexity: Option<u8>,
    /// A custom dictionary for the password, if set the complexity parameter is ignored.
    #[clap(short, long, value_parser, value_name = "DICTIONARY")]
    dictionary: Option<String>
}

fn main() {
    let args = Arguments::parse();

    const UPPERCASE: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    const LOWERCASE: &str = "abcdefghijklmnopqrstuvwxyz";
    const NUMBERS: &str = "0123456789";
    const SYMBOLS: &str = "\\|\"$%&/()='?^~";
    let mut dictionary: String = "".to_string();
    
    if let Some(input_dictionary) = args.dictionary.as_deref(){
        dictionary = input_dictionary.to_string();
    }
    else{
        if let Some(input_complexity) = args.complexity{
            match input_complexity{
                1 => {
                    dictionary.push_str(&LOWERCASE);
                },
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
                _ =>{}
            }
        }
        else{
            dictionary.push_str(&UPPERCASE);
            dictionary.push_str(&LOWERCASE);
            dictionary.push_str(&NUMBERS);
            dictionary.push_str(&SYMBOLS);
        }
        
    }

    

    let mut rng = rand::thread_rng();

    let password: String = (0..args.length)
        .map(|_| {
            let idx = rng.gen_range(0..dictionary.len());
            dictionary.chars().nth(idx).unwrap() as char
        })
        .collect();
    println!("{}", password);
}
