mod parser;
mod writer;

use parser::{ Song };
use writer::{ generate_song };

use std::collections::HashMap;
use std::fs;


fn main() {
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
}
