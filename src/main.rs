extern crate clap;
use clap::{Arg, App};
use std::io;
use std::io::prelude::*;
mod topsort;
use topsort::{TopSort,OrderType};


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
                          .arg(Arg::with_name("reverse")
                          	   .short("r")
                               .help("Reverse sort"))
                          .arg(Arg::with_name("results")
                          	   .short("n")
                               .help("Number of results to return")
                               .takes_value(true))
                          .arg(Arg::with_name("parallel")
                               .help("Number of threads to use")
                               .takes_value(true))
                          .get_matches();

    let delimiter = matches.value_of("delimiter").unwrap_or(",");
    let field_number = matches.value_of("field").unwrap_or("0").parse::<usize>().unwrap() - 1;
    let keep_results = matches.value_of("results").unwrap_or("10").parse::<usize>().unwrap();

    let ordering = if matches.is_present("reverse") {
    	OrderType::REVERSE 
    } else {
    	OrderType::DEFAULT
    };

    let mut top_sort = TopSort::new(ordering,keep_results);
	let stdin = io::stdin();
	for line in stdin.lock().lines() {
		let actual_line = line.unwrap();
		let field = match actual_line.split(delimiter).nth(field_number){
			Some(n) => n,
			None => continue
		};
		top_sort.add(field,&actual_line);	
		
	}
	for value in top_sort.get_result() {
		println!("{}", value);
	}
	
	
    
}