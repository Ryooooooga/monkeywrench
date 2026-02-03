mod helpers;

use helpers::TestEnv;

fn run_cmd(env: &TestEnv, dir: &str) -> String {
    let (stdout, stderr) = run_cmd_output(env, dir).unwrap();
    assert_eq!(stderr, "");
    stdout
}

fn run_cmd_action(env: &TestEnv, action: &str, dir: &str) -> String {
    let (stdout, stderr) = run_cmd_action_output(env, action, dir).unwrap();
    assert_eq!(stderr, "");
    stdout
}

fn run_cmd_output(env: &TestEnv, dir: &str) -> Result<(String, String), (i32, String)> {
    env.run_command_output(&["command"], dir)
}

fn run_cmd_action_output(
    env: &TestEnv,
    action: &str,
    dir: &str,
) -> Result<(String, String), (i32, String)> {
    env.run_command_output(&["command", "--action", action], dir)
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

    assert_eq!(run_cmd(env, "."), "npm\n");
    assert_eq!(run_cmd_action(env, "build", "."), "npm run build\n");
    assert_eq!(run_cmd_action(env, "run", "."), "npm start\n");
    assert_eq!(run_cmd_action(env, "test", "."), "npm test\n");
    assert_eq!(run_cmd_action(env, "lint", "."), "npm run lint\n");
    assert_eq!(run_cmd_action(env, "format", "."), "npm run fmt\n");
    assert_eq!(run_cmd_action(env, "generate", "."), "npm run gen\n");

    assert_eq!(run_cmd(env, "src"), "npm\n");
    assert_eq!(run_cmd_action(env, "build", "src"), "npm run build\n");
    assert_eq!(run_cmd_action(env, "run", "src"), "npm start\n");
    assert_eq!(run_cmd_action(env, "test", "src"), "npm test\n");
    assert_eq!(run_cmd_action(env, "lint", "src"), "npm run lint\n");
    assert_eq!(run_cmd_action(env, "format", "src"), "npm run fmt\n");
    assert_eq!(run_cmd_action(env, "generate", "src"), "npm run gen\n");
}

