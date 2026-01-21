#!/usr/bin/env bash
set -eu

root_dir="$(realpath -- "$(dirname -- "$0")/..")"
test_dir="$root_dir/test"
tmp_dir="$test_dir/tmp"

mw() {
  "$root_dir/target/debug/monkeywrench" "$@"
}

run-test() {
  local test_name="$1"
  local test_dir="$tmp_dir/$test_name"

  echo "Running test: $test_name"
  mkdir -p "$test_dir"

  (cd "$test_dir" && "$1")
}

# monkeywrench deno toplevel

deno-toplevel-no-deno-json() {
  set -x
  # ./
  # +-- index.js
  # +-- src/
  #     +-- index.js
  mkdir -p src
  touch \
    index.js \
    src/index.js

  [[ "$(mw deno toplevel)" = "$PWD" ]]
  [[ "$(mw deno toplevel --root)" = "$PWD" ]]
  [[ "$(cd src && mw deno toplevel)" = "$PWD/src" ]]
  [[ "$(cd src && mw deno toplevel --root)" = "$PWD/src" ]]
}

deno-toplevel-with-deno-json() {
  set -x
  # ./
  # +-- deno.json
  # +-- index.js
  # +-- src/
  #     +-- index.js
  mkdir -p src
  touch \
    deno.json \
    index.js \
    src/index.js

  [[ "$(mw deno toplevel)" = "$PWD" ]]
  [[ "$(mw deno toplevel --root)" = "$PWD" ]]
  [[ "$(cd src && mw deno toplevel)" = "$PWD" ]]
  [[ "$(cd src && mw deno toplevel --root)" = "$PWD" ]]
}

run-test deno-toplevel-no-deno-json
run-test deno-toplevel-with-deno-json

# monkeywrench deno tasks

deno-tasks-none() {
  set -x
  # ./
  # +-- src/
  mkdir -p src

  [[ "$(mw deno tasks)" = "" ]]
  !(mw deno tasks --has start)
  !(mw deno tasks --has test)

  [[ "$(cd src && mw deno tasks)" = "" ]]
  !(cd src && mw deno tasks --has start)
  !(cd src && mw deno tasks --has test)
}

deno-tasks-no-tasks() {
  set -x
  # ./
  # +-- deno.json
  # +-- src/
  mkdir -p src
  echo '{}' > "deno.json"

  [[ "$(mw deno tasks)" = "" ]]
  !(mw deno tasks --has start)
  !(mw deno tasks --has test)

  [[ "$(cd src && mw deno tasks)" = "" ]]
  !(cd src && mw deno tasks --has start)
  !(cd src && mw deno tasks --has test)
}

deno-tasks-empty-tasks() {
  set -x
  # ./
  # +-- deno.json
  # +-- src/
  mkdir -p src
  echo '{"tasks":{}}' > "deno.json"

  [[ "$(mw deno tasks)" = "" ]]
  !(mw deno tasks --has start)
  !(mw deno tasks --has test)

  [[ "$(cd src && mw deno tasks)" = "" ]]
  !(cd src && mw deno tasks --has start)
  !(cd src && mw deno tasks --has test)
}

deno-tasks-invalid-tasks() {
  set -x
  # ./
  # +-- deno.json
  # +-- src/
  mkdir -p src
  echo '{"tasks":{"start":0,"test":"deno test"}}' > "deno.json"

  [[ "$(mw deno tasks)" = $'test\tdeno test' ]]
  !(mw deno tasks --has start)
  (mw deno tasks --has test)

  [[ "$(cd src && mw deno tasks)" = $'test\tdeno test' ]]
  !(cd src && mw deno tasks --has start)
  (cd src && mw deno tasks --has test)
}

deno-tasks-single-tasks() {
  set -x
  # ./
  # +-- deno.json
  # +-- src/
  mkdir -p src
  echo '{"tasks":{"start":"echo hello"}}' > deno.json

  [[ "$(mw deno tasks)" = $'start\techo hello' ]]
  (mw deno tasks --has start)
  !(mw deno tasks --has test)

  [[ "$(cd src && mw deno tasks)" = $'start\techo hello' ]]
  (cd src && mw deno tasks --has start)
  !(cd src && mw deno tasks --has test)
}

deno-tasks-multi-tasks() {
  set -x
  # ./
  # +-- deno.json
  # +-- src/
  mkdir -p src
  echo '{"tasks":{"start":"echo hello","lint":"deno lint"}}' > deno.json

  [[ "$(mw deno tasks)" = $'lint\tdeno lint\nstart\techo hello' ]]
  (mw deno tasks --has start)
  (mw deno tasks --has lint)
  !(mw deno tasks --has test)

  [[ "$(cd src && mw deno tasks)" = $'lint\tdeno lint\nstart\techo hello' ]]
  (cd src && mw deno tasks --has start)
  (cd src && mw deno tasks --has lint)
  !(cd src && mw deno tasks --has test)
}

