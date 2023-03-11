mod parser;
mod writer;

use parser::{ Song };
use writer::{ generate_song };

use std::collections::HashMap;
use std::fs;

// use rocket::time::Date;
// use rocket::http::{Status, ContentType};
// use rocket::form::{Form, Contextual, FromForm, FromFormField, Context};
use rocket_dyn_templates::Template;
use rocket::form::{FromForm, FromFormField, Context};

#[macro_use] extern crate rocket_dyn_templates;
#[macro_use] extern crate rocket;

// #[derive(FromFormField)]
// enum Lang {
//     #[field(value = "en")]
//     English,
//     #[field(value = "ru")]
//     #[field(value = "ру")]
//     Russian
// }

// #[derive(FromForm)]
// struct Options<'r> {
//     emoji: bool,
//     name: Option<&'r str>,
// }


// #[get("/world")]
// fn world() -> &'static str {
//     "Hello, world!"
// }

// #[get("/<name>/<age>")]
// fn wave(name: &str, age: u8) -> String {
//     format!("👋 Hello, {} year old named {}!", age, name)
// }


// #[get("/?<lang>&<opt..>")]
// fn hello(lang: Option<Lang>, opt: Options<'_>) -> String {
//     let mut greeting = String::new();
//     if opt.emoji {
//         greeting.push_str("👋 ");
//     }

//     match lang {
//         Some(Lang::Russian) => greeting.push_str("Привет"),
//         Some(Lang::English) => greeting.push_str("Hello"),
//         None => greeting.push_str("Hi"),
//     }

//     if let Some(name) = opt.name {
//         greeting.push_str(", ");
//         greeting.push_str(name);
//     }

//     greeting.push('!');
//     greeting
// }

#[get("/")]
fn index() -> Template {
    Template::render("index", &Context::default())
}

#[launch]
fn rocket() -> _ {
    let contents = fs::read_to_string("test.txt")
	.expect("Should have been able to read the file");
    let contents2 = fs::read_to_string("test2.txt")
	.expect("Should have been able to read the file");
    let mut song_obj = Song{
	string_count: 6,
	strings: Vec::new(),
	bpm: 140,
	song_structure: Vec::new(),
	song: HashMap::new(),
    };
    let mut song_obj2 = Song{
	string_count: 6,
	strings: Vec::new(),
	bpm: 140,
	song_structure: Vec::new(),
	song: HashMap::new(),
    };
    song_obj.parse_song_file(contents.to_owned());
    song_obj.display_attrs();
    song_obj2.parse_song_file(contents2.to_owned());
    song_obj2.display_attrs();
    let song_list = vec![song_obj, song_obj2];
    generate_song(song_list);
    
    rocket::build()
        .mount("/", routes![index])
	.attach(Template::fairing())
}
