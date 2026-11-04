//! End-to-end tests of the built binary: the flags, the output shapes and the
//! exit codes a user sees. Unit tests of the pieces live in src/main.rs.

use std::fs;
use std::path::PathBuf;
use std::process::{Command, Output};

fn pgen(args: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_pgen"))
        .args(args)
        .output()
        .expect("pgen runs")
}

fn stdout(output: &Output) -> String {
    String::from_utf8(output.stdout.clone()).expect("utf-8 stdout")
}

fn stderr(output: &Output) -> String {
    String::from_utf8(output.stderr.clone()).expect("utf-8 stderr")
}

/// A fresh, empty directory under the target dir for tests that write files.
fn scratch_dir(name: &str) -> PathBuf {
    let dir = PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join(name);
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).expect("scratch dir");
    dir
}

#[test]
fn version_matches_the_manifest() {
    let output = pgen(&["--version"]);
    assert!(output.status.success());
    assert_eq!(
        stdout(&output).trim(),
        format!("pgen {}", env!("CARGO_PKG_VERSION"))
    );
}

#[test]
fn length_is_required() {
    let output = pgen(&[]);
    assert_eq!(output.status.code(), Some(2), "clap usage error");
    assert!(stderr(&output).contains("--length"));
}

#[test]
fn single_password_in_text_mode() {
    let output = pgen(&["-l", "12", "-c", "1"]);
    assert!(output.status.success());
    let text = stdout(&output);
    let lines: Vec<&str> = text.split('\n').collect();
    assert_eq!(lines[0], "### PASSWORD ###");
    assert_eq!(lines[1].len(), 12);
    assert!(lines[1].chars().all(|c| c.is_ascii_lowercase()));
    assert_eq!(&lines[2..], &["", ""], "block ends with a blank line");
}

#[test]
fn several_passwords_are_numbered() {
    let output = pgen(&["-l", "8", "-n", "3"]);
    assert!(output.status.success());
    let text = stdout(&output);
    for i in 1..=3 {
        assert!(text.contains(&format!("### PASSWORD {i} ###\n")));
    }
    assert!(!text.contains("### PASSWORD ###"));
}

#[test]
fn one_password_with_number_one_is_still_numbered() {
    let output = pgen(&["-l", "8", "-n", "1"]);
    assert!(stdout(&output).starts_with("### PASSWORD 1 ###\n"));
}

#[test]
fn json_mode_is_an_array_of_the_requested_passwords() {
    let output = pgen(&["-l", "16", "-c", "4", "-n", "5", "-j"]);
    assert!(output.status.success());
    let passwords: Vec<String> = serde_json::from_str(&stdout(&output)).expect("valid JSON");
    assert_eq!(passwords.len(), 5);
    assert!(passwords.iter().all(|p| p.chars().count() == 16));
}

#[test]
fn json_mode_wraps_a_single_password_in_an_array() {
    let output = pgen(&["-l", "10", "-j"]);
    let passwords: Vec<String> = serde_json::from_str(&stdout(&output)).expect("valid JSON");
    assert_eq!(passwords.len(), 1);
    assert_eq!(passwords[0].chars().count(), 10);
}

#[test]
fn custom_dictionary_is_the_only_source_of_characters() {
    let output = pgen(&["-l", "200", "-d", "ab 1", "-c", "1", "-j"]);
    let passwords: Vec<String> = serde_json::from_str(&stdout(&output)).expect("valid JSON");
    assert!(passwords[0].chars().all(|c| "ab 1".contains(c)));
    assert!(
        passwords[0].contains(' '),
        "a space in the dictionary is sampled too"
    );
}

#[test]
fn multibyte_dictionary_is_sampled_by_character() {
    let output = pgen(&["-l", "20", "-d", "äöü€", "-j"]);
    assert!(output.status.success(), "{}", stderr(&output));
    let passwords: Vec<String> = serde_json::from_str(&stdout(&output)).expect("valid JSON");
    assert_eq!(passwords[0].chars().count(), 20);
    assert!(passwords[0].chars().all(|c| "äöü€".contains(c)));
}

#[test]
fn empty_dictionary_is_a_usage_error() {
    let output = pgen(&["-l", "8", "-d", ""]);
    assert_eq!(output.status.code(), Some(2));
    assert!(stderr(&output).contains("--dictionary"));
}

#[test]
fn complexity_outside_one_to_four_is_a_usage_error() {
    for bad in ["0", "5", "x"] {
        let output = pgen(&["-l", "8", "-c", bad]);
        assert_eq!(output.status.code(), Some(2), "-c {bad}");
        assert!(stderr(&output).contains("--complexity"));
    }
}

#[test]
fn output_file_is_written_relative_to_the_working_directory() {
    let dir = scratch_dir("relative-output");
    let output = Command::new(env!("CARGO_BIN_EXE_pgen"))
        .args(["-l", "8", "-n", "2", "-j", "-o", "passwords.json"])
        .current_dir(&dir)
        .output()
        .expect("pgen runs");
    assert!(output.status.success(), "{}", stderr(&output));
    assert!(
        stdout(&output).is_empty(),
        "nothing on stdout when writing a file"
    );
    let written = fs::read_to_string(dir.join("passwords.json")).expect("file written in cwd");
    let passwords: Vec<String> = serde_json::from_str(&written).expect("valid JSON");
    assert_eq!(passwords.len(), 2);
}

#[test]
fn output_file_in_text_mode_matches_stdout_format() {
    let dir = scratch_dir("text-output");
    let path = dir.join("out.txt");
    let output = pgen(&["-l", "8", "-o", path.to_str().unwrap()]);
    assert!(output.status.success(), "{}", stderr(&output));
    let written = fs::read_to_string(&path).expect("file written at the absolute path");
    assert!(written.starts_with("### PASSWORD ###\n"));
    assert!(written.ends_with("\n\n"));
}

#[test]
fn unwritable_output_file_fails_with_a_message_naming_it() {
    let dir = scratch_dir("missing-parent");
    let path = dir.join("no-such-dir").join("out.txt");
    let output = pgen(&["-l", "8", "-o", path.to_str().unwrap()]);
    assert_eq!(output.status.code(), Some(1));
    let message = stderr(&output);
    assert!(message.starts_with("pgen: "), "{message}");
    assert!(message.contains("no-such-dir"), "{message}");
    assert!(!message.contains("panicked"), "{message}");
}
