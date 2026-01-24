mod helpers;

use helpers::TestEnv;

mod package_manager {
    use super::*;

    fn run_pkm(env: &TestEnv, dir: &str) -> String {
        env.run_command_in(&["node", "package-manager"], dir)
    }

    #[test]
    fn default() {
        let env = &TestEnv::new();

        // ./
        // +-- src/
        env.create_dir_all("src");

        assert_eq!(run_pkm(env, "."), "npm");
        assert_eq!(run_pkm(env, "src"), "npm");
    }

    #[test]
    fn npm() {
        let env = &TestEnv::new();

        // ./
        // +-- package.json
        // +-- package-lock.json
        // +-- src/
        env.create_file("package.json");
        env.create_file("package-lock.json");
        env.create_dir_all("src");

        assert_eq!(run_pkm(env, "."), "npm");
        assert_eq!(run_pkm(env, "src"), "npm");
    }

    #[test]
    fn yarn() {
        let env = &TestEnv::new();

        // ./
        // +-- package.json
        // +-- yarn.lock
        // +-- src/
        env.create_file("package.json");
        env.create_file("yarn.lock");
        env.create_dir_all("src");

        assert_eq!(run_pkm(env, "."), "yarn");
        assert_eq!(run_pkm(env, "src"), "yarn");
    }

    #[test]
    fn pnpm() {
        let env = &TestEnv::new();

        // ./
        // +-- package.json
        // +-- pnpm-lock.yaml
        // +-- src/
        env.create_file("package.json");
        env.create_file("pnpm-lock.yaml");
        env.create_dir_all("src");

        assert_eq!(run_pkm(env, "."), "pnpm");
        assert_eq!(run_pkm(env, "src"), "pnpm");
    }
}

mod scripts {
    use super::*;

    fn run_scripts(env: &TestEnv, dir: &str) -> String {
        env.run_command_in(&["node", "scripts"], dir)
    }

    fn run_scripts_has(env: &TestEnv, dir: &str, script: &str) -> i32 {
        let (status, stdout, stderr) =
            env.run_command_output(&["node", "scripts", "--has", script], dir);
        assert_eq!(stdout, "");
        assert_eq!(stderr, "");
        status.code().unwrap()
    }

    #[test]
    fn none() {
        let env = &TestEnv::new();

        // ./
        // +-- src/
        env.create_dir_all("src");

        assert_eq!(run_scripts(env, "."), "");
        assert_eq!(run_scripts_has(env, ".", "start"), 1);
        assert_eq!(run_scripts_has(env, ".", "test"), 1);

        assert_eq!(run_scripts(env, "src"), "");
        assert_eq!(run_scripts_has(env, "src", "start"), 1);
        assert_eq!(run_scripts_has(env, "src", "test"), 1);
    }

    #[test]
    fn no_scripts() {
        let env = &TestEnv::new();

        // ./
        // +-- package.json
        // +-- src/
        env.create_file_with_content(
            "package.json",
            r#"
            {}
            "#,
        );
        env.create_dir_all("src");

        assert_eq!(run_scripts(env, "."), "");
        assert_eq!(run_scripts_has(env, ".", "start"), 1);
        assert_eq!(run_scripts_has(env, ".", "test"), 1);

        assert_eq!(run_scripts(env, "src"), "");
        assert_eq!(run_scripts_has(env, "src", "start"), 1);
        assert_eq!(run_scripts_has(env, "src", "test"), 1);
    }

    #[test]
    fn empty_scripts() {
        let env = &TestEnv::new();

        // ./
        // +-- package.json
        // +-- src/
        env.create_file_with_content(
            "package.json",
            r#"
            {
                "scripts": {}
            }
            "#,
        );
        env.create_dir_all("src");

        assert_eq!(run_scripts(env, "."), "");
        assert_eq!(run_scripts_has(env, ".", "start"), 1);
        assert_eq!(run_scripts_has(env, ".", "test"), 1);

        assert_eq!(run_scripts(env, "src"), "");
        assert_eq!(run_scripts_has(env, "src", "start"), 1);
        assert_eq!(run_scripts_has(env, "src", "test"), 1);
    }

    #[test]
    fn invalid_scripts() {
        let env = &TestEnv::new();

        // ./
        // +-- package.json
        // +-- src/
        env.create_file_with_content(
            "package.json",
            r#"
            {
                "scripts": {
                    "start": 0,
                    "test": "vitest"
                }
            }
            "#,
        );
        env.create_dir_all("src");

        assert_eq!(run_scripts(env, "."), "test\tvitest");
        assert_eq!(run_scripts_has(env, ".", "start"), 1);
        assert_eq!(run_scripts_has(env, ".", "test"), 0);

        assert_eq!(run_scripts(env, "src"), "test\tvitest");
        assert_eq!(run_scripts_has(env, "src", "start"), 1);
        assert_eq!(run_scripts_has(env, "src", "test"), 0);
    }

