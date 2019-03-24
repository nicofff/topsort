extern crate clap;
extern crate csv;
use clap::{App, Arg};
use std::io;
mod topsort;
use topsort::top_sort::{OrderType, TopSort, TopSortEntry};

fn main() {
     let matches = App::new("TopSort")
          .version("0.1")
          .author("Nicolas Far <nfar@gmail.com>")
          .about("Combine sort | tail -n in a single command")
          .arg(Arg::with_name("delimiter")
               .short("t")
               .help("Delimiter to break fields by")
               .takes_value(true))
          .arg(Arg::with_name("field")
               .short("k")
               .help("Number of field to sort by")
               .takes_value(true))
          .arg(Arg::with_name("reverse").short("r").help("Reverse sort"))
          .arg(Arg::with_name("results")
               .short("n")
               .help("Number of results to return")
               .takes_value(true))
          .arg(Arg::with_name("parallel")
               .help("Number of threads to use")
               .takes_value(true))
          .get_matches();

     let delimiter = matches.value_of("delimiter").unwrap_or(",");
     let field_number = matches
          .value_of("field")
          .unwrap_or("0")
          .parse::<usize>()
          .unwrap()
          - 1;
     let keep_results = matches
          .value_of("results")
          .unwrap_or("10")
          .parse::<usize>()
          .unwrap();

     let ordering = if matches.is_present("reverse") {
          OrderType::REVERSE
     } else {
          OrderType::DEFAULT
     };

     let mut rdr = csv::ReaderBuilder::new()
          .has_headers(false)
          .delimiter(delimiter.chars().nth(0).unwrap() as u8)
          .from_reader(io::stdin());
     let mut top_sort = TopSort::new(ordering, keep_results);

     for result in rdr.records() {
          if let Ok(record) = result{
               let entry = TopSortEntry::new(record.get(field_number).unwrap(), &record).unwrap();
               top_sort.add(entry);
          }
     }
     for value in top_sort.get_result() {
          let string_vec : Vec<&str> = value.iter().collect();
          println!("{}", string_vec.join(delimiter));
     }

}
