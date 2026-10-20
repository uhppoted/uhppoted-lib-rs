// use std::env;
use std::fs;
use std::path::Path;

fn main() {
    println!("cargo:warning=*** CODE GENERATION ***");
    // let out_dir = env::var("OUT_DIR").unwrap();
    // let dest_path = Path::new(&out_dir).join("generated_tests.rs");

    let dest_path = Path::new("src/generated/generated_tests.rs");

    let generated_code = r#"
        #[test]
        fn test_generated_case() {
            assert_eq!(2 * 2, 4);
        }
    "#;

    // println!("cargo:warning=*** CODE GENERATION *** {}", out_dir);
    println!(
        "cargo:warning=*** CODE GENERATION *** {}",
        dest_path.display()
    );

    fs::write(dest_path, generated_code).unwrap();

    println!("cargo:rerun-if-changed=build.rs");
}
