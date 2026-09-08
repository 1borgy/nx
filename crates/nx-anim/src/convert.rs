use nx_common::Game;

use crate::{Error, boned, mapping};

pub fn convert(
    anim: &boned::Animation,
    in_game: Game,
    out_game: Game,
) -> Result<boned::Animation, Error> {
    match (in_game, out_game) {
        (Game::THUG, Game::THPS4) => Ok(mapping::thug_to_thps4(anim)),
        _ => Err(Error::MappingNotImplemented(in_game, out_game)),
    }
}
