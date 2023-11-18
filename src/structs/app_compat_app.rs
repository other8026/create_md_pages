use crate::stdin_functions::get_option_string_from_user::get_option_string_from_user;
use crate::stdin_functions::{
    get_bool_from_user::get_bool_from_user, get_option_bool_from_user::get_option_bool_from_user,
    get_string_from_user::get_string_from_user,
};
use crate::structs::bool_or_none::BoolOrNone;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::path::PathBuf;

#[derive(Debug, Deserialize, Serialize)]
pub struct AppCompatApp {
    pub app_name: String,
    pub package_name: String,
    pub works: bool,
    pub works_without_compat_mode: bool,
    pub works_without_gms: BoolOrNone,
    pub works_installed_by_any_source: BoolOrNone,
    pub comment: Option<String>,
}

impl AppCompatApp {
    // call this to create a new struct from user input from stdin
    // (this will be used to create the yaml files)
    pub fn new_from_command_line() -> Result<Self, String> {
        // get the pretty name
        let app_name = get_string_from_user("The app's name:", false)?;

        // get the package name
        let package_name = get_string_from_user("Package name (i.e. com.company.app):", false)?;

        // get whether it works
        let works = get_bool_from_user("Does the app work? (y/n)")?;

        // if works is false, then just return a faster "doesn't work" thing
        if !works {
            return Ok(Self {
                app_name,
                package_name,
                works,
                works_without_compat_mode: false,
                works_without_gms: BoolOrNone(None),
                works_installed_by_any_source: BoolOrNone(None),
                comment: None,
            });
        }

        // get whether it requires exploit protection compatibility mode
        let works_without_compat_mode = get_bool_from_user(
            "Does the app work without Exploit Protection Compatibility Mode? (y/n)",
        )?;

        // get whether the app requires Google Play etc to be installed in the same profile
        let works_without_gms = get_option_bool_from_user("Does the app work without Google Play being installed in the same profile? (y/n or just leave this empty)")?;

        // get whether the app requires that it's installed by Google Play
        let works_installed_by_any_source =
            get_option_bool_from_user("Does the app if installed by an app other than Google Play? (y/n or just leave this empty)")?;

        let comment =
            get_option_string_from_user("Any other comments about the app's compatibility?")?;

        Ok(Self {
            app_name,
            package_name,
            works,
            works_without_compat_mode,
            works_without_gms,
            works_installed_by_any_source,
            comment,
        })
    }

    // convenience function to get the first character of the app name
    pub fn get_name_first_char(&self) -> char {
        self.app_name
            .chars()
            .nth(0)
            .expect("Unable to get the first letter of an app name. Is it empty?")
            .to_ascii_uppercase()
    }

    pub fn print_card_line(&self) -> String {
        let general_status_icon = match (self.works, self.works_without_compat_mode) {
            (true, true) => "✅",
            (true, false) => "⚠️",
            _ => "❌",
        };

        format!("{{{{ app_compat_card( app_name = \"{}\", package_name = \"{}\", works = \"{}\", works_without_gms = \"{}\", works_installed_by_any_source = \"{}\", comments = \"{}\", works_bool = {} ) }}}}",
            self.app_name,
            self.package_name,
            general_status_icon,
            self.works_without_gms,
            self.works_installed_by_any_source,
            if let Some(c) = &self.comment {
                c.to_string()
            } else {
                "".to_string()
            },
            self.works,
        )
    }

    pub fn save_to_file(&self, path: &mut PathBuf, filename: String) -> Result<(), String> {
        path.push(filename);

        // don't save over an old file
        if path.is_file() {
            return Err("This file already exists.\nPlease update the file manually.".to_string());
        }

        let new_config_file = File::create(path).map_err(|e| e.to_string())?;
        serde_yaml::to_writer(&new_config_file, self).map_err(|e| e.to_string())?;

        Ok(())
    }

    // remove all double quotes here so that they don't mess up the
    // shortcode in the `.md` file
    // This shouldn't be necessary, but I'll use as a failsafe.
    pub fn remove_double_quotes_from_all_string_fields(&mut self) {
        self.app_name = self.app_name.replace("\"", "'");
        self.package_name = self.package_name.replace("\"", "'");

        if let Some(comment) = &self.comment {
            self.comment = Some(comment.replace("\"", "'"));
        }
    }

    // replace all \n in comments with <br>
    // because a new line would break the shortcode
    pub fn fix_new_line_in_comments(&mut self) {
        if let Some(comment) = &self.comment {
            self.comment = Some(comment.replace("\n", "<br>"));
        }
    }
}
