use ccase::{UserCase, UserPattern};
use clap::ArgMatches;
use convert_case::{Boundary, Converter};
use is_terminal::IsTerminal;
use std::io::{self, stdin, BufRead};

fn main() {
    let mut app = ccase::build_app();

    let missing_error = app.error(
        clap::error::ErrorKind::MissingRequiredArgument,
        "The following required arguments were not provided:\n  \
            \x1b[32m<input>...\x1b[m",
    );

    let matches = app.get_matches();

    let inputs: Box<dyn Iterator<Item = String>> = match matches.get_many::<String>("input") {
        Some(values) => Box::new(values.cloned()),
        None => {
            if stdin_is_piped() {
                let stdin = io::stdin();
                let reader = io::BufReader::new(stdin.lock());
                Box::new(reader.lines().map(|l| l.unwrap()))
            } else {
                missing_error.exit();
            }
        }
    };

    inputs.for_each(|input| convert(&matches, &input));
}

fn convert(matches: &ArgMatches, input: &String) {
    let mut conv = Converter::new();

    if let Some(&from) = matches.get_one::<UserCase>("from") {
        conv = conv.from_case(from.into());
    } else if let Some(boundary_str) = matches.get_one::<String>("boundaries") {
        let boundaries = Boundary::defaults_from(boundary_str);
        conv = conv.set_boundaries(&boundaries);
    }

    if let Some(&to) = matches.get_one::<UserCase>("to") {
        conv = conv.to_case(to.into());
    } else if let Some(&pattern) = matches.get_one::<UserPattern>("pattern") {
        conv = conv.set_pattern(pattern.apply());

        if let Some(delim) = matches.get_one::<String>("delimeter") {
            conv = conv.set_delim(delim);
        }
    }

    println!("{}", conv.convert(input));
}

fn stdin_is_piped() -> bool {
    if cfg!(debug_assertions) {
        if let Ok(val) = std::env::var("CCASE_TEST_STDIN_IS_PIPED") {
            return val == "true";
        }
    }
    !stdin().is_terminal()
}

#[cfg(test)]
mod test {
    use assert_cmd::{assert::Assert, Command};
    use predicates::str::contains;

    fn ccase(args: &[&str]) -> Assert {
        Command::cargo_bin("ccase").unwrap().args(args).assert()
    }

    #[test]
    fn to_case() {
        ccase(&["-t", "snake", "myVarName"])
            .success()
            .stdout("my_var_name\n");
        ccase(&["--to", "kebab", "myVarName"])
            .success()
            .stdout("my-var-name\n");
        ccase(&["--to", "kebab", "my Var Name"])
            .success()
            .stdout("my-var-name\n");
    }

    #[test]
    fn from_case() {
        ccase(&["-f", "snake", "-t", "pascal", "my_var-name"])
            .success()
            .stdout("MyVar-name\n");
        ccase(&["-t", "snake", "--from", "pascal", "myVar-name"])
            .success()
            .stdout("my_var-name\n");
        ccase(&["-t", "snake", "--from", "lower", "my Var-name"])
            .success()
            .stdout("my_var-name\n");
    }

    #[test]
    fn to_required() {
        ccase(&["myvarname"])
            .failure()
            .stderr(contains("following required arguments"))
            .stderr(contains("--to"));
    }

    #[test]
    fn pattern_only() {
        ccase(&["-p", "capital", "MY_VAR_NAME"])
            .success()
            .stdout("MyVarName\n");
        ccase(&["-p", "Sentence", "MY_VAR_NAME"])
            .success()
            .stdout("Myvarname\n");
    }

    #[test]
    fn to_exclusive_with_pattern_delim() {
        ccase(&["-t", "snake", "-p", "capital", "MY_VAR_NAME"])
            .failure()
            .stderr(contains("--to <case>"))
            .stderr(contains("cannot be used with"))
            .stderr(contains("--pattern <pattern>"));
        ccase(&["-t", "snake", "-d", "-", "MY_VAR_NAME"])
            .failure()
            .stderr(contains("--to <case>"))
            .stderr(contains("cannot be used with"))
            .stderr(contains("--delimeter <string>"));
    }

    #[test]
    fn delimeter() {
        ccase(&["-p", "sentence", "-d", ".", "myVarName"])
            .success()
            .stdout("My.var.name\n");
    }

    #[test]
    fn input_required() {
        let assert = Command::cargo_bin("ccase")
            .unwrap()
            .arg("-t")
            .arg("snake")
            .env("CCASE_TEST_STDIN_IS_PIPED", "false")
            .assert();

        assert
            .failure()
            .stderr(contains("following required arguments"))
            .stderr(contains("input"));
    }

