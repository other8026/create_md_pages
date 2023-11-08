use crate::stdin_functions::{
    get_bool_from_user::get_bool_from_user, get_option_bool_from_user::get_option_bool_from_user,
    get_string_from_user::get_string_from_user,
};
use crate::structs::bool_or_none::BoolOrNone;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::fmt::Display;

#[derive(Debug, Deserialize, Serialize)]
pub struct AppCompatApp {
    pub app_name: String,
    pub package_name: String,
    pub works: bool,
    pub requires_compat_mode: bool,
    pub requires_gms: BoolOrNone,
    pub requires_installed_by_play: BoolOrNone,
    pub comment: String,
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
                requires_compat_mode: false,
                requires_gms: BoolOrNone(None),
                requires_installed_by_play: BoolOrNone(None),
                comment: "".to_string(),
            });
        }

        // get whether it requires exploit protection compatibility mode
        let requires_compat_mode = get_bool_from_user(
            "Does the app require Exploit Protection Compatibility Mode? (y/n)",
        )?;

        // get whether the app requires Google Play etc to be installed in the same profile
        let requires_gms = get_option_bool_from_user("Does the app require Google Play be installed in the same profile? (y/n/idk or just leave this empty)")?;

        // get whether the app requires that it's installed by Google Play
        let requires_installed_by_play =
            get_option_bool_from_user("Does the app require it was installed by Google Play? (y/n/idk or just leave this empty)")?;

        let comment =
            get_string_from_user("Any other comments about the app's compatibility?", true)?;

        Ok(Self {
            app_name,
            package_name,
            works,
            requires_compat_mode,
            requires_gms,
            requires_installed_by_play,
            comment,
        })
    }
}

impl Display for AppCompatApp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let status_icon = match (self.works, self.requires_compat_mode) {
            (true, true) => "⚠️",
            (true, false) => "✅",
            (false, true) => "❌",
            (false, false) => "❌",
        };

        write!(
            f,
            "|{}|`{}`|{}|{}|{}|",
            self.app_name,
            self.package_name,
            status_icon,
            self.requires_gms,
            self.requires_installed_by_play
        )
    }
}
