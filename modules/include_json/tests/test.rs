use include_json::{parse_json, IncludeJson};

#[test]
fn simple_parse_json() {
    #[derive(Debug, PartialEq, Eq, IncludeJson)]
    struct Test {
        num: i32,
        string: String
    }

    let instance = Test {
        num: 0,
        string: "test".to_string()
    };

    assert_eq!(parse_json!(Test, "{\"num\": 0, \"string\": \"test\"}"), instance);
}

#[test]
fn nested_struct_parse_json() {
    #[derive(Debug, PartialEq, Eq, IncludeJson)]
    struct Outer {
        i: Inner
    }

    #[derive(Debug, PartialEq, Eq, IncludeJson)]
    struct Inner {
        val: i64
    }

    let instance = Outer {
        i: Inner {
            val: 0
        }
    };

    assert_eq!(parse_json!(Outer, "{\"i\": {\"val\": 0}}"), instance);
}

#[test]
fn vec_struct_parse_json() {
    #[derive(Debug, PartialEq, Eq, IncludeJson)]
    struct Outer {
        i: Vec<Inner>
    }

    #[derive(Debug, PartialEq, Eq, IncludeJson)]
    struct Inner {
        val: i64
    }

    let instance = Outer {
        i: vec![
            Inner {
                val: 0
            },
            Inner {
                val: 1
            }
        ]
    };

    assert_eq!(parse_json!(Outer, "{\"i\": [{\"val\": 0}, {\"val\": 1}]}"), instance);
}
