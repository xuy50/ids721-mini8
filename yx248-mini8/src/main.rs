use clap::{App, Arg, };
use yx248_mini8::price_filter_cli;
use std::process;

#[tokio::main]
async fn main() {
    let matches = App::new("Price Filter CLI")
        .version("1.0")
        .author("YX")
        .about("Filters products by price range")
        .arg(Arg::with_name("low")
                 .long("low")
                 .value_name("LOW")
                 .help("Low price bound")
                 .takes_value(true)
                 .required(true))
        .arg(Arg::with_name("high")
                 .long("high")
                 .value_name("HIGH")
                 .help("High price bound")
                 .takes_value(true)
                 .required(true))
        .get_matches();

    let low: f64 = matches.value_of("low").unwrap().parse().expect("Low price bound must be a number");
    let high: f64 = matches.value_of("high").unwrap().parse().expect("High price bound must be a number");

    if let Err(e) = price_filter_cli(low, high).await {
        eprintln!("Error: {}", e);
        process::exit(1);
    }
}


