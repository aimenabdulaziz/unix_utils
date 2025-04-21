use clap::Parser;
use std::{fs, io};

#[derive(Parser, Debug)]
#[command(version, about = "Concatenate and print files", long_about = None)]
struct Args {
    // Number the non-blank output lines, starting at 1.
    #[arg(short, help = "Number the non-blank output lines, starting at 1.")]
    b: bool,

    // Number all output lines, starting at 1.
    #[arg(short, help = "Number all output lines, starting at 1.")]
    n: bool,

    // Squeeze multiple blank lines to a single blank line.
    #[arg(short, help = "Squeeze multiple blank lines to a single blank line.")]
    s: bool,

    // List of files
    files: Option<Vec<String>>,
}

fn read_from_stdin(all_contents: &mut String) -> io::Result<()> {
    let mut input = String::new();
    loop {
        match io::stdin().read_line(&mut input) {
            Ok(0) => break, // EOF reached
            Ok(_) => {
                print!("{input}");
                input.clear();
            }
            Err(e) => return Err(e),
        }
    }
    Ok(())
}

fn handle_error(e: io::Error, context: &str, error_code: i32) -> ! {
    eprintln!("{}: {}", context, e);
    std::process::exit(error_code);
}

fn process_input(input: &str, args: &Args) -> String {
    todo!()
}

fn main() {
    let args = Args::parse();
    let mut all_contents = String::new();

    match args.files {
        Some(files) => {
            for file in files {
                if file == "-" {
                    if let Err(e) = read_from_stdin(&mut all_contents) {
                        handle_error(e, "Error reading from stdin", 1);
                    }
                } else {
                    match fs::read_to_string(file) {
                        Ok(contents) => {
                            all_contents.push_str(&contents);
                            print!("{all_contents}");
                            all_contents.clear();
                        }
                        Err(e) => {
                            handle_error(e, "Error reading file", 2);
                        }
                    }
                }
            }
        }
        _ => {
            // Read from stdin if no files are given
            if let Err(e) = read_from_stdin(&mut all_contents) {
                handle_error(e, "Error reading from stdin", 3);
            }
        }
    }

    if args.b {
        println!("found b")
    }
    if args.n {
        println!("found n")
    }
    if args.s {
        println!("found s")
    }
}
