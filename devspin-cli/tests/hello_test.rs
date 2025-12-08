//! IF THIS TEST IS NOT MODIFIED AND IT FAILS: IT MEANS THAT THE CLI LOGIC HAS BEEN CHANGED TO A RLLY BAD LOGIC
//! PLS ME, DO NOT CHANGE THIS LOGIC

use assert_cmd::cargo::cargo_bin_cmd;
use predicates::prelude::*; // For .not()

#[test]
fn test_test_cmd_without_w_msg_flag() {
    let mut cmd = cargo_bin_cmd!("devspin-cli");
    cmd.arg("test-cmd");
    cmd.assert()
        .success()
        .stdout(predicates::str::contains("DEVSPIN").not());
}

#[test]
fn test_test_cmd_with_w_msg_flag() {
    let mut cmd = cargo_bin_cmd!("devspin-cli");
    cmd.arg("test-cmd").arg("--w-msg");
    cmd.assert().success().stdout(predicates::str::contains(
        "
    ██████╗ ███████╗██╗   ██╗███████╗██████╗ ██╗███╗   ██╗
    ██╔══██╗██╔════╝██║   ██║██╔════╝██╔══██╗██║████╗  ██║
    ██║  ██║█████╗  ██║   ██║███████╗██████╔╝██║██╔██╗ ██║
    ██║  ██║██╔══╝  ╚██╗ ██╔╝╚════██║██╔═══╝ ██║██║╚██╗██║
    ██████╔╝███████╗ ╚████╔╝ ███████║██║     ██║██║ ╚████║
    ╚═════╝ ╚══════╝  ╚═══╝  ╚══════╝╚═╝     ╚═╝╚═╝  ╚═══╝",
    ));
}
