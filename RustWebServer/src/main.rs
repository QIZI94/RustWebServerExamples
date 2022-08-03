#![allow(non_snake_case)]
#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;
extern crate serde_json;
extern crate serde;

use rocket::response::content;
use rocket::response::status;
use rocket::State;
use std::sync::Mutex;

pub type NameList = Vec<String>;
pub type SafeNameList = Mutex<NameList>;

#[get("/")]
fn index() -> content::Html<String>  {
	
	use std::fs;
	let html = fs::read_to_string("src/index.html")
						.expect("Unable to read file");
    return content::Html(html);
}

#[get("/<name>")]
fn AddToList(name: String, state: State<SafeNameList>) -> String {
	let list = &mut state.inner().lock().unwrap();
	list.push(name);

	let listRef =&**list; 
	return serde_json::to_string(listRef).unwrap();
}

#[get("/list")]
fn ReadFromList(state: State<SafeNameList>) -> String {
	let list = state.inner().lock().unwrap();
	
	let listRef =&*list; 
	return serde_json::to_string(listRef).unwrap();
}

fn main() {
	let list: SafeNameList = SafeNameList::new(vec![]);

    rocket::ignite()
		.mount("/", routes![index, AddToList, ReadFromList])
		.manage(list)
		.launch();	
}