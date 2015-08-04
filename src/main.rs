extern crate time;

use std::cmp::Ordering;
use std::fs::File;
use std::fs::OpenOptions;
use std::collections::LinkedList;
use std::io::BufReader;
use std::io::BufRead;
use std::io::Write;
use std::fmt::Display;

#[derive(Debug)]
struct Item {
	name: String,
	date: String,
	category: String 
}

impl Item { 

	fn new(name: &str,
	date: &str,
	category: &str) -> Item {

		Item {
			name: name.to_string(),
			date: date.to_string(),
			category: category.to_string() }
	}
}

impl PartialEq for Item {

	fn eq(&self, other: &Item) -> bool {
		self.name == other.name
			&& self.date == other.date
			&& self.category == self.category
	}

	fn ne(&self, other: &Item) -> bool { !self.eq(other) }
}

impl PartialOrd for Item {

	fn partial_cmp(&self, other: &Item) -> Option<Ordering> {
		self.date.partial_cmp(&other.date)
	}
}

impl Ord for Item {

	fn cmp(&self, other:&Item) -> Ordering {
		self.date.cmp(&other.date)
	}
}

impl Eq for Item {}

impl ToString for Item {

	fn to_string(&self) -> String {

		format!("{},{},{}",
		self.name,
		self.date,
		self.category)
	}
} 

fn add_item(input: &[String]) {
	
	let len = input.len();

	if input.len() < 2 {
		print_usage();
		//println!("{}", "error");
		return;	
	}

	let mut db = load();
	let date = time::now();
	let category = match len {
		2 => input[1].clone(),
		_ => "none".to_string(),
	};

	db.push_back(Item { name: input[0].clone(),
			    date: date.rfc3339().to_string(),
			    category: category });
	store(db);
} 

fn load() -> LinkedList<Item> {

	let mut db = LinkedList::new();
	let file = 
		match File::open("sample.cv") {
			Ok(file) => file,
			Err(..) => match File::create("sample.cv") {
				Ok(file) => file,
				Err(..) => panic!("File couldn't be generated!!"),
			}
		};

	for line in BufReader::new(&file).lines() {

		let raw_string = match line {
			Ok(content) => content,
			Err(..) => panic!("Read Line Error has occured!"),
		};

		let mut strmap = raw_string.split(",").map(|str| str.to_string());
		db.push_back(Item { name: strmap.nth(0).unwrap(),
				    date: strmap.nth(0).unwrap(),
				    category: strmap.nth(0).unwrap()}); 
	}

	db
}

fn store(db: LinkedList<Item>) { 

	let mut file = 
		OpenOptions::new().write(true).open("sample.cv").unwrap(); 

	for item in db {
		file.write(item.to_string().as_bytes());
	}
} 

fn print_usage() {

	println!("Usage:");
	println!("  simdb cmd [categoryName]");
	println!("  add     add item, followed by optional category");
	println!("{}{}",
		 "  latest  print last added item(s), followed by ",
           	 "optional category");
	println!("  all     print all");
	println!("{}{}", 
		 "  For instance: add \"some item name\" ",
		 "\"some category name\"");
}

fn print_latest(input: &[String]) {

	let db = load();
	if db.is_empty() {
		println!("No entries in the database.");
		return;
	}
	
	let mut vec: Vec<_> = db.into_iter().collect();
	vec.sort();

	if input.len() == 1 {

		for item in vec {
			if item.category == input[0] {
				println!("{}", item.to_string());
			}
		}
	} else {
		println!("{}", vec.get(0).unwrap().to_string());
	}
} 

fn print_all() {

	let db = load();
	if db.is_empty() {
		println!("No entries in the database.");
		return;
	}

	let mut vec: Vec<_> = db.into_iter().collect();
	vec.sort();

	for item in vec { 
		println!("{}", item.to_string());
	}
}	

fn  main() {

	let argv: Vec<String> = std::env::args().collect();
	let (_, args) = argv.split_at(2);
	let add = "add".to_string();
	let latest = "latest".to_string();
	let all = "all".to_string();

	match argv.get(1) {

		Some(command) => 
			if command == "add" { add_item(args); } 
			else if command == "latest" { print_latest(args); }
			else if command == "all" { print_all(); }
			else { print_usage(); }, 

		_ => print_usage(),
	} 
}
