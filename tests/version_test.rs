mod helpers;

use helpers::TestEnv;

mod inc {
    use super::*;

    fn run_inc(env: &TestEnv, input: &str, level: i32) -> String {
        env.run_command(&["version", "inc", "--level", &level.to_string(), input])
    }

    #[test]
    fn test() {
        let env = &TestEnv::new();

        assert_eq!(run_inc(env, "a", 1), "a\n");
        assert_eq!(run_inc(env, "a", 2), "a\n");
        assert_eq!(run_inc(env, "a", 3), "a\n");
        assert_eq!(run_inc(env, "a", 4), "a\n");

        assert_eq!(run_inc(env, "v", 1), "v\n");
        assert_eq!(run_inc(env, "v", 2), "v\n");
        assert_eq!(run_inc(env, "v", 3), "v\n");
        assert_eq!(run_inc(env, "v", 4), "v\n");

        assert_eq!(run_inc(env, "0", 1), "1\n");
        assert_eq!(run_inc(env, "0", 2), "1\n");
        assert_eq!(run_inc(env, "0", 3), "1\n");
        assert_eq!(run_inc(env, "0", 4), "1\n");

        assert_eq!(run_inc(env, "v0", 1), "v1\n");
        assert_eq!(run_inc(env, "v0", 2), "v1\n");
        assert_eq!(run_inc(env, "v0", 3), "v1\n");
        assert_eq!(run_inc(env, "v0", 4), "v1\n");

        assert_eq!(run_inc(env, "v1", 1), "v2\n");
        assert_eq!(run_inc(env, "v1", 2), "v2\n");
        assert_eq!(run_inc(env, "v1", 3), "v2\n");
        assert_eq!(run_inc(env, "v1", 4), "v2\n");

        assert_eq!(run_inc(env, "1.0", 1), "1.1\n");
        assert_eq!(run_inc(env, "1.0", 2), "2.0\n");
        assert_eq!(run_inc(env, "1.0", 3), "2.0\n");
        assert_eq!(run_inc(env, "1.0", 4), "2.0\n");

        assert_eq!(run_inc(env, "1.00", 1), "1.1\n");
        assert_eq!(run_inc(env, "1.00", 2), "2.0\n");
        assert_eq!(run_inc(env, "1.00", 3), "2.0\n");
        assert_eq!(run_inc(env, "1.00", 4), "2.0\n");

        assert_eq!(run_inc(env, "v1.0", 1), "v1.1\n");
        assert_eq!(run_inc(env, "v1.0", 2), "v2.0\n");
        assert_eq!(run_inc(env, "v1.0", 3), "v2.0\n");
        assert_eq!(run_inc(env, "v1.0", 4), "v2.0\n");

        assert_eq!(run_inc(env, "v1.00", 1), "v1.1\n");
        assert_eq!(run_inc(env, "v1.00", 2), "v2.0\n");
        assert_eq!(run_inc(env, "v1.00", 3), "v2.0\n");
        assert_eq!(run_inc(env, "v1.00", 4), "v2.0\n");

        assert_eq!(run_inc(env, "1.2.3", 1), "1.2.4\n");
        assert_eq!(run_inc(env, "1.2.3", 2), "1.3.0\n");
        assert_eq!(run_inc(env, "1.2.3", 3), "2.0.0\n");
        assert_eq!(run_inc(env, "1.2.3", 4), "2.0.0\n");
        assert_eq!(run_inc(env, "v1.2.3", 1), "v1.2.4\n");
        assert_eq!(run_inc(env, "v1.2.3", 2), "v1.3.0\n");
        assert_eq!(run_inc(env, "v1.2.3", 3), "v2.0.0\n");
        assert_eq!(run_inc(env, "v1.2.3", 4), "v2.0.0\n");

        assert_eq!(run_inc(env, "1.2.3+4", 1), "1.2.3+5\n");
        assert_eq!(run_inc(env, "1.2.3+4", 2), "1.2.4+0\n");
        assert_eq!(run_inc(env, "1.2.3+4", 3), "1.3.0+0\n");
        assert_eq!(run_inc(env, "1.2.3+4", 4), "2.0.0+0\n");
        assert_eq!(run_inc(env, "v1.2.3+4", 1), "v1.2.3+5\n");
        assert_eq!(run_inc(env, "v1.2.3+4", 2), "v1.2.4+0\n");
        assert_eq!(run_inc(env, "v1.2.3+4", 3), "v1.3.0+0\n");
        assert_eq!(run_inc(env, "v1.2.3+4", 4), "v2.0.0+0\n");
    }
}
