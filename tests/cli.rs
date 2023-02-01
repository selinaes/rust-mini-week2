
use assert_cmd::Command;

#[test]
//a test to invoke the cli with an subcommand 'convert' and option --num 34
fn thirtyfour() {
    let mut cmd = Command::cargo_bin("mini1").unwrap();
    cmd.arg("convert").arg("--num").arg("34");
    cmd.assert().success().stdout("XXXIV\n");

}

#[test]
//a test to invoke the cli with an subcommand 'convert' and option --num 58
fn fiftyeight() {
    let mut cmd = Command::cargo_bin("mini1").unwrap();
    cmd.arg("convert").arg("--num").arg("58");
    cmd.assert().success().stdout("LVIII\n");

}

#[test]
//a test to invoke the cli with an subcommand 'convert' and option --num 1994
fn test1994() {
    let mut cmd = Command::cargo_bin("mini1").unwrap();
    cmd.arg("convert").arg("--num").arg("1994");
    cmd.assert().success().stdout("MCMXCIV\n");

}

#[test]
//a test to invoke the cli with an subcommand 'convert' and option --num 2004
fn test2004() {
    let mut cmd = Command::cargo_bin("mini1").unwrap();
    cmd.arg("convert").arg("--num").arg("2004");
    cmd.assert().success().stdout("MMIV\n");
}