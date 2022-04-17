use clap::{App, Arg};
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    number_lines: bool,
    number_nonblank_lines: bool,
}

type MyResult<T> = Result<T, Box<dyn Error>>;

pub fn get_args() -> MyResult<Config> {
    let matches = App::new("catr")
        .version("0.1.0")
        .author("John Doe")
        .about("Rust cat")
        .arg(
            Arg::with_name("files")
            .value_name("FILE")
            .help("input file(s)")
            .multiple(true)
            .default_value("-")
        )
        .arg(
            Arg::with_name("number")
            .short("n")
            .long("number")
            .help("Number lines")
            .takes_value(false)
        )
        .arg(
            Arg::with_name("number_nonblank")
            .short("b")
            .long("number-nonblank")
            .help("Number non-blank lines")
            .takes_value(false)
            .conflicts_with("number")
        )
        .get_matches();

    Ok(Config {
        files: matches.values_of_lossy("files").unwrap(),
        number_lines: matches.is_present("number"),
        number_nonblank_lines: matches.is_present("number_nonblank"),
    })
}

pub fn run(config: Config) -> MyResult<()> {
    //dbg!(config);
    for filename in config.files {
        //println!("{}", filename);
        match open(&filename) {
            Err(err) => eprintln!("{}: {}", filename, err),
            Ok(bufreader) => {
                //println!("Opened {}", filename);
                let mut numline = 0;
                for (i, line_result) in bufreader.lines().enumerate() {
                    let line = line_result?;
                    if config.number_lines {
                        print!("{:>6}\t", i + 1);
                    } else if config.number_nonblank_lines {
                        if !line.is_empty() {
                            numline += 1;
                            print!("{:>6}\t", numline);
                        }
                    }
                    println!("{}", line);
                }
            },
        }
    }
    Ok(())
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?)))
    }
}