    #[test]
    fn single_script() {
        let env = &TestEnv::new();

        // ./
        // +-- package.json
        // +-- src/
        env.create_file_with_content(
            "package.json",
            r#"
            {
                "scripts": {
                    "start": "node server.js"
                }
            }
            "#,
        );
        env.create_dir_all("src");

        assert_eq!(run_scripts(env, "."), "start\tnode server.js");
        assert_eq!(run_scripts_has(env, ".", "start"), 0);
        assert_eq!(run_scripts_has(env, ".", "test"), 1);

        assert_eq!(run_scripts(env, "src"), "start\tnode server.js");
        assert_eq!(run_scripts_has(env, "src", "start"), 0);
        assert_eq!(run_scripts_has(env, "src", "test"), 1);
    }

    #[test]
    fn multi_scripts() {
        let env = &TestEnv::new();

        // ./
        // +-- package.json
        // +-- src/
        env.create_file_with_content(
            "package.json",
            r#"
            {
                "scripts": {
                    "start": "node server.js",
                    "test": "vitest",
                    "build": "tsc"
                }
            }
            "#,
        );
        env.create_dir_all("src");

        assert_eq!(
            run_scripts(env, "."),
            "build\ttsc\nstart\tnode server.js\ntest\tvitest"
        );
        assert_eq!(run_scripts_has(env, ".", "start"), 0);
        assert_eq!(run_scripts_has(env, ".", "test"), 0);
        assert_eq!(run_scripts_has(env, ".", "build"), 0);
        assert_eq!(run_scripts_has(env, ".", "dev"), 1);

        assert_eq!(
            run_scripts(env, "src"),
            "build\ttsc\nstart\tnode server.js\ntest\tvitest"
        );
        assert_eq!(run_scripts_has(env, "src", "start"), 0);
        assert_eq!(run_scripts_has(env, "src", "test"), 0);
        assert_eq!(run_scripts_has(env, "src", "build"), 0);
        assert_eq!(run_scripts_has(env, ".", "dev"), 1);
    }

    #[test]
    fn workspace() {
        let env = &TestEnv::new();

        // ./
        // +-- package.json
        // +-- src/
        // +-- workspace/
        //     +-- package.json
        //     +-- src/
        env.create_file_with_content(
            "package.json",
            r#"
            {
                "scripts": {
                    "start": "node server.js",
                    "test": "npm -w workspace test"
                },
                "workspaces": ["workspace"]
            }
            "#,
        );
        env.create_dir_all("src");
        env.create_file_with_content(
            "workspace/package.json",
            r#"
            {
                "scripts": {
                    "build": "tsc",
                    "test": "vitest"
                }
            }
            "#,
        );
        env.create_dir_all("workspace/src");

        assert_eq!(
            run_scripts(env, "."),
            "start\tnode server.js\ntest\tnpm -w workspace test"
        );
        assert_eq!(run_scripts_has(env, ".", "start"), 0);
        assert_eq!(run_scripts_has(env, ".", "test"), 0);
        assert_eq!(run_scripts_has(env, ".", "build"), 1);

        assert_eq!(
            run_scripts(env, "src"),
            "start\tnode server.js\ntest\tnpm -w workspace test"
        );
        assert_eq!(run_scripts_has(env, "src", "start"), 0);
        assert_eq!(run_scripts_has(env, "src", "test"), 0);
        assert_eq!(run_scripts_has(env, "src", "build"), 1);

        assert_eq!(run_scripts(env, "workspace"), "build\ttsc\ntest\tvitest");
        assert_eq!(run_scripts_has(env, "workspace", "start"), 1);
        assert_eq!(run_scripts_has(env, "workspace", "test"), 0);
        assert_eq!(run_scripts_has(env, "workspace", "build"), 0);

        assert_eq!(
            run_scripts(env, "workspace/src"),
            "build\ttsc\ntest\tvitest"
        );
        assert_eq!(run_scripts_has(env, "workspace/src", "start"), 1);
        assert_eq!(run_scripts_has(env, "workspace/src", "test"), 0);
        assert_eq!(run_scripts_has(env, "workspace/src", "build"), 0);
    }
}

mod toplevel {
    use super::*;

    fn run_toplevel(env: &TestEnv, dir: &str) -> String {
        env.run_command_in(&["node", "toplevel"], dir)
    }

    fn run_toplevel_root(env: &TestEnv, dir: &str) -> String {
        env.run_command_in(&["node", "toplevel", "--root"], dir)
    }

    #[test]
    fn package_lock() {
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

        let root = env.path().to_string_lossy().to_string();
        assert_eq!(run_toplevel(env, "."), root);
        assert_eq!(run_toplevel_root(env, "."), root);
        assert_eq!(run_toplevel(env, "src"), root);
        assert_eq!(run_toplevel_root(env, "src"), root);
    }

