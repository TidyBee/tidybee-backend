use log::error;
use std::path;

pub struct Options {
    pub file_extensions_args: Option<Vec<String>>,
    pub file_types_args: Option<String>,
    pub directories_list_args: Option<Vec<path::PathBuf>>,
    pub directories_watch_args: Option<Vec<path::PathBuf>>,
    pub receive_address_arg: Option<String>,
    pub send_address_arg: Option<String>,
}

pub enum OptionsError {
    MissingCommand(String),
    InvalidDirectory(String),
    InvalidFileExtension(String),
    InvalidFileType(String),
}

pub fn get_options() -> Result<Options, OptionsError> {
    check_options(clap_options().get_matches())
}

fn clap_options() -> clap::App<'static, 'static> {
    let options: clap::App<'_, '_> = clap::App::new("TidyBee")
        .version("0.0.1")
        .author("majent4")
        .about("Watch for changes in directories and recursively list directories")
        .arg(
            clap::Arg::with_name("extension")
                .short("e")
                .long("extension")
                .value_name("EXTENSIONS")
                .multiple(true)
                .use_delimiter(true)
                .takes_value(true)
                .help("Specify file extensions to watch/list:\ndocx, jpeg, jpg, mp3, mp4, pdf, png and xlsx (default is all)"),
        )
        .arg(
            clap::Arg::with_name("type")
                .short("t")
                .long("type")
                .value_name("TYPES")
                .multiple(true)
                .use_delimiter(true)
                .takes_value(true)
                .help("Specify file types to watch/list:\nall, directory and regular (default is all)"),
        )
        .arg(
            clap::Arg::with_name("list")
                .short("l")
                .long("list")
                .value_name("DIRECTORIES")
                .multiple(true)
                .use_delimiter(true)
                .takes_value(true)
                .required(false)
                .help("Specify directories for listing"),
        )
        .arg(
            clap::Arg::with_name("watch")
                .short("w")
                .long("watch")
                .value_name("DIRECTORIES")
                .multiple(true)
                .use_delimiter(true)
                .takes_value(true)
                .required(false)
                .help("Specify directories for watching"),
        )
        .arg(
            clap::Arg::with_name("receive")
                .short("r")
                .long("receive")
                .value_name("ADDRESS")
                .takes_value(true)
                .help("Specify address for receiving data from the UI"),
        )
        .arg(
            clap::Arg::with_name("send")
                .short("s")
                .long("send")
                .value_name("ADDRESS")
                .takes_value(true)
                .help("Specify address for sending data to the UI"),
        );
    options
}

