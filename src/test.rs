use crate::types::*;
use crate::*;

#[tokio::test(flavor = "multi_thread")]
async fn test_sanity() {
    #[derive(Debug)]
    struct TestSpaceHandler;

    impl space::SpaceHandler for TestSpaceHandler {
        fn incoming_request(
            &self,
            _peer: DynHash,
            _req_id: Bytes,
            _data: Bytes,
        ) -> Result<()> {
            Ok(())
        }
    }

    #[derive(Debug)]
    struct TestKitsuneHandler;

    impl kitsune::Kitsune2Handler for TestKitsuneHandler {
        fn preflight_gather(&self, _peer_url: PeerUrl) -> Result<Bytes> {
            Ok(Bytes::from_static(b""))
        }

        fn preflight_validate(
            &self,
            _peer_url: PeerUrl,
            _data: Bytes,
        ) -> Result<()> {
            Ok(())
        }

        fn create_space(
            &self,
            _space: SpaceHash,
        ) -> BoxFuture<'_, Result<space::DynSpaceHandler>> {
            Box::pin(async move {
                let handler: space::DynSpaceHandler =
                    Arc::new(TestSpaceHandler);
                Ok(handler)
            })
        }
    }

    let handler: kitsune::DynKitsune2Handler = Arc::new(TestKitsuneHandler);

    let builder = builder::Builder::new_testing();
    let kitsune = builder.build(handler).await.unwrap();
    let space = kitsune.space(Bytes::from_static(b"test-space")).await;

    // now we have the space, we can do things like access
    // the peer_store inside the space.
    space
        .peer_store()
        .ingest_agent_info_list(vec![])
        .await
        .unwrap();
}
