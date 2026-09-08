use nx_anim::{Animation, WriteContext};
use nx_common::Game;

#[tokio::test]
async fn round_trip_thps4() {
    nx_test::test_round_trip::<Animation>(
        nx_test::find_assets("anim/thps4"),
        &mut (),
        &mut WriteContext { game: Game::THPS4 },
    )
    .await
}

#[tokio::test]
async fn round_trip_thug() {
    nx_test::test_round_trip::<Animation>(
        nx_test::find_assets("anim/thug"),
        &mut (),
        &mut WriteContext { game: Game::THUG },
    )
    .await
}

#[tokio::test]
async fn round_trip_thug2() {
    nx_test::test_round_trip::<Animation>(
        nx_test::find_assets("anim/thug2"),
        &mut (),
        &mut WriteContext { game: Game::THUG2 },
    )
    .await
}
