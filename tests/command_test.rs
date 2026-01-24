mod helpers;

use helpers::TestEnv;

fn run_cmd(env: &TestEnv, dir: &str) -> String {
    env.run_command_in(&["command"], dir)
}

fn run_cmd_action(env: &TestEnv, action: &str, dir: &str) -> String {
    env.run_command_in(&["command", "--action", action], dir)
}

fn run_cmd_outputs(env: &TestEnv, dir: &str) -> (i32, String, String) {
    let (status, stdout, stderr) = env.run_command_output(&["command"], dir);
    (status.code().unwrap(), stdout, stderr)
}

fn run_cmd_action_outputs(env: &TestEnv, action: &str, dir: &str) -> (i32, String, String) {
    let (status, stdout, stderr) = env.run_command_output(&["command", "--action", action], dir);
    (status.code().unwrap(), stdout, stderr)
}

#[test]
fn npm() {
    let env = &TestEnv::new();

    // ./
    // +-- package.json
    // +-- package-lock.json
    // +-- index.js
    // +-- src/
    //     +-- index.js
    env.create_file("package.json");
    env.create_file("package-lock.json");
    env.create_file("index.js");
    env.create_file("src/index.js");

    assert_eq!(run_cmd(env, "."), "npm");
    assert_eq!(run_cmd_action(env, "build", "."), "npm run build");
    assert_eq!(run_cmd_action(env, "run", "."), "npm start");
    assert_eq!(run_cmd_action(env, "test", "."), "npm test");
    assert_eq!(run_cmd_action(env, "lint", "."), "npm run lint");
    assert_eq!(run_cmd_action(env, "format", "."), "npm run fmt");

    assert_eq!(run_cmd(env, "src"), "npm");
    assert_eq!(run_cmd_action(env, "build", "src"), "npm run build");
    assert_eq!(run_cmd_action(env, "run", "src"), "npm start");
    assert_eq!(run_cmd_action(env, "test", "src"), "npm test");
    assert_eq!(run_cmd_action(env, "lint", "src"), "npm run lint");
    assert_eq!(run_cmd_action(env, "format", "src"), "npm run fmt");
}

#[test]
fn yarn() {
    let env = &TestEnv::new();

    // ./
    // +-- package.json
    // +-- yarn.lock
    // +-- index.js
    // +-- src/
    //     +-- index.js
    env.create_file("package.json");
    env.create_file("yarn.lock");
    env.create_file("index.js");
    env.create_file("src/index.js");

    assert_eq!(run_cmd(env, "."), "yarn");
    assert_eq!(run_cmd_action(env, "build", "."), "yarn build");
    assert_eq!(run_cmd_action(env, "run", "."), "yarn start");
    assert_eq!(run_cmd_action(env, "test", "."), "yarn test");
    assert_eq!(run_cmd_action(env, "lint", "."), "yarn lint");
    assert_eq!(run_cmd_action(env, "format", "."), "yarn fmt");

    assert_eq!(run_cmd(env, "src"), "yarn");
    assert_eq!(run_cmd_action(env, "build", "src"), "yarn build");
    assert_eq!(run_cmd_action(env, "run", "src"), "yarn start");
    assert_eq!(run_cmd_action(env, "test", "src"), "yarn test");
    assert_eq!(run_cmd_action(env, "lint", "src"), "yarn lint");
    assert_eq!(run_cmd_action(env, "format", "src"), "yarn fmt");
}

#[test]
fn pnpm() {
    let env = &TestEnv::new();

    // ./
    // +-- package.json
    // +-- pnpm-lock.yaml
    // +-- index.js
    // +-- src/
    //     +-- index.js
    env.create_file("package.json");
    env.create_file("pnpm-lock.yaml");
    env.create_file("index.js");
    env.create_file("src/index.js");

    assert_eq!(run_cmd(env, "."), "pnpm");
    assert_eq!(run_cmd_action(env, "build", "."), "pnpm build");
    assert_eq!(run_cmd_action(env, "run", "."), "pnpm start");
    assert_eq!(run_cmd_action(env, "test", "."), "pnpm test");
    assert_eq!(run_cmd_action(env, "lint", "."), "pnpm lint");
    assert_eq!(run_cmd_action(env, "format", "."), "pnpm fmt");

    assert_eq!(run_cmd(env, "src"), "pnpm");
    assert_eq!(run_cmd_action(env, "build", "src"), "pnpm build");
    assert_eq!(run_cmd_action(env, "run", "src"), "pnpm start");
    assert_eq!(run_cmd_action(env, "test", "src"), "pnpm test");
    assert_eq!(run_cmd_action(env, "lint", "src"), "pnpm lint");
    assert_eq!(run_cmd_action(env, "format", "src"), "pnpm fmt");
}

#[test]
fn deno() {
    let env = &TestEnv::new();

    // ./
    // +-- deno.json
    // +-- index.ts
    // +-- src/
    //     +-- index.ts
    env.create_file("deno.json");
    env.create_file("index.ts");
    env.create_file("src/index.ts");

    assert_eq!(run_cmd(env, "."), "deno");
    assert_eq!(run_cmd_action(env, "build", "."), "deno task build");
    assert_eq!(run_cmd_action(env, "run", "."), "deno task start");
    assert_eq!(run_cmd_action(env, "test", "."), "deno test");
    assert_eq!(run_cmd_action(env, "lint", "."), "deno lint");
    assert_eq!(run_cmd_action(env, "format", "."), "deno fmt");

    assert_eq!(run_cmd(env, "src"), "deno");
    assert_eq!(run_cmd_action(env, "build", "src"), "deno task build");
    assert_eq!(run_cmd_action(env, "run", "src"), "deno task start");
    assert_eq!(run_cmd_action(env, "test", "src"), "deno test");
    assert_eq!(run_cmd_action(env, "lint", "src"), "deno lint");
    assert_eq!(run_cmd_action(env, "format", "src"), "deno fmt");
}

