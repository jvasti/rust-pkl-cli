#[test]
fn cli_tests() {
    trycmd::TestCases::new()
        .case("tests/cases/*.md")
        .case("README.md");
}
