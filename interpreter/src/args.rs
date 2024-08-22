use std::{ffi::OsStr, fs, path::PathBuf};

use clap::{
    arg,
    builder::{OsStringValueParser, TypedValueParser},
    command, value_parser, ArgMatches,
};
use rand::Rng;

#[derive(Debug)]
pub struct Config {
    pub debug: bool,
    pub input: String,
    pub seed: u64,
}

impl Config {
    /// Convert clap matches to Config
    pub fn from_args(matches: &ArgMatches) -> Self {
        Self {
            debug: matches.get_flag("debug"),
            input: matches
                .get_one::<PathBuf>("input")
                .expect("<Input> is required")
                .as_path()
                .to_string_lossy()
                .to_string(),
            seed: matches
                .get_one::<u64>("seed")
                .copied()
                .unwrap_or(rand::thread_rng().gen()),
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
        .arg(arg!(
            -c --check "Check syntax only"
        ))
        .arg(arg!(
            -d --debug "Open debug mode"
        ))
        .arg(
            arg!(-s --seed <seed> "Run program with specific seed")
                .value_parser(value_parser!(u64)),
        )
}
