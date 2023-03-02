use std::collections::HashMap;

// [note,note,...] (note or None, one index == one tick)
//   \   
// [string, string]
//            \     
//     [song_part, song_part] (intro, chorus)
//              \  /
//              song

// EXAMPLE
// circle_the_drain_strings = ["E6","A5","D4","G3","B2","E1"]
// circle_the_drain_song_parts = ["intro", "chorus", "outro"]
// circle_the_drain_song = {
//     "intro": {
//     	"E6": ["","-","1", "", "0"],
//     	"A5": ["","-","1", "", "0"],
//     }
//     "chorus": [],
//     "outro": [],
// }

pub struct Song {
    pub string_count: u32,
    pub strings: Vec<String>,
    pub bpm: u32,
    pub song_structure: Vec<String>,
    pub song: HashMap<String, HashMap<String, Vec<String>>>,
}

impl Song {

    pub fn parse_song_file(&mut self, file_contents: String) {
	let mut neck: Vec<String> = vec![];
	let mut current_song_part = String::new();
	for line in file_contents.split_terminator('\n') {
	    if line == " " {
		if neck.len() > 0 {
		    println!("associating");
		    self.associate_song_notes(&mut neck, current_song_part.to_string().to_owned());
		    neck = vec![];
		}
	    }
	    if line.contains('[') {
		current_song_part = line.to_string().replace(&[']', '['],"").to_owned();
		self.song_structure.push(current_song_part.to_owned());
		self.song.insert(current_song_part.to_owned(), HashMap::new());
	    }
	    if line.contains('|') {
		self.append_song_tuning(line[0..2].to_string().replace(&['|', ' '],"").to_owned());
		neck.push(line.to_string().to_owned());
	    }
	}
    }
    
    pub fn append_song_tuning(&mut self, val: String)  {
	if self.strings.len() < self.string_count as usize {
	    self.strings.push(format!("{}{}", val, (self.strings.len() + 1).to_string()))
	}
    }

    pub fn associate_song_notes(&mut self, filled_neck: &mut Vec<String>, song_section: String) {
	for string_iter in 0..filled_neck.len() {
	    let string = filled_neck.get(string_iter).unwrap();
	    let string_tuning = string[0..2].to_string().replace(&['|', ' '],"").to_owned();
	    let mut note_array = Vec::new();
	    for tick in 0..string.len() {
		let mut match_next = true;
		match string.chars().nth(tick) {
		    x if x.unwrap().is_numeric() && match_next => {
			if string.chars().nth(tick+1).unwrap().is_numeric() {
			    match_next = false;
			    let next_num = tick + 1;
			    note_array.push(format!("{}{}", x.unwrap().to_string(), string.chars().nth(next_num).unwrap()));
			    note_array.push("-".to_string());
			} else {
			    note_array.push(x.unwrap().to_string());
			    match_next = true;
			}
		    },
		    Some('-') => {
			note_array.push(string.chars().nth(tick).unwrap().to_string());
		    },
		    Some('|') => {
			// we will seperate these into bars evenutally
			note_array.push(string.chars().nth(tick).unwrap().to_string());
		    },
		    _ => {println!("NA")},
		};
	    }
	    if self.song.contains_key(&song_section) {
		let map2 = self.song.get_mut(&song_section).unwrap();
		map2.insert(format!("{}{}", string_tuning, (string_iter + 1).to_string()), note_array.to_owned());
	    } else {
		println!("you effed this up, there was no key!");
	    }
	}
    }

    pub fn display_attrs(&self) {
	println!("String Count: {:?}", self.string_count);
	println!("Strings: {:?}", self.strings);
	println!("BPM: {:?}", self.bpm);
	println!("Song Structure: {:?}", self.song_structure);
	println!("Song: {:?}", self.song);
    }
    
}
