extern crate clap;
extern crate reqwest;

use std::time::Instant;

use clap::{App, Arg, ArgMatches};
use std::fs::File;
use std::io::Read;

fn main() {
    let matches = App::new("rttp")
        .version("1.0")
        .about("Get and put")
        .author("Erik Z.")
        .arg(
            Arg::with_name("file")
                .short("f")
                .value_name("FILE")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("METHOD")
                .help("HTTP Method to use")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::with_name("URL")
                .help("The URL to talk to")
                .required(true)
                .index(2),
        )
        .get_matches();

    let url = matches.value_of("URL").unwrap();

    let file_name = matches.value_of("file").unwrap_or("");
    println!("File name is {}", file_name);

    if file_name.len() == 0 {
        panic!("No file name")
    }

    let client = reqwest::ClientBuilder::new()
        .gzip(true)
        .tcp_nodelay()
        .build()
        .unwrap();

    let count = 5;
    let mut sum: u128 = 0;

    let first = run(&client, &matches, url, file_name);
    println!("First run {}", first);

    for _ in 0..count {
        let millis = run(&client, &matches, url, file_name);
        sum += millis;
        println!("Done {}", millis)
    }

    let seconds = (sum as f64 / count as f64) / 1000.0;
    let avg = 5.0 * 8.0 / seconds;

    println!("Avg {}", avg)
}

fn run(client: &reqwest::Client, matches: &ArgMatches, url: &str, file_name: &str) -> u128 {
    let c = match matches.value_of("METHOD").unwrap_or("") {
        "get" => client.get(url),
        "put" => {
            let mut f = File::open(file_name).unwrap();
            let mut buffer = Vec::new();
            f.read_to_end(&mut buffer).unwrap();

            client.put(url).body(buffer)
        }
        _ => panic!("No such method"),
    };

    let before = Instant::now();
    c.send().unwrap();
    let after = Instant::now();
    let d = after.duration_since(before);
    let millis = d.as_millis();

    return millis;
}
