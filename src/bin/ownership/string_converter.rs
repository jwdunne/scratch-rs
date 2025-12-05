use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader, BufWriter, Write};

#[derive(PartialEq, Debug)]
enum Case {
    Upper,
    Lower,
}

#[derive(Debug)]
struct Opts {
    case: Option<Case>,
    output_path: Option<String>,
    input_path: Option<String>,
    reverse: bool,
}

impl Opts {
    fn parse<S: AsRef<str>>(args: &[S]) -> Self {
        let mut opts = Self {
            case: None,
            output_path: None,
            input_path: None,
            reverse: false,
        };

        let mut args = args.iter().peekable();

        while let Some(arg) = args.next() {
            match arg.as_ref() {
                "--upper" => opts.case = Some(Case::Upper),
                "--lower" => opts.case = Some(Case::Lower),
                "--reverse" => opts.reverse = true,
                "--output" => opts.output_path = args.next().map(|s| s.as_ref().to_string()),
                arg if !arg.starts_with('-') => opts.input_path = Some(arg.to_string()),
                _ => {}
            }
        }

        opts
    }
}

fn main() -> io::Result<()> {
    let args: Vec<_> = env::args().skip(1).collect();
    let opts = Opts::parse(&args);

    let reader: Box<dyn BufRead> = match &opts.input_path {
        Some(path) => Box::new(BufReader::new(File::open(path)?)),
        None => Box::new(io::stdin().lock()),
    };

    let mut writer: Box<dyn Write> = match &opts.output_path {
        Some(path) => Box::new(BufWriter::new(File::create(path)?)),
        None => Box::new(io::stdout().lock()),
    };

    for line in reader.lines() {
        let mut line = line?;

        if opts.reverse {
            line = line.chars().rev().collect::<String>();
        }

        line = match opts.case {
            Some(Case::Upper) => line.to_uppercase(),
            Some(Case::Lower) => line.to_lowercase(),
            None => line,
        };

        writeln!(writer, "{}", line)?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_opt_parser_uppercase() {
        let opts = Opts::parse(&["--upper"]);
        assert_eq!(opts.case, Some(Case::Upper));
    }

    #[test]
    fn test_opt_parser_lowercase() {
        let opts = Opts::parse(&["--lower"]);
        assert_eq!(opts.case, Some(Case::Lower));
    }

    #[test]
    fn test_opt_parser_output_path() {
        let opts = Opts::parse(&["--output", "somefile.txt"]);
        assert_eq!(opts.output_path, Some("somefile.txt".to_string()));
    }

    #[test]
    fn test_opt_parser_input_path() {
        let opts = Opts::parse(&["hello.txt"]);
        assert_eq!(opts.input_path, Some("hello.txt".to_string()));
    }
}
