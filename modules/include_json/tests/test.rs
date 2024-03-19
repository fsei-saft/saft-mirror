use include_json::{parse_json, IncludeJson};

#[test]
fn test() {
    #[derive(Debug, PartialEq, Eq, IncludeJson)]
    struct Test {
        num: i64,
        string: String
    }

    let instance = Test {
        num: 0,
        string: "test".to_string()
    };

    assert_eq!(parse_json!(Test, "{\"num\": 0, \"string\": \"test\"}"), instance);
}