    #[test]
    fn help_default() {
        ccase(&[]).failure().stderr(contains("Usage"));
    }

    #[test]
    fn case_inputs_not_lower() {
        ccase(&["-t", "SNAKE", "myVarName"])
            .success()
            .stdout("my_var_name\n");
        ccase(&["-t", "SnAkE", "myVarName"])
            .success()
            .stdout("my_var_name\n");
        ccase(&["-t", "snake", "-f", "KEBab", "my-varName"])
            .success()
            .stdout("my_varname\n");
        ccase(&["-t", "snake", "-f", "KEBAB", "my-varName"])
            .success()
            .stdout("my_varname\n");
    }

    #[test]
    fn invalid_case() {
        ccase(&["-t", "SNEK", "myVarName"])
            .failure()
            .stderr(contains("variant not found"))
            .stderr(contains("--to"));
        ccase(&["-t", "snake", "-f", "SNEK", "my-varName"])
            .failure()
            .stderr(contains("variant not found"))
            .stderr(contains("--from"));
    }

    #[test]
    fn invalid_pattern() {
        ccase(&["-p", "SENT", "myVarName"])
            .failure()
            .stderr(contains("variant not found"))
            .stderr(contains("--pattern"));
        ccase(&["-p", "SENT", "-f", "snake", "my-varName"])
            .failure()
            .stderr(contains("variant not found"))
            .stderr(contains("--pattern"));
    }

    #[test]
    fn empty_string_input() {
        ccase(&["-t", "snake", r#""#]).success().stdout("\n");
    }

    #[test]
    fn boundaries() {
        ccase(&["-t", "snake", "-b", "aA", "myVar-Name-Longer"])
            .success()
            .stdout("my_var-name-longer\n");
        ccase(&["-t", "snake", "-b", "-", "myVar-Name-Longer"])
            .success()
            .stdout("myvar_name_longer\n");
    }

    #[test]
    fn from_and_boundaries_exclusive() {
        ccase(&["-t", "snake", "-b", "_", "-f", "kebab", "myVar-Name-Longer"])
            .failure()
            .stderr(contains("--from"))
            .stderr(contains("cannot be used with"))
            .stderr(contains("--boundaries"));
    }

    #[test]
    fn multiple_inputs() {
        ccase(&["-t", "snake", "myVarName", "anotherMultiWordToken"])
            .success()
            .stdout("my_var_name\nanother_multi_word_token\n");
    }

    #[test]
    fn help_contains_cases_and_patterns() {
        let output = Command::cargo_bin("ccase")
            .unwrap()
            .arg("--help")
            .assert()
            .get_output()
            .stdout
            .clone();
        let help_text = String::from_utf8_lossy(&output);
        assert!(help_text.contains("camel"));
        assert!(help_text.contains("noop"));
        assert!(help_text.contains("snake"));
        assert!(help_text.contains("title"));
        assert!(help_text.contains("train"));
    }

    #[test]
    fn pattern_with_custom_delimiter() {
        ccase(&["-p", "camel", "-d", "++", "my_var_name"])
            .success()
            .stdout("my++Var++Name\n");
    }

    mod stdin {
        use super::*;

        fn pipe_ccase(stdin: &str, args: &[&str]) -> Assert {
            Command::cargo_bin("ccase")
                .unwrap()
                .args(args)
                .write_stdin(stdin)
                .assert()
        }

        #[test]
        fn stdin() {
            pipe_ccase("myVarName", &["-t", "snake"])
                .success()
                .stdout("my_var_name\n");
        }

        #[test]
        fn newline_ending() {
            pipe_ccase("myVarName\n", &["-t", "snake"])
                .success()
                .stdout("my_var_name\n");
        }

        #[test]
        fn empty() {
            pipe_ccase(r#""#, &["-t", "snake"]).success().stdout("");
        }

        #[test]
        fn multiple_inputs() {
            pipe_ccase("myVarName\nanotherMultiWordToken\n", &["-t", "Pascal"])
                .success()
                .stdout("MyVarName\nAnotherMultiWordToken\n");
        }

        #[test]
        fn stdin_not_consumed_when_input_arg_provided() {
            let assert = Command::cargo_bin("ccase")
                .unwrap()
                .arg("--to")
                .arg("upper")
                .arg("hello")
                .write_stdin("foo\nbar\nbaz\n")
                .assert();

            assert.success().stdout("HELLO\n");
        }
    }
}
