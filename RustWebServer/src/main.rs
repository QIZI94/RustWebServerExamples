#![allow(non_snake_case)]
#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;
extern crate serde_json;
extern crate serde;


use rocket::response::content;
use rocket::State;
use std::sync::Mutex;
use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize)]
struct Person{
	name: String,
	age: u8,
	height: f32
}

type PersonList = Vec<Person>;
type SafePersonList = Mutex<PersonList>;

#[get("/")]
fn index() -> content::Html<String>  {
	
	use std::fs;
	let html = fs::read_to_string("src/index.html")
		.expect("Unable to read file");
    return content::Html(html);
}

#[get("/<name>/<age>/<height>")]
fn AddToList(name: String, age: u8, height: f32, state: State<SafePersonList>) -> String {
	let list = &mut state.inner().lock().unwrap();
	list.push(Person { name: name, age: age, height: height });

	let listRef =&**list; 
	return serde_json::to_string(listRef).unwrap();
}

#[get("/list")]
fn ReadFromList(state: State<SafePersonList>) -> String {
	let list = state.inner().lock().unwrap();
	
	let listRef =&*list; 
	return serde_json::to_string(listRef).unwrap();
}

fn main() {
	let list: SafePersonList = SafePersonList::new(vec![]);

    rocket::ignite()
		.mount("/", routes![index, AddToList, ReadFromList])
		.manage(list)
		.launch();	
}