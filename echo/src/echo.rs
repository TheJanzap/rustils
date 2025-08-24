//! Small port of the echo command without an external argument parser

use std::{env, io::Write, process};

#[derive(PartialEq, Debug)]
struct Options {
    /// Whether a newline should be triggered at the end of the output
    trailing_newline: bool,
    args: Vec<String>,
}

impl Default for Options {
    fn default() -> Self {
        Self {
            trailing_newline: true,
            args: Vec::new(),
        }
    }
}

fn get_args() -> Vec<String> {
    let args: Vec<_> = env::args()
        .skip(1) // Don't include the program name
        .collect();

    if args.is_empty() {
        println!();
        process::exit(0);
    }
    args
}

fn print_help() -> ! {
    println!(
        "\
displays a line of text
usage: echo <options...> [STRING]
options:
    -n              Disable the newline at the end of the output
    -h --help       Display this help text
    -v --version    Print version number"
    );
    process::exit(0);
}

fn print_version() -> ! {
    println!("{}", env!("CARGO_PKG_VERSION"));
    process::exit(0);
}

fn parse_long_arg(_options: &mut Options, flag: &str) {
    let flag = flag
        .strip_prefix("-")
        .expect("Long flag should have `--` prefix");

    match flag {
        "help" => print_help(),
        "version" => print_version(),
        _ => (),
    }
}

fn parse_args(mut args: Vec<String>) -> Options {
    let flags: Vec<_> = args
        .iter()
        .filter(|arg| {
            arg.chars()
                .next()
                .expect("Argument should have a nonzero length")
                == '-'
        })
        .map(|str| str.strip_prefix('-').expect("Flag should have a -"))
        .collect();

    let mut options = Options::default();

    for flag in flags {
        for c in flag.chars() {
            match c {
                '-' => parse_long_arg(&mut options, flag),
                'h' => print_help(),
                'v' => print_version(),
                'n' => options.trailing_newline = false,
                _ => break,
            }
        }
    }

    if !options.trailing_newline {
        args.remove(0);
    }

    options.args = args;
    options
}

/// Prints the result. Has been abstracted from simple `println!()` to enable testing
///
/// See https://rust-cli.github.io/book/tutorial/testing.html
fn print_content(options: Options, writer: &mut impl Write) {
    let last_index = options.args.len() - 1;
    let write_expect_msg = "Writing should not fail";

    for (i, arg) in options.args.into_iter().enumerate() {
        write!(writer, "{arg}").expect(write_expect_msg);

        // Print a space between arguments
        if i < last_index {
            write!(writer, " ").expect(write_expect_msg);
        }
    }

    if options.trailing_newline {
        writeln!(writer).expect(write_expect_msg);
    }
}

pub fn run() {
    let args = get_args();
    let options = parse_args(args);
    print_content(options, &mut std::io::stdout());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn single_arg_option() {
        let args = vec!["Hello".into()];
        let expected = Options {
            args: vec!["Hello".into()],
            ..Default::default()
        };

        let options = parse_args(args);
        assert_eq!(expected, options)
    }

    #[test]
    fn single_arg_print() {
        let mut results = Vec::new();
        let expected = b"Hello\n";
        let options = Options {
            args: vec!["Hello".into()],
            ..Default::default()
        };

        print_content(options, &mut results);
        assert_eq!(results, expected)
    }

    #[test]
    fn multi_arg_option() {
        let args = vec!["Hello".into(), "World".into()];
        let expected = Options {
            args: vec!["Hello".into(), "World".into()],
            ..Default::default()
        };

        let options = parse_args(args);
        assert_eq!(expected, options)
    }

    #[test]
    fn multi_arg_print() {
        let mut results = Vec::new();
        let expected = b"Hello World\n";
        let options = Options {
            args: vec!["Hello".into(), "World".into()],
            ..Default::default()
        };

        print_content(options, &mut results);
        assert_eq!(results, expected)
    }

    #[test]
    fn single_arg_option_no_newline() {
        let args = vec!["-n".into(), "Hello".into()];
        let expected = Options {
            args: vec!["Hello".into()],
            trailing_newline: false,
        };

        let options = parse_args(args);
        assert_eq!(expected, options)
    }

    #[test]
    fn single_arg_print_no_newline() {
        let mut results = Vec::new();
        let expected = b"Hello";
        let options = Options {
            args: vec!["Hello".into()],
            trailing_newline: false,
        };

        print_content(options, &mut results);
        assert_eq!(results, expected)
    }

    #[test]
    fn multi_arg_option_no_newline() {
        let args = vec!["-n".into(), "Hello".into(), "World".into()];
        let expected = Options {
            args: vec!["Hello".into(), "World".into()],
            trailing_newline: false,
        };

        let options = parse_args(args);
        assert_eq!(expected, options)
    }

    #[test]
    fn multi_arg_print_no_newline() {
        let mut results = Vec::new();
        let expected = b"Hello World";
        let options = Options {
            args: vec!["Hello".into(), "World".into()],
            trailing_newline: false,
        };

        print_content(options, &mut results);
        assert_eq!(results, expected)
    }
}