run-test deno-tasks-none
run-test deno-tasks-no-tasks
run-test deno-tasks-empty-tasks
run-test deno-tasks-invalid-tasks
run-test deno-tasks-single-tasks
run-test deno-tasks-multi-tasks

# monkeywrench node toplevel

node-toplevel-with-lock() {
  set -x
  # ./
  # +-- package.json
  # +-- package-lock.json
  # +-- index.js
  # +-- src/
  #     +-- index.js
  mkdir -p src
  touch \
    package.json \
    package-lock.json \
    index.js \
    src/index.js

  [[ "$(mw node toplevel)" = "$PWD" ]]
  [[ "$(mw node toplevel --root)" = "$PWD" ]]
  [[ "$(cd src && mw node toplevel)" = "$PWD" ]]
  [[ "$(cd src && mw node toplevel --root)" = "$PWD" ]]
}

node-toplevel-no-lock() {
  set -x
  # ./
  # +-- package.json
  # +-- index.js
  # +-- src/
  #     +-- index.js
  mkdir -p src
  touch \
    package.json \
    index.js \
    src/index.js

  [[ "$(mw node toplevel)" = "$PWD" ]]
  [[ "$(mw node toplevel --root)" = "$PWD" ]]
  [[ "$(cd src && mw node toplevel)" = "$PWD" ]]
  [[ "$(cd src && mw node toplevel --root)" = "$PWD" ]]
}

node-toplevel-workspace-no-lock() {
  set -x
  # ./
  # +-- package.json
  # +-- index.js
  # +-- src/
  #     +-- index.js
  # +-- workspace/
  #     +-- package.json
  #     +-- index.js
  #     +-- src/
  #         +-- index.js
  mkdir -p src workspace/src
  touch \
    package.json \
    index.js \
    src/index.js \
    workspace/package.json \
    workspace/index.js \
    workspace/src/index.js

  [[ "$(mw node toplevel)" = "$PWD" ]]
  [[ "$(mw node toplevel --root)" = "$PWD" ]]
  [[ "$(cd src && mw node toplevel)" = "$PWD" ]]
  [[ "$(cd src && mw node toplevel --root)" = "$PWD" ]]
  [[ "$(cd workspace && mw node toplevel)" = "$PWD/workspace" ]]
  [[ "$(cd workspace && mw node toplevel --root)" = "$PWD/workspace" ]]
  [[ "$(cd workspace/src && mw node toplevel)" = "$PWD/workspace" ]]
  [[ "$(cd workspace/src && mw node toplevel --root)" = "$PWD/workspace" ]]
}

node-toplevel-workspace-with-lock() {
  set -x
  # ./
  # +-- package.json
  # +-- package-lock.json
  # +-- index.js
  # +-- src/
  #     +-- index.js
  # +-- workspace/
  #     +-- package.json
  #     +-- index.js
  #     +-- src/
  #         +-- index.js
  mkdir -p src workspace/src
  touch \
    package.json \
    package-lock.json \
    index.js \
    src/index.js \
    workspace/package.json \
    workspace/index.js \
    workspace/src/index.js

  [[ "$(mw node toplevel)" = "$PWD" ]]
  [[ "$(mw node toplevel --root)" = "$PWD" ]]
  [[ "$(cd src && mw node toplevel)" = "$PWD" ]]
  [[ "$(cd src && mw node toplevel --root)" = "$PWD" ]]
  [[ "$(cd workspace && mw node toplevel)" = "$PWD/workspace" ]]
  [[ "$(cd workspace && mw node toplevel --root)" = "$PWD" ]]
  [[ "$(cd workspace/src && mw node toplevel)" = "$PWD/workspace" ]]
  [[ "$(cd workspace/src && mw node toplevel --root)" = "$PWD" ]]
}

run-test node-toplevel-no-lock
run-test node-toplevel-with-lock
run-test node-toplevel-workspace-no-lock
run-test node-toplevel-workspace-with-lock

# monkeywrench node package-manager

node-package-manager-default() {
  set -x
  # ./
  # +-- src/
  mkdir -p src

  [[ "$(mw node package-manager)" = "npm" ]]
  [[ "$(cd src && mw node package-manager)" = "npm" ]]
}

node-package-manager-npm() {
  set -x
  # ./
  # +-- package.json
  # +-- package-lock.json
  # +-- src/
  mkdir -p src
  touch \
    package.json \
    package-lock.json

  [[ "$(mw node package-manager)" = "npm" ]]
  [[ "$(cd src && mw node package-manager)" = "npm" ]]
}

node-package-manager-yarn() {
  set -x
  # ./
  # +-- package.json
  # +-- yarn.lock
  # +-- src/
  mkdir -p src
  touch \
    package.json \
    yarn.lock

  [[ "$(mw node package-manager)" = "yarn" ]]
  [[ "$(cd src && mw node package-manager)" = "yarn" ]]
}

