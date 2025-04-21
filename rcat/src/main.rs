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
fn handle_error(e: io::Error, context: &str, error_code: i32) -> ! {
    eprintln!("{}: {}", context, e);
    std::process::exit(error_code);
}

fn print_numbered_content(contents: &str, args: &Args, prev_blank: &mut bool, line_num: &mut i32) {
    for line in contents.lines() {
        let is_blank: bool = line.trim().is_empty();

        if args.s && is_blank && *prev_blank {
            continue;
        }

        // flag b takes precedence over flag n
        let print_line_num = if args.b { !is_blank } else { args.n };

        if print_line_num {
            println!("{:>6}\t{}", *line_num, line);
            *line_num += 1;
        } else {
            println!("{}", line);
        }
        *prev_blank = is_blank;
    }
}

fn read_from_stdin(args: &Args, prev_blank: &mut bool) -> io::Result<()> {
    let mut line_num = 1; // Start at 1 for each stdin session
    let mut input = String::new();

    loop {
        match io::stdin().read_line(&mut input) {
            Ok(0) => break, // EOF reached
            Ok(_) => {
                print_numbered_content(&input, args, prev_blank, &mut line_num);
                input.clear();
            }
            Err(e) => return Err(e),
        }
    }
    Ok(())
}

fn main() {
    let args = Args::parse();
    let mut prev_blank = false;

    match &args.files {
        Some(files) => {
            for file in files {
                if file == "-" {
                    if let Err(e) = read_from_stdin(&args, &mut prev_blank) {
                        handle_error(e, "Error reading from stdin", 1);
                    }
                } else {
                    match fs::read_to_string(file) {
                        Ok(contents) => {
                            let mut line_num = 1; // Start at 1 for each file
                            print_numbered_content(
                                &contents,
                                &args,
                                &mut prev_blank,
                                &mut line_num,
                            );
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
            if let Err(e) = read_from_stdin(&args, &mut prev_blank) {
                handle_error(e, "Error reading from stdin", 3);
            }
        }
    }
}
