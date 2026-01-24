mod helpers;

use helpers::TestEnv;

mod tasks {
    use super::*;

    fn run_tasks(env: &TestEnv, dir: &str) -> String {
        env.run_command_in(&["deno", "tasks"], dir)
    }

    fn run_tasks_has(env: &TestEnv, dir: &str, script: &str) -> i32 {
        let (status, stdout, stderr) =
            env.run_command_output(&["deno", "tasks", "--has", script], dir);
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

        assert_eq!(run_tasks(env, "."), "");
        assert_eq!(run_tasks_has(env, ".", "start"), 1);
        assert_eq!(run_tasks_has(env, ".", "test"), 1);

        assert_eq!(run_tasks(env, "src"), "");
        assert_eq!(run_tasks_has(env, "src", "start"), 1);
        assert_eq!(run_tasks_has(env, "src", "test"), 1);
    }

    #[test]
    fn no_tasks() {
        let env = &TestEnv::new();

        // ./
        // +-- deno.json
        // +-- src/
        env.create_file_with_content(
            "deno.json",
            r#"
            {}
            "#,
        );
        env.create_dir_all("src");

        assert_eq!(run_tasks(env, "."), "");
        assert_eq!(run_tasks_has(env, ".", "start"), 1);
        assert_eq!(run_tasks_has(env, ".", "test"), 1);

        assert_eq!(run_tasks(env, "src"), "");
        assert_eq!(run_tasks_has(env, "src", "start"), 1);
        assert_eq!(run_tasks_has(env, "src", "test"), 1);
    }

    #[test]
    fn empty_tasks() {
        let env = &TestEnv::new();

        // ./
        // +-- deno.json
        // +-- src/
        env.create_file_with_content(
            "deno.json",
            r#"
            {
                "tasks": {}
            }
            "#,
        );
        env.create_dir_all("src");

        assert_eq!(run_tasks(env, "."), "");
        assert_eq!(run_tasks_has(env, ".", "start"), 1);
        assert_eq!(run_tasks_has(env, ".", "test"), 1);

        assert_eq!(run_tasks(env, "src"), "");
        assert_eq!(run_tasks_has(env, "src", "start"), 1);
        assert_eq!(run_tasks_has(env, "src", "test"), 1);
    }

    #[test]
    fn invalid_tasks() {
        let env = &TestEnv::new();

        // ./
        // +-- deno.json
        // +-- src/
        env.create_file_with_content(
            "deno.json",
            r#"
            {
                "tasks": {
                    "start": 0,
                    "test": "deno test"
                }
            }
            "#,
        );
        env.create_dir_all("src");

        assert_eq!(run_tasks(env, "."), "test\tdeno test");
        assert_eq!(run_tasks_has(env, ".", "start"), 1);
        assert_eq!(run_tasks_has(env, ".", "test"), 0);

        assert_eq!(run_tasks(env, "src"), "test\tdeno test");
        assert_eq!(run_tasks_has(env, "src", "start"), 1);
        assert_eq!(run_tasks_has(env, "src", "test"), 0);
    }

    #[test]
    fn single_task() {
        let env = &TestEnv::new();

        // ./
        // +-- deno.json
        // +-- src/
        env.create_file_with_content(
            "deno.json",
            r#"
            {
                "tasks": {
                    "start": "deno run main.ts"
                }
            }
            "#,
        );
        env.create_dir_all("src");

        assert_eq!(run_tasks(env, "."), "start\tdeno run main.ts");
        assert_eq!(run_tasks_has(env, ".", "start"), 0);
        assert_eq!(run_tasks_has(env, ".", "test"), 1);

        assert_eq!(run_tasks(env, "src"), "start\tdeno run main.ts");
        assert_eq!(run_tasks_has(env, "src", "start"), 0);
        assert_eq!(run_tasks_has(env, "src", "test"), 1);
    }

    #[test]
    fn multi_tasks() {
        let env = &TestEnv::new();

        // ./
        // +-- deno.json
        // +-- src/
        env.create_file_with_content(
            "deno.json",
            r#"
            {
                "tasks": {
                    "start": "deno run main.ts",
                    "test": "deno test"
                }
            }
            "#,
        );
        env.create_dir_all("src");

        assert_eq!(
            run_tasks(env, "."),
            "start\tdeno run main.ts\ntest\tdeno test"
        );
        assert_eq!(run_tasks_has(env, ".", "start"), 0);
        assert_eq!(run_tasks_has(env, ".", "test"), 0);
        assert_eq!(run_tasks_has(env, ".", "build"), 1);

        assert_eq!(
            run_tasks(env, "src"),
            "start\tdeno run main.ts\ntest\tdeno test"
        );
        assert_eq!(run_tasks_has(env, "src", "start"), 0);
        assert_eq!(run_tasks_has(env, "src", "test"), 0);
        assert_eq!(run_tasks_has(env, "src", "build"), 1);
    }
}

mod toplevel {
    use super::*;

    fn run_toplevel(env: &TestEnv, dir: &str) -> String {
        env.run_command_in(&["deno", "toplevel"], dir)
    }

    #[test]
    fn no_deno_json() {
        let env = &TestEnv::new();

        // ./
        // +-- index.ts
        // +-- src/
        //     +-- index.ts
        env.create_file("index.ts");
        env.create_file("src/index.ts");

        let root = env.path().to_string_lossy().to_string();
        assert_eq!(run_toplevel(env, "."), root);
        assert_eq!(run_toplevel(env, "src"), format!("{root}/src"));
    }

    #[test]
    fn deno_json() {
        let env = &TestEnv::new();

        // ./
        // +-- deno.json
        // +-- index.ts
        // +-- src/
        //     +-- index.ts
        env.create_file("deno.json");
        env.create_file("index.ts");
        env.create_file("src/index.ts");

        let root = env.path().to_string_lossy().to_string();
        assert_eq!(run_toplevel(env, "."), root);
        assert_eq!(run_toplevel(env, "src"), root);
    }
}
