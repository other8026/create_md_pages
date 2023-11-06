use crate::structs::app_compat_app::AppCompatApp;
use std::fmt;
use std::fmt::Display;
use std::fs::{read_dir, File};
use std::path::PathBuf;

pub struct AppCompatList(Vec<AppCompatApp>);

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

                // skip the template, directories, any dot files, and any files that aren't yaml files
                if filename == "_template.yaml"
                    || f.is_dir()
                    || !filename.ends_with(".yaml")
                    || filename.starts_with(".")
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
        self.0.sort_by(|a, b| a.app_name.to_lowercase().cmp(&b.app_name.to_lowercase()));
    }
}

impl Display for AppCompatList {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let app_list = self.0
            .iter()
            .map(|app| format!("{}", app))
            .collect::<Vec<String>>()
            .join("\n");

        let full_table = format!("|App Name|Package Name|Status|Requires GMS|Requires Installed by Play|\n|---|---|---|---|---|\n{}", app_list);

        write!(
            f,
            "{}",
            full_table
        )
    }
}
