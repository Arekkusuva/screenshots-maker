use clap::{App, Arg};

mod maker;
use maker::Maker;

use std::thread;
use std::fs;
use std::time::Duration;
use std::path::PathBuf;

static DEFAULT_PATH: &'static str = "$HOME/Screenshots";

#[inline]
fn get_default_path() -> PathBuf {
    dirs::home_dir().unwrap().join("Screenshots")
}

fn main() {
    let matches = App::new("screenshots-maker")
        .version("1.0.0")
        .about("Takes screenshots at regular intervals")
        .arg(Arg::with_name("output")
            .value_name("OUTPUT_DIRECTORY")
            .short("o")
            .long("output")
            .help("Sets path to output directory")
            .takes_value(true)
            .default_value(DEFAULT_PATH))
        .arg(Arg::with_name("interval")
            .value_name("SECONDS")
            .short("i")
            .long("interval")
            .help("Sets interval")
            .takes_value(true)
            .default_value("540"))
        .arg(Arg::with_name("datetime_format")
            .value_name("DATE_AND_TIME_FORMAT")
            .long("datetime-format")
            .help("Sets date and time format for files names")
            .takes_value(true)
            .default_value("%Y-%m-%d_%H-%M-%S"))
        .get_matches();

    let output_dir = match matches.value_of("output") {
        Some(r ) => if r == DEFAULT_PATH { get_default_path() } else { PathBuf::from(r) },
        None => get_default_path(),
    };
    fs::create_dir_all(&output_dir)
        .expect("Couldn't create output path");
    let interval: u64 = matches.value_of("interval").unwrap()
        .parse().unwrap();
    let dt_format = matches.value_of("datetime_format").unwrap()
        .to_string();

    println!("Running screenshots-maker v1.0.0");
    println!("- Output directory: {}", output_dir.display());
    if interval > 60 {
        println!("- Interval: {} minutes", interval / 60);
    } else {
        println!("- Interval: {} seconds", interval);
    }
    println!("- Date format for file name: {}", &dt_format);

    println!("Screenshots taking");
    let mk = Maker::with_path_generator(move || {
        let file_name = chrono::Local::now()
            .format(&dt_format)
            .to_string();
        output_dir.join(file_name)
    });

    let mut i: u64 = 0;
    // TODO: Improve accuracy
    loop {
        let path = mk.take();
        i += 1;
        println!("#{} saved to {}", i, path.display());
        thread::sleep(Duration::new(interval, 0));
    }
}
