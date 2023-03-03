use crate::parser::Song;
use std::collections::HashMap;
// use rand::Rng;

pub fn generate_song(song_list: Vec<Song>) -> Song {
    println!("generating a new song");
    let mut new_song = Song{
	string_count: 0,
	strings: Vec::new(),
	bpm: 140,
	song_structure: Vec::new(),
	song: HashMap::new(),
    };
    let mut max_string_count = 0;
    let mut bpms: Vec<&u32> = Vec::new();
    let mut tunings: Vec<&Vec<String>> = Vec::new();
    for song in song_list.iter() {
	if song.string_count > max_string_count { max_string_count = song.string_count; }
	bpms.push(&song.bpm);
	tunings.push(&song.strings);
	
	// generate song sections based off what given songs have.
	// then for each part in the songs, go though all strings at once for each song in a given section.
	// compare the notes played per tick, randomly pick from one of the song choices, or neither
	// print that result into the song section
	println!("song here")
    }
    bpms.sort();
    println!("{:?}", bpms);
    // println!("{:?}", rng.gen_range(bpms[0],bpms[(bpms.len() -1 )]));
    new_song.string_count = max_string_count;
    // new_song.display_attrs();
    new_song
}


