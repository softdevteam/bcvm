use lang_tester::LangTester;
use std::{fs::read_to_string, process::Command};
use tempfile::NamedTempFile;

const COMMENT: &str = "//";

fn main() {
    let temp_path = NamedTempFile::new().unwrap().into_temp_path();
    LangTester::new()
        .test_dir("lang_tests")
        .test_file_filter(|p| p.extension().map(|x| x.to_str().unwrap()) == Some("c"))
        .test_extract(move |p| {
            read_to_string(p)
                .unwrap()
                .lines()
                .skip_while(|l| !l.starts_with(COMMENT))
                .take_while(|l| l.starts_with(COMMENT))
                .map(|l| &l[COMMENT.len()..])
                .collect::<Vec<_>>()
                .join("\n")
        })
        .test_cmds(move |p| {
            let temp_path_str = temp_path.to_str().unwrap();
            let mut compiler = Command::new("clang");
            compiler.args(&["-emit-llvm", "-o", temp_path_str, "-c", p.to_str().unwrap()]);
            let mut runtime = Command::new("cargo");
            runtime.args(&["run", "--release", temp_path_str]);
            vec![("Compiler", compiler), ("Runtime", runtime)]
        })
        .run();
}
