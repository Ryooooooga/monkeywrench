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
