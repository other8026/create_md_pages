use crate::stdin_functions::get_option_string_from_user::get_option_string_from_user;
use crate::stdin_functions::{
    get_bool_from_user::get_bool_from_user, get_option_bool_from_user::get_option_bool_from_user,
    get_string_from_user::get_string_from_user,
};
use crate::structs::bool_or_none::BoolOrNone;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::path::PathBuf;
use crate::structs::string_or_none::StringOrNone;

#[derive(Debug, Deserialize, Serialize)]
pub struct AppCompatApp {
    pub app_name: String,
    pub package_name: String,
    pub version: String,
    pub repo_or_download_link: StringOrNone,
    pub description: StringOrNone,
    pub works: bool,
    pub works_without_compat_mode: bool,
    pub works_without_gms: BoolOrNone,
    pub works_installed_by_any_source: BoolOrNone,
    pub other_compatibility_comment: StringOrNone,
}

impl AppCompatApp {
    // call this to create a new struct from user input from stdin
    // (this will be used to create the yaml files)
    pub fn new_from_command_line() -> Result<Self, String> {
        let app_name = get_string_from_user("[required] The app's name:", false)?;

        let package_name =
            get_string_from_user("[required] Package name (i.e. com.company.app):", false)?;

        let version = get_string_from_user("[required] Version number (i.e. 1.23.4):", false)?;

        let repo_or_download_link =
            get_option_string_from_user("[optional] Download or repo link:")?;

        let description = get_option_string_from_user("[optional] App description:")?;

        let works = get_bool_from_user("[required] Does the app work? (y/n)")?;

        // if works is false, then just return a faster "doesn't work" thing
        if !works {
            return Ok(Self {
                app_name,
                package_name,
                version,
                repo_or_download_link,
                description,
                works,
                works_without_compat_mode: false,
                works_without_gms: BoolOrNone(None),
                works_installed_by_any_source: BoolOrNone(None),
                other_compatibility_comment: StringOrNone(None),
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

        let other_compatibility_comment =
            get_option_string_from_user("Any other comments about the app's compatibility?")?;

        Ok(Self {
            app_name,
            package_name,
            version,
            repo_or_download_link,
            description,
            works,
            works_without_compat_mode,
            works_without_gms,
            works_installed_by_any_source,
            other_compatibility_comment,
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
            self.other_compatibility_comment,
            self.works,
        )
    }

    pub fn save_to_file(&self, path: &mut PathBuf) -> Result<(), String> {
        path.push(format!("{}.yaml", &self.package_name));

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
    pub fn replace_double_quotes_from_all_string_fields(&mut self) {
        self.app_name = self.app_name.replace("\"", "'");
        self.package_name = self.package_name.replace("\"", "'");

        if let StringOrNone(Some(comment)) = &self.other_compatibility_comment {
            self.other_compatibility_comment = StringOrNone(Some(comment.replace("\"", "'")));
        }
    }

    // replace all \n in comments with <br>
    // because a new line would break the shortcode
    pub fn fix_new_line_in_comments(&mut self) {
        if let StringOrNone(Some(comment)) = &self.other_compatibility_comment {
            // this is to remove any trailing new lines
            // which seems to always happen when typing like this:
            //
            // comment: |
            //   stuff stuff stuff
            //   more stuff
            let comment = comment.trim();

            // wrap each line with <p> tags
            self.other_compatibility_comment = StringOrNone(Some(
                comment
                    .split("\n")
                    .map(|s| format!("<p>{}</p>", s))
                    .collect::<Vec<String>>()
                    .join(""),
            ));
        }
    }
}
