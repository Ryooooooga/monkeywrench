use std::{
    env,
    fs::{File, canonicalize, create_dir_all},
    io::Write,
    path::PathBuf,
    process::Command,
};
use tempfile::TempDir;

pub struct TestEnv {
    test_dir: TempDir,
}

#[allow(unused)]
impl TestEnv {
    pub fn new() -> Self {
        let test_dir = TempDir::new().expect("Failed to create temp dir");
        Self { test_dir }
    }

    pub fn create_file(&self, path: &str) {
        let fullpath = self.path().join(path);
        create_dir_all(fullpath.parent().unwrap()).expect("Failed to create directory");
        File::create_new(fullpath).expect("Failed to create file");
    }

    pub fn create_file_with_content(&self, path: &str, content: &str) {
        let fullpath = self.path().join(path);
        create_dir_all(fullpath.parent().unwrap()).expect("Failed to create directory");
        let mut file = File::create_new(fullpath).expect("Failed to create file");
        file.write_all(content.as_bytes())
            .expect("Failed to write file");
    }

    pub fn create_dir_all(&self, path: &str) {
        let fullpath = self.path().join(path);
        create_dir_all(fullpath).expect("Failed to create directory");
    }

    pub fn run_command(&self, args: &[&str]) -> String {
        self.run_command_in(args, ".")
    }

    pub fn run_command_in(&self, args: &[&str], dir: &str) -> String {
        let (stdout, stderr) = self.run_command_output(args, dir).unwrap();
        assert_eq!(stderr, "");
        stdout.to_string()
    }

    pub fn run_command_output(
        &self,
        args: &[&str],
        dir: &str,
    ) -> Result<(String, String), (i32, String)> {
        let command_path = self.path().join(dir);

        let output = cli()
            .args(args)
            .current_dir(command_path)
            .output()
            .expect("Failed to execute command");
        let stdout = String::from_utf8_lossy(&output.stdout).to_string();
        let stderr = String::from_utf8_lossy(&output.stderr).to_string();

        if output.status.success() {
            Ok((stdout, stderr))
        } else {
            Err((output.status.code().unwrap_or(-1), stderr))
        }
    }

    pub fn path(&self) -> PathBuf {
        canonicalize(self.test_dir.path()).expect("Failed to canonicalize path")
    }
}

fn cli() -> Command {
    let mut cmd = Command::new(env!("CARGO_BIN_EXE_monkeywrench"));
    cmd.env_clear();
    cmd
}
