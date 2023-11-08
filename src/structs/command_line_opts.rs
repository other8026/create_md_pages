use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt, Debug, Clone)]
#[structopt(name = "create_md_pages")]
pub struct CommandLineOpts {
    /// Use --add to create a new app config file.
    #[structopt(long)]
    pub add: bool,
    /// Use this to run the thing and create the file.
    #[structopt(long)]
    pub run: bool,

    /// An input yaml for creating the .md files.
    /// Required for run and check.
    #[structopt(long, parse(from_os_str))]
    pub yaml_file: Option<PathBuf>,
    /// The folder where all app files are stored.
    /// It's always required.
    #[structopt(long, parse(from_os_str))]
    pub app_files_folder: PathBuf,
    /// The file where the resulting .md file will be saved.
    /// It's only required when using --run.
    #[structopt(long, parse(from_os_str))]
    pub output_md_file: Option<PathBuf>,
}
