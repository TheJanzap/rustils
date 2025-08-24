//! Small port of the cat command using clap derive parsing

use clap::Parser;
use std::{fs, io::Write, ops::Add, path::PathBuf, process};

#[derive(Parser, Default)]
#[command(version, about = "concatenate files and print on the standard output")]
struct Options {
    path: PathBuf,
    #[arg(short = 'E', long = "show-ends")]
    show_ends: bool,
    #[arg(short = 'n', long = "number")]
    number: bool,
}

fn read_file(path: &PathBuf) -> String {
    match fs::read_to_string(path) {
        Ok(content) => content,
        Err(_) => {
            eprintln!("File {} not found", &path.display());
            process::exit(1);
        }
    }
}

fn print_number(content: String, writer: &mut impl Write) {
    // Get the number of digits of the number of lines to properly right-align them for printing
    let digits_of_nof_lines = content
        .chars()
        .filter(|x| *x == '\n')
        .count()
        .checked_ilog10()
        .unwrap_or(0) // Case for no newlines in file, log10 returns Err
        .add(1) as usize;

    for (i, line) in content.lines().enumerate() {
        writeln!(
            writer,
            "{:>width$\t} {line}",
            i + 1,
            width = digits_of_nof_lines
        )
        .expect("Writing should not fail");
    }
}

fn print_content(options: Options, mut content: String, writer: &mut impl Write) {
    if options.show_ends {
        content = content.replace("\n", "$\n");
    }

    if options.number {
        print_number(content, writer);
        return;
    }

    write!(writer, "{content}").expect("Writing should not fail");
}

pub fn run() {
    let options = Options::parse();
    let content = read_file(&options.path);
    print_content(options, content, &mut std::io::stdout());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_file() {
        let content = String::new();
        let options = Options::default();

        let mut result = Vec::new();
        let expected = Vec::new();

        print_content(options, content, &mut result);
        assert_eq!(result, expected);
    }

    #[test]
    fn one_line() {
        let content = String::from("Hello World!");
        let options = Options::default();

        let mut result = Vec::new();
        let expected: Vec<u8> = b"Hello World!".into();

        print_content(options, content, &mut result);
        assert_eq!(result, expected);
    }

    #[test]
    fn two_lines() {
        let content = String::from("Hello World!\nAnd everyone else!");
        let options = Options::default();

        let mut result = Vec::new();
        let expected: Vec<u8> = b"Hello World!\nAnd everyone else!".into();

        print_content(options, content, &mut result);
        assert_eq!(result, expected);
    }

    #[test]
    fn one_line_replace() {
        let content = String::from("Hello World!");
        let options = Options {
            show_ends: true,
            ..Default::default()
        };

        let mut result = Vec::new();
        let expected: Vec<u8> = b"Hello World!".into();

        print_content(options, content, &mut result);
        assert_eq!(result, expected);
    }

    #[test]
    fn two_lines_replace() {
        let content = String::from("Hello World!\nAnd everyone else!");
        let options = Options {
            show_ends: true,
            ..Default::default()
        };

        let mut result = Vec::new();
        let expected: Vec<u8> = b"Hello World!$\nAnd everyone else!".into();

        print_content(options, content, &mut result);
        assert_eq!(result, expected);
    }

    #[test]
    fn one_line_number() {
        let content = String::from("Hello World!");
        let options = Options {
            number: true,
            ..Default::default()
        };

        let mut result = Vec::new();
        let expected: Vec<u8> = b"1 Hello World!\n".into();

        print_content(options, content, &mut result);
        assert_eq!(String::from_utf8(result), String::from_utf8(expected));
    }

    #[test]
    fn ten_lines_number() {
        let content = String::from("One\nTwo\nThree\nFour\nFive\nSix\nSeven\nEight\nNine\nTen");
        let options = Options {
            number: true,
            ..Default::default()
        };

        let mut result = Vec::new();
        let expected: Vec<u8> =
            b"1 One\n2 Two\n3 Three\n4 Four\n5 Five\n6 Six\n7 Seven\n8 Eight\n9 Nine\n10 Ten\n"
                .into();

        print_content(options, content, &mut result);
        assert_eq!(String::from_utf8(result), String::from_utf8(expected));
    }
}
