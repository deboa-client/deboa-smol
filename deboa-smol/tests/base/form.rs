use crate::common::helpers::{create_client, create_server, protocol_version};
use deboa::TestResult;
use deboa_smol::Client;
use easyhttpmock_vetis_smol::{vetis_adapter::VetisAdapter, EasyHttpMock};
use macro_rules_attribute::apply;
use rstest::rstest;
use smol_macros::test;

#[test]
fn test_encoded_form() -> TestResult<()> {
    deboa_test_utils::base::form::test_encoded_form()
}

#[rstest]
#[test_attr(apply(test))]
async fn test_submit_encoded_form(
    create_client: Client,
    #[future] create_server: EasyHttpMock<VetisAdapter>,
    protocol_version: http::Version,
) -> TestResult<()> {
    let mut server = create_server.await;
    deboa_test_utils::base::form::test_submit_encoded_form(
        &create_client,
        &mut server,
        protocol_version,
    )
    .await
}
