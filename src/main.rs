extern crate clap;
use clap::{Arg, App};
use std::io;
use std::io::prelude::*;
use std::collections::BTreeMap;

fn get_min(map: &BTreeMap<i32,String>)-> i32{
	let ret = match map.iter().next() {
		Some((&key,_value)) => key,
		None => panic!("{:?}", 1)
	};
	ret
}

fn get_max(map: &BTreeMap<i32,String>)-> i32{
	let ret = match map.iter().next_back() {
		Some((&key,_value)) => key,
		None => panic!("{:?}", 1)
	};
	ret
}

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
                               .help("Reverse sort")
                               .takes_value(true))
                          .arg(Arg::with_name("results")
                          	   .short("n")
                               .help("Number of results to return")
                               .takes_value(true))
                          .get_matches();

    // Gets a value for config if supplied by user, or defaults to "default.conf"
    let delimiter = matches.value_of("delimiter").unwrap_or(",");
    let field_number = matches.value_of("field").unwrap_or("0").parse::<usize>().unwrap();
    let keep_results = matches.value_of("results").unwrap_or("10").parse::<usize>().unwrap();
    let mut results : BTreeMap<i32,String> = BTreeMap::new();
	let stdin = io::stdin();
	for line in stdin.lock().lines() {
		let actual_line = line.unwrap();
		let field = match actual_line.split(delimiter).nth(field_number){
			Some(n) => n.parse::<i32>().unwrap(),
			None => continue
		};
		results.insert(field,actual_line);	
		if results.len() > keep_results {	
			let min = get_min(&results);
			results.remove(&min);
		}
	}
	for (_key,value) in results{
		println!("{:?}", value);	
	}
	
    
}