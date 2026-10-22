use std::fs;
use std::path::Path;

use base64::{Engine as _, engine::general_purpose};
use serde::{Deserialize, Deserializer};

#[derive(Deserialize, Debug)]
struct Tests {
    #[serde(rename(deserialize = "requests"))]
    requests: Vec<Request>,
}

#[derive(Deserialize, Debug)]
struct Request {
    #[serde(rename(deserialize = "request"))]
    _request: String,
    tests: Vec<Test>,
}

#[derive(Deserialize, Debug)]
struct Test {
    #[serde(rename(deserialize = "test"))]
    name: String,

    #[serde(rename(deserialize = "expected"), deserialize_with = "from_base64")]
    expected: Packet,
    // expected: Vec<u8>,
}

pub type Packet = [u8; 64];

fn main() {
    // ... parse model files
    let json = fs::read_to_string("../.codegen/generated/encode.json").unwrap();
    let deserialized: Tests = serde_json::from_str(&json).unwrap();

    for request in deserialized.requests.iter() {
        for test in request.tests.iter() {
            println!("cargo:warning=... TEST {:?}", test.name);
            println!("cargo:warning=... TEST {:?}", test.expected);
        }
    }

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

fn from_base64<'de, D: Deserializer<'de>>(deserializer: D) -> Result<Packet, D::Error> {
    <&str>::deserialize(deserializer).and_then(|s| {
        let bytes = general_purpose::STANDARD
            .decode(s)
            .map_err(serde::de::Error::custom)?;

        if bytes.len() != 64 {
            return Err(serde::de::Error::custom(format!(
                "Invalid packet length: expected 64 bytes, got {}",
                bytes.len()
            )));
        }

        let mut packet = [0u8; 64];

        packet.copy_from_slice(&bytes);

        Ok(packet)
    })
}
