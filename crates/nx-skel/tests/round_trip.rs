use nx_skel::Skeleton;

#[tokio::test]
async fn round_trip() {
    nx_test::test_round_trip::<Skeleton>(nx_test::find_assets("skel"), &mut (), &mut ()).await
}
