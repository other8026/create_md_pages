use crate::structs::app_compat_app::AppCompatApp;
use std::fs::{read_dir, File};
use std::path::PathBuf;

pub struct AppCompatList {
    apps: Vec<AppCompatApp>,
    toc: Vec<String>,
}

const FAKE_FIRST_CHAR: char = 'z';

const DIV_START_STRING: &'static str =
    "{{ raw_html( html = \"<div class='app-compat-list-section'>\") }}";
const DIV_END_STRING: &'static str = "{{ raw_html( html = \"</div>\") }}";

const TOC_CLASS: &'static str = "toc";

impl AppCompatList {
    pub fn new_from_folder(folder: PathBuf) -> Result<Self, String> {
        let dir = read_dir(folder).map_err(|e| format!("unable to read the folder: {}", e))?;

        let mut toc = vec![];

        // iterate through the directory scan results
        // filter out filenames
        // grab data from yaml files
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

                let file = File::open(&f).expect("error opening file");

                let mut app: AppCompatApp = serde_yaml::from_reader(file)
                    .expect("there was an error deserializing the file, so panicking");

                let letter: String = app.get_name_first_char().to_lowercase().to_string();
                if !toc.contains(&letter) {
                    toc.push(letter);
                }

                app.replace_double_quotes_from_all_string_fields();
                app.replace_new_lines_with_p_tags();

                Some(app)
            })
            .collect::<Vec<AppCompatApp>>();

        Ok(Self { apps: list, toc })
    }

    pub fn sort_list(&mut self) {
        self.apps
            .sort_by(|a, b| a.app_name.to_lowercase().cmp(&b.app_name.to_lowercase()));
        self.toc.sort();
    }

    pub fn print_cards_list(&self) -> String {
        // this will kind of be like the table of contents
        let mut contents_list = vec![];

        // all strings to print later
        // includes things like `# A` and apps
        let mut strings_list = vec![];

        // the list should already be sorted, so just assuming that to make it easier
        // 'Z' won't be the first, so just using it as a placeholder
        let mut last: char = FAKE_FIRST_CHAR;

        // todo make the <div></div> for groups suck way less
        for app in self.apps.iter() {
            // build the "table of contents" / `contents_list`
            // and at the same time, this first part will open or close `<div>`
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

            // finally add the app entry
            strings_list.push(app.print_card_line());
        }

        // should close the last <div>
        strings_list.push(DIV_END_STRING.to_string());

        // print the strings list with line breaks between
        strings_list.join("\n")
    }

    pub fn print_md_toc(&self) -> String {
        self.toc
            .iter()
            .map(|letter| format!("[{}](#{})", letter.to_uppercase(), letter))
            .collect::<Vec<String>>()
            .join(" | ")
    }

    pub fn print_md_toc_wrapped_in_div(&self) -> String {
        format!("{{{{ raw_html( html = \"<div class='{}'>\") }}}}\n{}\n{{{{ raw_html( html = \"</div>\") }}}}",
                TOC_CLASS,
            self.print_md_toc()
        )
    }
}
