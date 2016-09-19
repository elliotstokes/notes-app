use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

const NOTES_FOLDER : &'static str = "./notes";

pub struct Manager {
    pub notes: Vec<String>
}

impl Manager {
    pub fn new() -> Manager {
        get_notes();
        Manager {
            notes: get_notes()
        }
    }

    pub fn save(&self) {
        save_notes(&self.notes);
    }
}

fn save_notes(notes: &Vec<String>) {
    for (i, note) in notes.iter().enumerate() {
        let file_name = format!("{}/{}.not", NOTES_FOLDER, i);
        let path = Path::new(&file_name);
        let mut file = fs::File::create(&path).unwrap();
        file.write_all(note.as_bytes()).unwrap();
    }
}

fn get_notes() -> Vec<String> {
    let mut note_text = Vec::<String>::new();
    let paths = fs::read_dir(NOTES_FOLDER).unwrap();
    for dir_entry in paths {
        let mut file = File::open(&dir_entry.unwrap().path()).unwrap();

        let mut s = String::new();
        file.read_to_string(&mut s).unwrap();

        note_text.push(s);
    }

    return note_text;
}