#[test]
fn npm_workspace_with_makefile() {
    let env = &TestEnv::new();

    // ./
    // +-- package.json
    // +-- package-lock.json
    // +-- workspace/
    //     +-- package.json
    //     +-- Makefile
    //     +-- index.js
    env.create_file("package.json");
    env.create_file("package-lock.json");
    env.create_file("index.js");
    env.create_file("workspace/package.json");
    env.create_file("workspace/Makefile");
    env.create_file("workspace/index.js");

    assert_eq!(run_cmd(env, "."), "npm\n");
    assert_eq!(run_cmd_action(env, "build", "."), "npm run build\n");
    assert_eq!(run_cmd_action(env, "run", "."), "npm start\n");
    assert_eq!(run_cmd_action(env, "test", "."), "npm test\n");
    assert_eq!(run_cmd_action(env, "lint", "."), "npm run lint\n");
    assert_eq!(run_cmd_action(env, "format", "."), "npm run fmt\n");
    assert_eq!(run_cmd_action(env, "generate", "."), "npm run gen\n");

    assert_eq!(run_cmd(env, "workspace"), "npm\n");
    assert_eq!(run_cmd_action(env, "build", "workspace"), "npm run build\n");
    assert_eq!(run_cmd_action(env, "run", "workspace"), "npm start\n");
    assert_eq!(run_cmd_action(env, "test", "workspace"), "npm test\n");
    assert_eq!(run_cmd_action(env, "lint", "workspace"), "npm run lint\n");
    assert_eq!(run_cmd_action(env, "format", "workspace"), "npm run fmt\n");
    assert_eq!(
        run_cmd_action(env, "generate", "workspace"),
        "npm run gen\n"
    );
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

    assert_eq!(run_cmd(env, "."), "yarn\n");
    assert_eq!(run_cmd_action(env, "build", "."), "yarn build\n");
    assert_eq!(run_cmd_action(env, "run", "."), "yarn start\n");
    assert_eq!(run_cmd_action(env, "test", "."), "yarn test\n");
    assert_eq!(run_cmd_action(env, "lint", "."), "yarn lint\n");
    assert_eq!(run_cmd_action(env, "format", "."), "yarn fmt\n");
    assert_eq!(run_cmd_action(env, "generate", "."), "yarn gen\n");

    assert_eq!(run_cmd(env, "src"), "yarn\n");
    assert_eq!(run_cmd_action(env, "build", "src"), "yarn build\n");
    assert_eq!(run_cmd_action(env, "run", "src"), "yarn start\n");
    assert_eq!(run_cmd_action(env, "test", "src"), "yarn test\n");
    assert_eq!(run_cmd_action(env, "lint", "src"), "yarn lint\n");
    assert_eq!(run_cmd_action(env, "format", "src"), "yarn fmt\n");
    assert_eq!(run_cmd_action(env, "generate", "src"), "yarn gen\n");
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

    assert_eq!(run_cmd(env, "."), "pnpm\n");
    assert_eq!(run_cmd_action(env, "build", "."), "pnpm build\n");
    assert_eq!(run_cmd_action(env, "run", "."), "pnpm start\n");
    assert_eq!(run_cmd_action(env, "test", "."), "pnpm test\n");
    assert_eq!(run_cmd_action(env, "lint", "."), "pnpm lint\n");
    assert_eq!(run_cmd_action(env, "format", "."), "pnpm fmt\n");
    assert_eq!(run_cmd_action(env, "generate", "."), "pnpm gen\n");

    assert_eq!(run_cmd(env, "src"), "pnpm\n");
    assert_eq!(run_cmd_action(env, "build", "src"), "pnpm build\n");
    assert_eq!(run_cmd_action(env, "run", "src"), "pnpm start\n");
    assert_eq!(run_cmd_action(env, "test", "src"), "pnpm test\n");
    assert_eq!(run_cmd_action(env, "lint", "src"), "pnpm lint\n");
    assert_eq!(run_cmd_action(env, "format", "src"), "pnpm fmt\n");
    assert_eq!(run_cmd_action(env, "generate", "src"), "pnpm gen\n");
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

    assert_eq!(run_cmd(env, "."), "deno\n");
    assert_eq!(run_cmd_action(env, "build", "."), "deno task build\n");
    assert_eq!(run_cmd_action(env, "run", "."), "deno task start\n");
    assert_eq!(run_cmd_action(env, "test", "."), "deno test\n");
    assert_eq!(run_cmd_action(env, "lint", "."), "deno lint\n");
    assert_eq!(run_cmd_action(env, "format", "."), "deno fmt\n");
    assert_eq!(run_cmd_action(env, "generate", "."), "deno task gen\n");

    assert_eq!(run_cmd(env, "src"), "deno\n");
    assert_eq!(run_cmd_action(env, "build", "src"), "deno task build\n");
    assert_eq!(run_cmd_action(env, "run", "src"), "deno task start\n");
    assert_eq!(run_cmd_action(env, "test", "src"), "deno test\n");
    assert_eq!(run_cmd_action(env, "lint", "src"), "deno lint\n");
    assert_eq!(run_cmd_action(env, "format", "src"), "deno fmt\n");
    assert_eq!(run_cmd_action(env, "generate", "src"), "deno task gen\n");
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

    assert_eq!(run_cmd(env, "."), "make\n");
    assert_eq!(run_cmd_action(env, "build", "."), "make\n");
    assert_eq!(run_cmd_action(env, "run", "."), "make run\n");
    assert_eq!(run_cmd_action(env, "test", "."), "make test\n");
    assert_eq!(run_cmd_action(env, "lint", "."), "make lint\n");
    assert_eq!(run_cmd_action(env, "format", "."), "make fmt\n");
    assert_eq!(run_cmd_action(env, "generate", "."), "make gen\n");

    assert_eq!(run_cmd(env, "src"), "make\n");
    assert_eq!(run_cmd_action(env, "build", "src"), "make\n");
    assert_eq!(run_cmd_action(env, "run", "src"), "make run\n");
    assert_eq!(run_cmd_action(env, "test", "src"), "make test\n");
    assert_eq!(run_cmd_action(env, "lint", "src"), "make lint\n");
    assert_eq!(run_cmd_action(env, "format", "src"), "make fmt\n");
    assert_eq!(run_cmd_action(env, "generate", "src"), "make gen\n");
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

    assert_eq!(run_cmd(env, "."), "cargo\n");
    assert_eq!(run_cmd_action(env, "build", "."), "cargo build\n");
    assert_eq!(run_cmd_action(env, "run", "."), "cargo run\n");
    assert_eq!(run_cmd_action(env, "test", "."), "cargo test\n");
    assert_eq!(run_cmd_action(env, "lint", "."), "cargo clippy\n");
    assert_eq!(run_cmd_action(env, "format", "."), "cargo fmt\n");
    assert_eq!(
        run_cmd_action_output(env, "generate", ".").unwrap_err(),
        (1, "".to_string())
    );

    assert_eq!(run_cmd(env, "src"), "cargo\n");
    assert_eq!(run_cmd_action(env, "build", "src"), "cargo build\n");
    assert_eq!(run_cmd_action(env, "run", "src"), "cargo run\n");
    assert_eq!(run_cmd_action(env, "test", "src"), "cargo test\n");
    assert_eq!(run_cmd_action(env, "lint", "src"), "cargo clippy\n");
    assert_eq!(run_cmd_action(env, "format", "src"), "cargo fmt\n");
    assert_eq!(
        run_cmd_action_output(env, "generate", "src").unwrap_err(),
        (1, "".to_string())
    );
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

    assert_eq!(run_cmd(env, "."), "go\n");
    assert_eq!(run_cmd_action(env, "build", "."), "go build\n");
    assert_eq!(run_cmd_action(env, "run", "."), "go run .\n");
    assert_eq!(run_cmd_action(env, "test", "."), "go test ./...\n");
    assert_eq!(run_cmd_action(env, "lint", "."), "golangci-lint run\n");
    assert_eq!(run_cmd_action(env, "format", "."), "goimports -w .\n");
    assert_eq!(run_cmd_action(env, "generate", "."), "go gen -v ./...\n");

    assert_eq!(run_cmd(env, "pkg"), "go\n");
    assert_eq!(run_cmd_action(env, "build", "pkg"), "go build\n");
    assert_eq!(run_cmd_action(env, "run", "pkg"), "go run .\n");
    assert_eq!(run_cmd_action(env, "test", "pkg"), "go test ./...\n");
    assert_eq!(run_cmd_action(env, "lint", "pkg"), "golangci-lint run\n");
    assert_eq!(run_cmd_action(env, "format", "pkg"), "goimports -w .\n");
    assert_eq!(run_cmd_action(env, "generate", "pkg"), "go gen -v ./...\n");
}

