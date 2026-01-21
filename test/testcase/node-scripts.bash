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
