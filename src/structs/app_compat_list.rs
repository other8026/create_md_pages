use crate::structs::app_compat_app::AppCompatApp;
use std::fs::{read_dir, File};
use std::path::PathBuf;

pub struct AppCompatList(Vec<AppCompatApp>);

const FAKE_FIRST_CHAR: char = 'z';

const DIV_START_STRING: &'static str = "{{ app_compat_div_start() }}";
const DIV_END_STRING: &'static str = "{{ app_compat_div_end() }}";

impl AppCompatList {
    pub fn new_from_folder(folder: PathBuf) -> Result<Self, String> {
        // get directory contents
        let dir = read_dir(folder).map_err(|e| format!("unable to read the folder: {}", e))?;

        // this will be the app list
        let list = dir
            .filter_map(|file| {
                let f = file.expect("error getting file from directory list").path();

                let filename = f
                    .file_name()
                    .expect("cannot read file's file name")
                    .to_str()
                    .expect("cannot convert filename to string");

                // skip the template (any file with a filename that starts with `_`,
                // directories, any dot files, and any files that aren't yaml files
                if f.is_dir()
                    || !(filename.ends_with(".yaml") || filename.ends_with(".yml"))
                    || filename.starts_with(".")
                    || filename.starts_with("_")
                {
                    println!("Skipping {}", filename);
                    return None;
                }

                let file = File::open(f).expect("error opening file");

                let app: AppCompatApp = serde_yaml::from_reader(file)
                    .expect("there was an error deserializing the file, so panicking");
                Some(app)
            })
            .collect::<Vec<AppCompatApp>>();

        Ok(Self(list))
    }

    pub fn sort_list(&mut self) {
        self.0
            .sort_by(|a, b| a.app_name.to_lowercase().cmp(&b.app_name.to_lowercase()));
    }

    pub fn print_cards_list(&self) -> String {
        // keep a list of all the letters that app names start with
        let mut contents_list = vec![];

        // keep a list of strings to print (joined by \n)
        let mut strings_list = vec![];

        // the list should already be sorted, so just assuming that to make it easier
        // 'Z' won't be the first, so just use this first
        let mut last: char = FAKE_FIRST_CHAR;

        for app in self.0.iter() {
            // check if the letter is the same as `last`
            // if not, then add that
            // this should be used so I can make a sort of TOC to link to
            let letter = app.get_name_first_char();
            if letter != last {
                if letter != FAKE_FIRST_CHAR {
                    // end the previous div
                    strings_list.push(DIV_END_STRING.to_string());
                }
                contents_list.push(letter);
                strings_list.push(format!("\n# {}\n", letter));
                last = letter;

                // new div should start after the new letter is added
                strings_list.push(DIV_START_STRING.to_string());
            }

            // checked the first letter, so add this thing to the strings list
            strings_list.push(app.print_card_line());
        }

        strings_list.push(DIV_END_STRING.to_string());

        // print the strings list with line breaks between
        strings_list.join("\n")
    }
}