#[test]
fn unknown() {
    let env = &TestEnv::new();

    // ./
    // +-- README.md
    // +-- src/
    env.create_file("README.md");
    env.create_dir_all("src");

    assert_eq!(run_cmd_output(env, ".").unwrap_err(), (1, "".to_string()));
    assert_eq!(
        run_cmd_action_output(env, "build", ".").unwrap_err(),
        (1, "".to_string())
    );
    assert_eq!(
        run_cmd_action_output(env, "run", ".").unwrap_err(),
        (1, "".to_string())
    );
    assert_eq!(
        run_cmd_action_output(env, "test", ".").unwrap_err(),
        (1, "".to_string())
    );
    assert_eq!(
        run_cmd_action_output(env, "lint", ".").unwrap_err(),
        (1, "".to_string())
    );
    assert_eq!(
        run_cmd_action_output(env, "format", ".").unwrap_err(),
        (1, "".to_string())
    );
    assert_eq!(
        run_cmd_action_output(env, "generate", ".").unwrap_err(),
        (1, "".to_string())
    );

    assert_eq!(run_cmd_output(env, "src").unwrap_err(), (1, "".to_string()));
    assert_eq!(
        run_cmd_action_output(env, "build", "src").unwrap_err(),
        (1, "".to_string())
    );
    assert_eq!(
        run_cmd_action_output(env, "run", "src").unwrap_err(),
        (1, "".to_string())
    );
    assert_eq!(
        run_cmd_action_output(env, "test", "src").unwrap_err(),
        (1, "".to_string())
    );
    assert_eq!(
        run_cmd_action_output(env, "lint", "src").unwrap_err(),
        (1, "".to_string())
    );
    assert_eq!(
        run_cmd_action_output(env, "format", "src").unwrap_err(),
        (1, "".to_string())
    );
    assert_eq!(
        run_cmd_action_output(env, "generate", "src").unwrap_err(),
        (1, "".to_string())
    );
}
