use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about=None)]
struct Args {
    
    // Length
    #[arg(short, long, default_value_t = 16)]
    length: usize,

    /// Number of passwords to generate
    #[arg(long, default_value_t = 1)]
    count: usize,
}

fn main() {
    let args = Args::parse();

    // Build character set
    let mut charset = Vec::new();
    charset.extend("abcdefghijklmnopqrstuvwxyz".chars()); // lower case
    charset.extend("ABCDEFGHIJKLMNOPQRSTUVWXYZ".chars()); // capitalised 
    charset.extend("0123456789".chars()); // numbers
    charset.extend("!@#$%^&*()-_=+[]{};:,.?/".chars()); // only ASCII (keyboards nowadays tut)

    // Safety checks
    if charset.is_empty() {
        eprintln!("Error: character set is empty. Enable at least one of --lower/--upper/--digits/--symbols.");
        std::process::exit(1);
    }
    if args.length == 0 {
        eprintln!("Error: length must be > 0.");
        std::process::exit(1);
    }

    for _count in 0..args.count {
        let mut pwd = String::from(" ");
        for _length in 0..args.length {
            let rng= rand::random_range(0.. charset.len());
            pwd.push(charset[rng]);
        }
        println!("{}", pwd)
    }
}
