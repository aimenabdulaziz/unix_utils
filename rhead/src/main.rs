use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about = "Display a line of text", long_about = None)]
struct Args {
    #[arg(
        short,
        default_value_t = 10,
        help = "Print count lines of each of the specified files."
    )]
    n: i32,

    // List of files
    files: Option<Vec<String>>,
}

fn main() {
    let args = Args::parse();

    let num_lines = args.n;

    println!("n: {}", args.n);
    println!("{:?}", args.files);
}

// if there is no file, read from stdin
// if there is a file, read from file
// if there are multiple files, read from each file
/*
==> file name <==
{file contents}

==> file2.txt <==

{file contents}.
*/