#[test]
fn makefile() {
    let env = &TestEnv::new();

    // ./
    // +-- Makefile
    // +-- src/
    //     +-- main.c
    env.create_file("Makefile");
    env.create_file("src/main.c");

    assert_eq!(run_cmd(env, "."), "make");
    assert_eq!(run_cmd_action(env, "build", "."), "make");
    assert_eq!(run_cmd_action(env, "run", "."), "make run");
    assert_eq!(run_cmd_action(env, "test", "."), "make test");
    assert_eq!(run_cmd_action(env, "lint", "."), "make lint");
    assert_eq!(run_cmd_action(env, "format", "."), "make fmt");

    assert_eq!(run_cmd(env, "src"), "make");
    assert_eq!(run_cmd_action(env, "build", "src"), "make");
    assert_eq!(run_cmd_action(env, "run", "src"), "make run");
    assert_eq!(run_cmd_action(env, "test", "src"), "make test");
    assert_eq!(run_cmd_action(env, "lint", "src"), "make lint");
    assert_eq!(run_cmd_action(env, "format", "src"), "make fmt");
}

#[test]
fn cargo() {
    let env = &TestEnv::new();

    // ./
    // +-- Cargo.toml
    // +-- src/
    //     +-- main.rs
    env.create_file("Cargo.toml");
    env.create_file("src/main.rs");

    assert_eq!(run_cmd(env, "."), "cargo");
    assert_eq!(run_cmd_action(env, "build", "."), "cargo build");
    assert_eq!(run_cmd_action(env, "run", "."), "cargo run");
    assert_eq!(run_cmd_action(env, "test", "."), "cargo test");
    assert_eq!(run_cmd_action(env, "lint", "."), "cargo clippy");
    assert_eq!(run_cmd_action(env, "format", "."), "cargo fmt");

    assert_eq!(run_cmd(env, "src"), "cargo");
    assert_eq!(run_cmd_action(env, "build", "src"), "cargo build");
    assert_eq!(run_cmd_action(env, "run", "src"), "cargo run");
    assert_eq!(run_cmd_action(env, "test", "src"), "cargo test");
    assert_eq!(run_cmd_action(env, "lint", "src"), "cargo clippy");
    assert_eq!(run_cmd_action(env, "format", "src"), "cargo fmt");
}

#[test]
fn go() {
    let env = &TestEnv::new();

    // ./
    // +-- go.mod
    // +-- main.go
    // +-- pkg/
    //     +-- pkg.go
    env.create_file("go.mod");
    env.create_file("main.go");
    env.create_file("pkg/pkg.go");

    assert_eq!(run_cmd(env, "."), "go");
    assert_eq!(run_cmd_action(env, "build", "."), "go build");
    assert_eq!(run_cmd_action(env, "run", "."), "go run .");
    assert_eq!(run_cmd_action(env, "test", "."), "go test ./...");
    assert_eq!(run_cmd_action(env, "lint", "."), "golangci-lint run");
    assert_eq!(run_cmd_action(env, "format", "."), "goimports -w .");

    assert_eq!(run_cmd(env, "pkg"), "go");
    assert_eq!(run_cmd_action(env, "build", "pkg"), "go build");
    assert_eq!(run_cmd_action(env, "run", "pkg"), "go run .");
    assert_eq!(run_cmd_action(env, "test", "pkg"), "go test ./...");
    assert_eq!(run_cmd_action(env, "lint", "pkg"), "golangci-lint run");
    assert_eq!(run_cmd_action(env, "format", "pkg"), "goimports -w .");
}

#[test]
fn unknown() {
    let env = &TestEnv::new();

    // ./
    // +-- README.md
    // +-- src/
    env.create_file("README.md");
    env.create_dir_all("src");

    assert_eq!(
        run_cmd_outputs(env, "."),
        (1, "".to_string(), "".to_string())
    );
    assert_eq!(
        run_cmd_action_outputs(env, "build", "."),
        (1, "".to_string(), "".to_string())
    );
    assert_eq!(
        run_cmd_action_outputs(env, "run", "."),
        (1, "".to_string(), "".to_string())
    );
    assert_eq!(
        run_cmd_action_outputs(env, "test", "."),
        (1, "".to_string(), "".to_string())
    );
    assert_eq!(
        run_cmd_action_outputs(env, "lint", "."),
        (1, "".to_string(), "".to_string())
    );
    assert_eq!(
        run_cmd_action_outputs(env, "format", "."),
        (1, "".to_string(), "".to_string())
    );

    assert_eq!(
        run_cmd_outputs(env, "src"),
        (1, "".to_string(), "".to_string())
    );
    assert_eq!(
        run_cmd_action_outputs(env, "build", "src"),
        (1, "".to_string(), "".to_string())
    );
    assert_eq!(
        run_cmd_action_outputs(env, "run", "src"),
        (1, "".to_string(), "".to_string())
    );
    assert_eq!(
        run_cmd_action_outputs(env, "test", "src"),
        (1, "".to_string(), "".to_string())
    );
    assert_eq!(
        run_cmd_action_outputs(env, "lint", "src"),
        (1, "".to_string(), "".to_string())
    );
    assert_eq!(
        run_cmd_action_outputs(env, "format", "src"),
        (1, "".to_string(), "".to_string())
    );
}
