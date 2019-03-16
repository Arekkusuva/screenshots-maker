use clap::{App, Arg};

mod maker;
use maker::Maker;

use std::thread;
use std::time::Duration;
use std::path::PathBuf;
use std::fs;

fn main() {
    let matches = App::new("screenshots-maker")
        .version("1.0.0")
        .about("Takes periodic screenshots")
        .arg(Arg::with_name("output")
            .value_name("OUTPUT_DIRECTORY")
            .short("o")
            .long("output")
            .help("Sets path to output directory")
            .takes_value(true)
            .required(true))
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

//    let home_dir = env::home_dir().unwrap();
    let output_dir = PathBuf::from(
        matches.value_of("output").unwrap(),
    );
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
