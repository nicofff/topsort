extern crate clap;
use clap::{Arg, App};
use std::io;
use std::io::prelude::*;
mod top_sort;
mod decimal;
use decimal::Decimal;
use top_sort::MBTreeMap;

enum OrderType {
	DEFAULT,
	REVERSE
}

fn trim_tree(tree: MBTreeMap<Decimal,String>, to_size: usize, ordering: &OrderType) -> MBTreeMap<Decimal,String>{
	match ordering {
	    OrderType::DEFAULT => trim_tree_top(tree,to_size),
	    OrderType::REVERSE => trim_tree_bottom(tree, to_size),
	}
}

fn trim_tree_top(mut tree: MBTreeMap<Decimal,String>, to_size: usize) -> MBTreeMap<Decimal,String>{
	if to_size > tree.len(){
		return tree
	}
	let ammount_to_prune = tree.len() - to_size;
	let (&splitter,_) = tree.iter().nth(ammount_to_prune).unwrap();
	let tree_top = tree.split_off(&splitter);
	assert_eq!(tree_top.len(),to_size);
	tree_top
}

fn trim_tree_bottom(mut tree:  MBTreeMap<Decimal,String>, to_size: usize) -> MBTreeMap<Decimal,String>{
	if to_size > tree.len(){
		return tree
	}
	let ammount_to_prune = tree.len() - to_size;
	let (&splitter,_) = tree.iter().rev().nth(ammount_to_prune-1).unwrap();
	let _tree_top = tree.split_off(&splitter);
	assert_eq!(tree.len(),to_size);
	tree
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
                               .help("Reverse sort"))
                          .arg(Arg::with_name("results")
                          	   .short("n")
                               .help("Number of results to return")
                               .takes_value(true))
                          .get_matches();

    let trim_ratio = 2;
    let delimiter = matches.value_of("delimiter").unwrap_or(",");
    let field_number = matches.value_of("field").unwrap_or("0").parse::<usize>().unwrap() - 1;
    let keep_results = matches.value_of("results").unwrap_or("10").parse::<usize>().unwrap();
    let ordering = match matches.is_present("reverse") {
    	true => OrderType::REVERSE,
    	false => OrderType::DEFAULT
    };
    let mut results : MBTreeMap<Decimal,String> = MBTreeMap::new();
	let stdin = io::stdin();
	for line in stdin.lock().lines() {
		let actual_line = line.unwrap();
		let field = match actual_line.split(delimiter).nth(field_number){
			Some(n) => n.parse::<Decimal>().unwrap(),
			None => continue
		};
		results.insert(field,actual_line);	
		if results.len() > keep_results * trim_ratio {	
			results = trim_tree(results,keep_results+1,&ordering);
		}
	}
	let results: Vec<String> = trim_tree(results,keep_results,&ordering).flatten();
	let mut extra_results = 0;
	if results.len() > keep_results {
		extra_results = results.len() - keep_results;
	}
	
	match ordering {
	    OrderType::DEFAULT => {
	    	for value in results.iter().skip(extra_results){
				println!("{}", value);	
			}
	    },
	    OrderType::REVERSE => {
	    	for value in results.iter().rev().skip(extra_results){
				println!("{}", value);	
			}
	    }
	};
	
	
    
}