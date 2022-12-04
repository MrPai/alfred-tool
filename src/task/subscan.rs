use std::{env, fs::File};
 use alfred_primitives::types::*;
 use crate::types::ss58_registry::*;

 pub fn parse_for_subscan_url(json_path: &str, output_path: &str){
   println!("parse_for_subscan_url");
	let file = File::open(json_path).unwrap();
	let summary: Summary = serde_json::from_reader(file).unwrap();

	println!("summary: {:?}", summary);
 }