    #[test]
    fn no_lock() {
        let env = &TestEnv::new();

        // ./
        // +-- package.json
        // +-- index.js
        // +-- src/
        //     +-- index.js
        env.create_file("package.json");
        env.create_file("index.js");
        env.create_file("src/index.js");

        let root = env.path().to_string_lossy().to_string();
        assert_eq!(run_toplevel(env, "."), root);
        assert_eq!(run_toplevel_root(env, "."), root);
        assert_eq!(run_toplevel(env, "src"), root);
        assert_eq!(run_toplevel_root(env, "src"), root);
    }

    #[test]
    fn workspace_package_lock() {
        let env = &TestEnv::new();

        // ./
        // +-- package.json
        // +-- package-lock.json
        // +-- src/
        //     +-- index.js
        // +-- packages/
        //     +-- pkg-a/
        //         +-- package.json
        //         +-- index.js
        //     +-- pkg-b/
        //         +-- package.json
        //         +-- index.js
        env.create_file("package.json");
        env.create_file("package-lock.json");
        env.create_file("src/index.js");
        env.create_file("packages/pkg-a/package.json");
        env.create_file("packages/pkg-a/index.js");
        env.create_file("packages/pkg-b/package.json");
        env.create_file("packages/pkg-b/index.js");

        let root = env.path().to_string_lossy().to_string();
        assert_eq!(run_toplevel(env, "."), root);
        assert_eq!(run_toplevel_root(env, "."), root);
        assert_eq!(run_toplevel(env, "src"), root);
        assert_eq!(run_toplevel_root(env, "src"), root);
        assert_eq!(
            run_toplevel(env, "packages/pkg-a"),
            format!("{root}/packages/pkg-a")
        );
        assert_eq!(run_toplevel_root(env, "packages/pkg-a"), root);
        assert_eq!(
            run_toplevel(env, "packages/pkg-b"),
            format!("{root}/packages/pkg-b")
        );
        assert_eq!(run_toplevel_root(env, "packages/pkg-b"), root);
    }

    #[test]
    fn workspace_yarn_lock() {
        let env = &TestEnv::new();

        // ./
        // +-- package.json
        // +-- yarn.lock
        // +-- src/
        //     +-- index.js
        // +-- packages/
        //     +-- pkg-a/
        //         +-- package.json
        //         +-- index.js
        //     +-- pkg-b/
        //         +-- package.json
        //         +-- index.js
        env.create_file("package.json");
        env.create_file("yarn.lock");
        env.create_file("src/index.js");
        env.create_file("packages/pkg-a/package.json");
        env.create_file("packages/pkg-a/index.js");
        env.create_file("packages/pkg-b/package.json");
        env.create_file("packages/pkg-b/index.js");

        let root = env.path().to_string_lossy().to_string();
        assert_eq!(run_toplevel(env, "."), root);
        assert_eq!(run_toplevel_root(env, "."), root);
        assert_eq!(run_toplevel(env, "src"), root);
        assert_eq!(run_toplevel_root(env, "src"), root);
        assert_eq!(
            run_toplevel(env, "packages/pkg-a"),
            format!("{root}/packages/pkg-a")
        );
        assert_eq!(run_toplevel_root(env, "packages/pkg-a"), root);
        assert_eq!(
            run_toplevel(env, "packages/pkg-b"),
            format!("{root}/packages/pkg-b")
        );
        assert_eq!(run_toplevel_root(env, "packages/pkg-b"), root);
    }

    #[test]
    fn workspace_no_lock() {
        let env = &TestEnv::new();

        // ./
        // +-- package.json
        // +-- src/
        //     +-- index.js
        // +-- packages/
        //     +-- pkg-a/
        //         +-- package.json
        //         +-- index.js
        //     +-- pkg-b/
        //         +-- package.json
        //         +-- index.js
        env.create_file("package.json");
        env.create_file("src/index.js");
        env.create_file("packages/pkg-a/package.json");
        env.create_file("packages/pkg-a/index.js");
        env.create_file("packages/pkg-b/package.json");
        env.create_file("packages/pkg-b/index.js");

        let root = env.path().to_string_lossy().to_string();
        assert_eq!(run_toplevel(env, "."), root);
        assert_eq!(run_toplevel_root(env, "."), root);
        assert_eq!(run_toplevel(env, "src"), root);
        assert_eq!(run_toplevel_root(env, "src"), root);
        assert_eq!(
            run_toplevel(env, "packages/pkg-a"),
            format!("{root}/packages/pkg-a")
        );
        assert_eq!(
            run_toplevel_root(env, "packages/pkg-a"),
            format!("{root}/packages/pkg-a")
        );
        assert_eq!(
            run_toplevel(env, "packages/pkg-b"),
            format!("{root}/packages/pkg-b")
        );
        assert_eq!(
            run_toplevel_root(env, "packages/pkg-b"),
            format!("{root}/packages/pkg-b")
        );
    }
}
