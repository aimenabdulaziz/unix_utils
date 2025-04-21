use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about = "Display a line of text", long_about = None)]
struct Args {
    // Flag for the newline
    // if true, don't print the newline
    #[arg(short)]
    n: bool,

    // Positional arguments for the string
    string: Option<String>,
}

fn main() {
    let args = Args::parse();

    if let Some(string) = args.string {
        print!("{}", string);
    }

    if !args.n {
        println!();
    }
}
