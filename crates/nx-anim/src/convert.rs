use std::{ffi::OsString, path::PathBuf};

use nx_common::Game;

use crate::{Error, boned, mapping};

/// `name` is the animation's name. When `settle` holds an exit for that
/// name, the converted animation's tail is eased onto the pose it gets cut into.
pub fn convert(
    anim: &boned::Animation,
    in_game: Game,
    out_game: Game,
    name: &str,
    settle: Option<(&mapping::ExitSpec, &mapping::SettleContext)>,
) -> Result<boned::Animation, Error> {
    let converted = match (in_game, out_game) {
        (Game::THUG, Game::THPS4) => mapping::thug_to_thps4(anim)?,
        _ => return Err(Error::MappingNotImplemented(in_game, out_game)),
    };

    match settle {
        Some((spec, ctx)) => Ok(mapping::settle(converted, name, spec, ctx)?),
        None => Ok(converted),
    }
}

/// An animation's bare name, with every extension stripped: `Wallpush_Standing.ska.xbx` ->
/// `Wallpush_Standing`.
pub fn anim_name(path: &std::path::Path) -> Option<String> {
    path.with_extension("")
        .with_extension("")
        .file_name()
        .map(|name| name.to_string_lossy().into_owned())
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
