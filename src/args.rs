use std::{ffi::OsStr, fs, path::PathBuf};

use clap::{
    arg,
    builder::{OsStringValueParser, TypedValueParser},
    command, ArgMatches,
};

#[derive(Debug)]
pub struct Config {
    pub debug: bool,
    pub input: String,
}

impl Config {
    /// Convert clap matches to Config
    pub fn from_args(matches: &ArgMatches) -> Self {
        Self {
            debug: matches.contains_id("debug"),
            input: matches
                .get_one::<PathBuf>("input")
                .expect("<Input> is required")
                .as_path()
                .to_string_lossy()
                .to_string(),
        }
    }
}

/// Create clap args parser
pub fn cmd() -> clap::Command {
    command!()
        // input file
        .arg(
            arg!(<input>)
                .help("Input file")
                .value_parser(OsStringValueParser::new().try_map(|os| {
                    let path = PathBuf::from(os);
                    if path.extension() != Some(OsStr::new("alt")) {
                        return Err(String::from("Input file must have .alt extension"));
                    }

                    if fs::metadata(&path).is_err() {
                        return Err(String::from("Input file does not exist"));
                    }

                    Ok(path)
                })),
        )
        // debug flag
        .arg(arg!(
            -d --debug "Open debug mode"
        ))
}
