use std::process::Command;
fn main() {
    let output = Command::new("cargo")
        .args([
            "contract",
            "call",
            "--contract",
            "5E3rCrW49guH5V9HMrBqys76nSgkzneaLvRvupZzT7DEoqj9",
            "--message",
            "set_assistant",
            "--args",
            "0x416C66726564",
            "--suri",
            "//Alice",
            // "--dry-run"
        ])
        .current_dir("../batman")
        .output()
        .expect("failed to execute process");

    println!("{:#?}", output);
}
