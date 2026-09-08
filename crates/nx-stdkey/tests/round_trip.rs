#[tokio::test]
async fn round_trip() {
    nx_test::test_round_trip::<nx_stdkey::StdKey>(nx_test::find_assets("stdkey"), &mut (), &mut ())
        .await
}
