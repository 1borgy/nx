#[tokio::test]
async fn round_trip() {
    nx_test::test_round_trip::<nx_qb::Qb>(nx_test::find_assets("qb"), &mut (), &mut ()).await
}