node-package-manager-pnpm() {
  set -x
  # ./
  # +-- package.json
  # +-- pnpm-lock.yaml
  # +-- src/
  mkdir -p src
  touch \
    package.json \
    pnpm-lock.yaml

  [[ "$(mw node package-manager)" = "pnpm" ]]
  [[ "$(cd src && mw node package-manager)" = "pnpm" ]]
}

run-test node-package-manager-default
run-test node-package-manager-npm
run-test node-package-manager-yarn
run-test node-package-manager-pnpm

# monkeywrench node scripts

node-scripts-none() {
  set -x
  # ./
  # +-- src/
  mkdir -p src

  [[ "$(mw node scripts)" = "" ]]
  !(mw node scripts --has start)
  !(mw node scripts --has test)

  [[ "$(cd src && mw node scripts)" = "" ]]
  !(cd src && mw node scripts --has start)
  !(cd src && mw node scripts --has test)
}

node-scripts-no-scripts() {
  set -x
  # ./
  # +-- package.json
  # +-- src/
  mkdir -p src
  echo '{}' > "package.json"

  [[ "$(mw node scripts)" = "" ]]
  !(mw node scripts --has start)
  !(mw node scripts --has test)

  [[ "$(cd src && mw node scripts)" = "" ]]
  !(cd src && mw node scripts --has start)
  !(cd src && mw node scripts --has test)
}

node-scripts-empty-scripts() {
  set -x
  # ./
  # +-- package.json
  # +-- src/
  mkdir -p src
  echo '{"scripts":{}}' > "package.json"

  [[ "$(mw node scripts)" = "" ]]
  !(mw node scripts --has start)
  !(mw node scripts --has test)

  [[ "$(cd src && mw node scripts)" = "" ]]
  !(cd src && mw node scripts --has start)
  !(cd src && mw node scripts --has test)
}

node-scripts-invalid-scripts() {
  set -x
  # ./
  # +-- package.json
  # +-- src/
  mkdir -p src
  echo '{"scripts":{"start":0,"test":"jest"}}' > "package.json"

  [[ "$(mw node scripts)" = $'test\tjest' ]]
  !(mw node scripts --has start)
  (mw node scripts --has test)

  [[ "$(cd src && mw node scripts)" = $'test\tjest' ]]
  !(cd src && mw node scripts --has start)
  (cd src && mw node scripts --has test)
}

node-scripts-single-scripts() {
  set -x
  # ./
  # +-- package.json
  # +-- src/
  mkdir -p src
  echo '{"scripts":{"start":"echo hello"}}' > package.json

  [[ "$(mw node scripts)" = $'start\techo hello' ]]
  (mw node scripts --has start)
  !(mw node scripts --has test)

  [[ "$(cd src && mw node scripts)" = $'start\techo hello' ]]
  (cd src && mw node scripts --has start)
  !(cd src && mw node scripts --has test)
}

node-scripts-multi-scripts() {
  set -x
  # ./
  # +-- package.json
  # +-- src/
  mkdir -p src
  echo '{"scripts":{"start":"echo hello","lint":"eslint"}}' > package.json

  [[ "$(mw node scripts)" = $'lint\teslint\nstart\techo hello' ]]
  (mw node scripts --has start)
  (mw node scripts --has lint)
  !(mw node scripts --has test)

  [[ "$(cd src && mw node scripts)" = $'lint\teslint\nstart\techo hello' ]]
  (cd src && mw node scripts --has start)
  (cd src && mw node scripts --has lint)
  !(cd src && mw node scripts --has test)
}

node-scripts-workspace() {
  set -x
  # ./
  # +-- package.json
  # +-- workspace/
  #     +-- package.json
  #     +-- src/
  mkdir -p src workspace/src
  echo '{"scripts":{"start":"echo hello","lint":"eslint"}}' > package.json
  echo '{"scripts":{"start":"echo guten tag","test":"jest"}}' > workspace/package.json

  [[ "$(mw node scripts)" = $'lint\teslint\nstart\techo hello' ]]
  (mw node scripts --has start)
  (mw node scripts --has lint)
  !(mw node scripts --has test)

  [[ "$(cd src && mw node scripts)" = $'lint\teslint\nstart\techo hello' ]]
  (cd src && mw node scripts --has start)
  (cd src && mw node scripts --has lint)
  !(cd src && mw node scripts --has test)

  [[ "$(cd workspace && mw node scripts)" = $'start\techo guten tag\ntest\tjest' ]]
  (cd workspace && mw node scripts --has start)
  !(cd workspace && mw node scripts --has lint)
  (cd workspace && mw node scripts --has test)

  [[ "$(cd workspace/src && mw node scripts)" = $'start\techo guten tag\ntest\tjest' ]]
  (cd workspace/src && mw node scripts --has start)
  !(cd workspace/src && mw node scripts --has lint)
  (cd workspace/src && mw node scripts --has test)
}

run-test node-scripts-none
run-test node-scripts-no-scripts
run-test node-scripts-empty-scripts
run-test node-scripts-invalid-scripts
run-test node-scripts-single-scripts
run-test node-scripts-multi-scripts
run-test node-scripts-workspace
