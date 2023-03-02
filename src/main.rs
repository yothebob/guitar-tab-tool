mod parser;

use parser::{ Song };
use std::collections::HashMap;
use std::fs;


fn main() {
    let contents = fs::read_to_string("test.txt")
	.expect("Should have been able to read the file");
    let mut song_obj = Song{
	string_count: 6,
	strings: Vec::new(),
	bpm: 140,
	song_structure: Vec::new(),
	song: HashMap::new(),
    };
    song_obj.parse_song_file(contents.to_owned());
    song_obj.display_attrs();

}
