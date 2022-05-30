use assert_cmd::Command;

#[test]
fn runs() {
    let mut cmd = Command::cargo_bin("image_combiner").unwrap();
    cmd.arg("images/1.jpeg")
        .arg("images/2.jpeg")
        .arg("images/output.jpeg");
    cmd.assert().success();
}