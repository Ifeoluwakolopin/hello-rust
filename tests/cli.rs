use assert_cmd::Command;

#[test]
fn runs() {
    let mut cmd = Command::cargo_bin("image_combiner").unwrap();
    cmd.arg("tests/images/1.jpeg")
        .arg("tests/images/2.jpeg")
        .arg("tests/images/output.jpeg");
    cmd.assert().success();
}