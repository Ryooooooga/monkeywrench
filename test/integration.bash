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

# monkeywrench node toplevel

node-toplevel-with-node_modules() {
  set -x
  # ./
  # +-- package.json
  # +-- index.js
  # +-- node_modules/
  # +-- src/
  #     +-- index.js
  mkdir -p node_modules src
  touch \
    package.json \
    index.js \
    src/index.js

  [[ "$(mw node toplevel)" = "$PWD" ]]
  [[ "$(mw node toplevel --root)" = "$PWD" ]]
  [[ "$(cd src && mw node toplevel)" = "$PWD" ]]
  [[ "$(cd src && mw node toplevel --root)" = "$PWD" ]]
}

node-toplevel-no-node_modules() {
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

node-toplevel-workspace-no-node_modules() {
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

node-toplevel-workspace-with-node_modules() {
  set -x
  # ./
  # +-- package.json
  # +-- index.js
  # +-- node_modules/
  # +-- src/
  #     +-- index.js
  # +-- workspace/
  #     +-- package.json
  #     +-- index.js
  #     +-- src/
  #         +-- index.js
  mkdir -p node_modules src workspace/src
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
  [[ "$(cd workspace && mw node toplevel --root)" = "$PWD" ]]
  [[ "$(cd workspace/src && mw node toplevel)" = "$PWD/workspace" ]]
  [[ "$(cd workspace/src && mw node toplevel --root)" = "$PWD" ]]
}

run-test node-toplevel-no-node_modules
run-test node-toplevel-with-node_modules
run-test node-toplevel-workspace-no-node_modules
run-test node-toplevel-workspace-with-node_modules

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
  echo '{"scripts":{"start":0}}' > "package.json"

  [[ "$(mw node scripts)" = "" ]]
  !(mw node scripts --has start)
  !(mw node scripts --has test)

  [[ "$(cd src && mw node scripts)" = "" ]]
  !(cd src && mw node scripts --has start)
  !(cd src && mw node scripts --has test)
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