fn check_options(matches: clap::ArgMatches<'_>) -> Result<Options, OptionsError> {
    let directories_list_args: Option<Vec<path::PathBuf>> = matches
        .values_of("list")
        .map(|dirs: clap::Values<'_>| dirs.map(path::PathBuf::from).collect())
        .or(Some(vec![path::PathBuf::from(".")]));

    let directories_watch_args: Option<Vec<path::PathBuf>> = matches
        .values_of("watch")
        .map(|dirs: clap::Values<'_>| dirs.map(path::PathBuf::from).collect())
        .or(Some(vec![path::PathBuf::from(".")]));

    if !directories_list_args.is_some() || !directories_watch_args.is_some() {
        return Err(OptionsError::MissingCommand("".to_string()));
    }

    if let Some(directories) = &directories_list_args {
        for directory in directories {
            if !directory.is_dir() {
                return Err(OptionsError::InvalidDirectory(format!(
                    "Specified directory does not exists: {:?}",
                    directory
                )));
            }
        }
    }

    if let Some(directories) = &directories_watch_args {
        for directory in directories {
            if !directory.is_dir() {
                return Err(OptionsError::InvalidDirectory(format!(
                    "Specified directory does not exists: {:?}",
                    directory
                )));
            }
        }
    }

    let file_extensions_args: Option<Vec<String>> = matches
        .values_of("extension")
        .map(|exts: clap::Values<'_>| exts.map(String::from).collect());

    let valid_extensions: Vec<&str> =
        vec!["docx", "jpeg", "jpg", "mp3", "mp4", "pdf", "png", "xlsx"];

    if let Some(file_extensions_args) = &file_extensions_args {
        for e in file_extensions_args {
            if !valid_extensions.contains(&e.as_str()) {
                return Err(OptionsError::InvalidFileExtension(format!(
                    "Invalid file extension: {}",
                    e
                )));
            }
        }
    }

    let file_types_args_vec: Option<Vec<String>> = matches
        .values_of("type")
        .map(|file_types_args: clap::Values<'_>| file_types_args.map(String::from).collect());

    let valid_file_types_args: Vec<&str> = vec!["*", "all", "directory", "regular"];

    if let Some(file_types_args) = &file_types_args_vec {
        for t in file_types_args {
            if !valid_file_types_args.contains(&t.as_str()) {
                return Err(OptionsError::InvalidFileType(format!(
                    "Invalid file type: {}",
                    t
                )));
            }
        }
    }

    let mut file_types_args: Option<String> = None;

    if let Some(t) = &file_types_args_vec {
        if t.iter().any(|t: &String| t.contains("directory")) {
            file_types_args = Some("directory".to_string());
        }
        if t.iter()
            .any(|t: &String| t.contains("*") || t.contains("all"))
        {
            file_types_args = Some("all".to_string());
        }
        if t.iter().any(|t: &String| t.contains("regular")) {
            file_types_args = Some("regular".to_string());
        }
    }

    if let Some(file_extensions_args) = &file_extensions_args {
        if let Some(file_types_args) = &file_types_args {
            if file_types_args == "directory" && !file_extensions_args.is_empty() {
                return Err(OptionsError::InvalidFileType(
                    "Can't specify both file extensions and file type directory simultaneously"
                        .to_string(),
                ));
            }
        }
    }

    let receive_address_arg: Option<String> = matches.value_of("receive").map(String::from);

    let send_address_arg: Option<String> = matches.value_of("send").map(String::from);

    Ok(Options {
        directories_list_args,
        directories_watch_args,
        file_extensions_args,
        file_types_args,
        receive_address_arg,
        send_address_arg,
    })
}

pub fn print_option_error(error: OptionsError) {
    match error {
        OptionsError::MissingCommand(e) => {
            error!("{}", e);
        }
        OptionsError::InvalidDirectory(e) => {
            error!("{}", e);
        }
        OptionsError::InvalidFileExtension(e) => {
            error!("{}", e);
        }
        OptionsError::InvalidFileType(e) => {
            error!("{}", e);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_easy_missing_watch() {
        let arguments: Vec<&str> = vec!["tidybee-agent", "--list", "/usr"];
        let options: clap::ArgMatches<'_> = clap_options().get_matches_from(arguments);
        assert!(check_options(options).is_err());
    }

    #[test]
    fn test_easy_missing_list() {
        let arguments: Vec<&str> = vec!["tidybee-agent", "--watch", "/usr"];
        let options: clap::ArgMatches<'_> = clap_options().get_matches_from(arguments);
        assert!(check_options(options).is_err());
    }

    #[test]
    fn test_easy_valid() {
        let arguments: Vec<&str> = vec!["tidybee-agent", "--watch", "/tmp", "--list", "/tmp"];
        let options: clap::ArgMatches<'_> = clap_options().get_matches_from(arguments);
        assert!(check_options(options).is_ok());
    }

    #[test]
    fn test_easy_help() {
        let arguments: Vec<&str> = vec!["tidybee-agent", "--help"];
        let options: clap::ArgMatches<'_> = clap_options().get_matches_from(arguments);
        assert!(check_options(options).is_ok());
    }

    #[test]
    fn test_easy_version() {
        let arguments: Vec<&str> = vec!["tidybee-agent", "--version"];
        let options: clap::ArgMatches<'_> = clap_options().get_matches_from(arguments);
        assert!(check_options(options).is_ok());
    }

    //#[test]
    //fn test_easy_empty() {
    //    let arguments: Vec<&str> = vec!["tidybee-agent"];
    //    let options: clap::ArgMatches<'_> = clap_options().get_matches_from(arguments);
    //    assert!(check_options(options).is_err());
    //}

    #[test]
    fn test_directory_extension() {
        let arguments: Vec<&str> = vec![
            "tidybee-agent",
            "--watch",
            "/usr",
            "--list",
            "/usr",
            "-e",
            "pdf",
            "-t",
            "directory",
        ];
        let options: clap::ArgMatches<'_> = clap_options().get_matches_from(arguments);
        assert!(check_options(options).is_err());
    }

    #[test]
    fn test_file_extension() {
        let arguments: Vec<&str> = vec![
            "tidybee-agent",
            "--watch",
            "/usr",
            "--list",
            "/usr",
            "-e",
            "pdf",
            "-t",
            "file",
        ];
        let options: clap::ArgMatches<'_> = clap_options().get_matches_from(arguments);
        assert!(check_options(options).is_ok());
    }
}