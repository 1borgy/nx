use std::{ffi::OsString, path::PathBuf};

use nx_common::Game;

use crate::{Error, boned, mapping};

pub fn convert(
    anim: &boned::Animation,
    in_game: Game,
    out_game: Game,
) -> Result<boned::Animation, Error> {
    match (in_game, out_game) {
        (Game::THUG, Game::THPS4) => Ok(mapping::thug_to_thps4(anim)?),
        _ => Err(Error::MappingNotImplemented(in_game, out_game)),
    }
}

pub fn convert_path(path: &PathBuf, in_game: Game, out_game: Game) -> Result<PathBuf, Error> {
    match (in_game, out_game) {
        (Game::THUG, Game::THPS4) => {
            let mut new_filename = OsString::new();
            new_filename.push(
                path.with_extension("")
                    .with_extension("")
                    .file_name()
                    .ok_or_else(|| Error::FilenameError)?,
            );
            new_filename.push("ska.dat");
            Ok(path.with_file_name(new_filename))
        }
        _ => Err(Error::MappingNotImplemented(in_game, out_game)),
    }
}
