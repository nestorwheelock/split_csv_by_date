use chrono::{DateTime, NaiveDateTime, TimeZone};
use chrono_tz::Tz;
use clap::{App, Arg};
use csv::{ReaderBuilder, WriterBuilder};
use std::collections::HashMap;
use std::fs::File;

fn parse_date(date_str: &str) -> Option<DateTime<Tz>> {
    let naive = NaiveDateTime::parse_from_str(date_str, "%m/%d/%y, %I:%M:%S %p").ok()?;
    Some(Tz::America__Chicago.from_utc_datetime(&naive))
}

fn split_csv_by_date(file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut rdr = ReaderBuilder::new().from_path(file_path)?;
    let headers = rdr.headers()?.clone();

    let mut writers: HashMap<String, csv::Writer<File>> = HashMap::new();
    for result in rdr.records() {
        let record = result?;
        let date_str = &record[0]; // Assuming date is in the first column
        if let Some(date) = parse_date(date_str) {
            let formatted_date = date.format("%m-%d-%Y").to_string();
            let output_filename = format!("{}_{}.csv", file_path.trim_end_matches(".csv"), formatted_date);

            let writer = writers.entry(formatted_date.clone()).or_insert_with(|| {
                let file = File::create(&output_filename).expect("Cannot create file");
                let mut writer = WriterBuilder::new().from_writer(file);
                writer.write_record(&headers).expect("Cannot write headers");
                writer
            });
            writer.write_record(&record)?;
        }
    }
    Ok(())
}

fn main() {
    let matches = App::new("CSV Splitter")
        .version("0.1.0")
        .author("Your Name <your.email@example.com>")
        .about("Splits CSV files into multiple files based on dates")
        .arg(Arg::with_name("input")
             .help("The input CSV file path")
             .required(true)
             .index(1))
        .get_matches();

    let file_path = matches.value_of("input").unwrap();

    if let Err(e) = split_csv_by_date(file_path) {
        eprintln!("Error: {}", e);
    }
}

