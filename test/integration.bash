#!/usr/bin/env bash
set -eu

root_dir="$(realpath -- "$(dirname -- "$0")/..")"
test_dir="$root_dir/test"
tmp_dir="$(realpath -- "$(mktemp -d)")"
trap 'rm -rf "$tmp_dir"' EXIT
echo "Using temporary directory: $tmp_dir"

mw() {
  "$root_dir/target/debug/monkeywrench" "$@"
}

run-test() {
  local test_name="$1"
  local test_dir="$tmp_dir/$test_name"

  echo "Running test: $test_name"
  rm -rf "$test_dir"
  mkdir -p "$test_dir"

  (cd "$test_dir" && "$1")
}

. "$test_dir/testcase/command.bash"
. "$test_dir/testcase/deno-toplevel.bash"
. "$test_dir/testcase/deno-tasks.bash"
. "$test_dir/testcase/node-toplevel.bash"
. "$test_dir/testcase/node-package-manager.bash"
. "$test_dir/testcase/node-scripts.bash"
