use std::fs;
use std::path::Path;

use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct Tests {
    #[serde(rename(deserialize = "tests"))]
    _tests: Vec<Test>,
}

#[derive(Deserialize, Debug)]
struct Test {
    #[serde(rename(deserialize = "test"))]
    _test: String,
}

fn main() {
    // ... parse model files
    let json = fs::read_to_string("../.codegen/generated/encode.json").unwrap();
    let deserialized: Tests = serde_json::from_str(&json).unwrap();

    println!("cargo:warning=... tests {:?}", deserialized);

    // ... generate encode tests
    let file = Path::new("src/generated/encode_tests.rs");

    let generated = r#"
    #[test]
    fn find_controllers() {
        let expected: Msg = [
            0x17, 0x94, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        ];

        let result = get_controller_request(0).unwrap();

        assert_eq!(result, expected);
    }
    "#;

    println!(
        "cargo:warning=... generating encode unit tests {}",
        file.display()
    );

    fs::write(file, generated).unwrap();

    println!("cargo:rerun-if-changed=build.rs");
}
