mod stdin_functions;
mod structs;

use crate::structs::app_compat_app::AppCompatApp;
use crate::structs::app_compat_list::AppCompatList;
use crate::structs::app_output_md_file_config::AppOutputMdFileConfig;
use crate::structs::command_line_opts::CommandLineOpts;
use std::fs::File;
use std::io::Write;
use structopt::StructOpt;

fn main() -> Result<(), String> {
    let opt = CommandLineOpts::from_args();

    if opt.add {
        // first make sure the required files/folders are valid
        if !&opt.app_files_folder.is_dir() {
            return Err("A file or folder is not valid. Exiting".to_string());
        }

        // get info from user to make new csv entry
        let new_app = AppCompatApp::new_from_command_line().map_err(|e| e.to_string())?;

        // save the file
        new_app.save_to_file(&mut opt.app_files_folder.clone())?;

        Ok(())
    } else if opt.run {
        // check to make sure folders and files are valid
        if !opt.yaml_file.clone().is_some_and(|file| file.is_file())
            || !opt.clone().app_files_folder.is_dir()
            || !opt.clone().output_md_file.is_some()
        {
            return Err("Cannot run due to one of the paths provided not being valid. Please check your paths and try again.".to_string());
        }

        // get the data from the yaml file
        let yaml_file = File::open(&opt.yaml_file.unwrap()).map_err(|e| e.to_string())?;
        let yaml_file: AppOutputMdFileConfig =
            serde_yaml::from_reader(yaml_file).map_err(|e| e.to_string())?;

        let mut apps = AppCompatList::new_from_folder(opt.app_files_folder)?;

        // sorting apps here because they only need to be sorted if printing something,
        // otherwise it doesn't matter if they're out of order during a simple check
        apps.sort_list();

        // create the file
        let mut md_file = File::create(opt.output_md_file.unwrap()).map_err(|e| e.to_string())?;

        // write the whole file
        md_file
            .write_all(
                format!(
                    "+++\ntitle = \"{}\"\ndescription = \"{}\"\n\n[extra]\n\nrelated = []\n+++\n{}\n{}\n{}\n\n{}",
                    yaml_file.title,
                    yaml_file.description,
                    option_to_string_or_empty(yaml_file.before_text),
                    apps.print_md_toc_wrapped_in_div(),
                    apps.print_cards_list(),
                    option_to_string_or_empty(yaml_file.after_text)
                )
                    .as_bytes(),
            )
            .map_err(|e| e.to_string())?;

        Ok(())
    } else {
        Err(
            "You must pick either run or check. Run with --help for command line options."
                .to_string(),
        )
    }
}

// this is only for when printing the final md file
// like if the config yaml file has an empty before or after section
fn option_to_string_or_empty(thing: Option<String>) -> String {
    if let Some(text) = thing {
        text
    } else {
        "".to_string()
    }
}
