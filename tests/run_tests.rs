use lang_tester::LangTester;
use std::fs::read_to_string;
use std::process::Command;

static COMMENT_PREFIX: &str = "//";

#[test]
fn run_tests() {
    let mut path = std::env::var("PATH").unwrap_or_default();
    path.push_str(":./target/release");

    LangTester::new()
        .test_dir("tests/code_tests")
        .test_file_filter(|p| p.extension().unwrap().to_str().unwrap() == "osmx")
        .test_extract(|s| {
            read_to_string(s)
                .unwrap()
                .lines()
                .skip_while(|l| !l.starts_with(COMMENT_PREFIX))
                .take_while(|l| l.starts_with(COMMENT_PREFIX))
                .map(|l| &l[2..])
                .collect::<Vec<_>>()
                .join("\n")
        })
        .test_cmds(move |p| {
            let mut compiler = Command::new("osmon");
            compiler.args(&["--jit", p.to_str().unwrap()]);
            vec![("Compiler", compiler)]
        })
        .run();
}
