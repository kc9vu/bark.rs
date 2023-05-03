use base64::prelude::{Engine as _, BASE64_STANDARD};

fn main() {
    let key = b"d5a60a92ee2cf148d86cdb0beaf5f74e";
    let iv = b"ec8ae9f1e9392757";

    println!(
        "{}",
        BASE64_STANDARD.encode(key) == "ZDVhNjBhOTJlZTJjZjE0OGQ4NmNkYjBiZWFmNWY3NGU="
    );
    println!(
        "{}",
        BASE64_STANDARD.encode(iv) == "ZWM4YWU5ZjFlOTM5Mjc1Nw=="
    );
}
