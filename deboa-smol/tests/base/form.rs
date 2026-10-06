use deboa::TestResult;
use macro_rules_attribute::apply;
use smol_macros::test;

#[test]
fn test_encoded_form() -> TestResult<()> {
    deboa_test_utils::base::form::test_encoded_form()
}
