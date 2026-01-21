command-npm() {
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

  [[ "$(mw command)" = "npm" ]]
  [[ "$(mw command --action build)" = "npm run build" ]]
  [[ "$(mw command --action run)" = "npm start" ]]
  [[ "$(mw command --action test)" = "npm test" ]]
  [[ "$(mw command --action lint)" = "npm run lint" ]]
  [[ "$(mw command --action format)" = "npm run fmt" ]]
  [[ "$(cd src && mw command)" = "npm" ]]
  [[ "$(cd src && mw command --action build)" = "npm run build" ]]
  [[ "$(cd src && mw command --action run)" = "npm start" ]]
  [[ "$(cd src && mw command --action test)" = "npm test" ]]
  [[ "$(cd src && mw command --action lint)" = "npm run lint" ]]
  [[ "$(cd src && mw command --action format)" = "npm run fmt" ]]
}

command-yarn() {
  set -x
  # ./
  # +-- package.json
  # +-- yarn.lock
  # +-- index.js
  # +-- src/
  #     +-- index.js
  mkdir -p src
  touch \
    package.json \
    yarn.lock \
    index.js \
    src/index.js

  [[ "$(mw command)" = "yarn" ]]
  [[ "$(mw command --action build)" = "yarn build" ]]
  [[ "$(mw command --action run)" = "yarn start" ]]
  [[ "$(mw command --action test)" = "yarn test" ]]
  [[ "$(mw command --action lint)" = "yarn lint" ]]
  [[ "$(mw command --action format)" = "yarn fmt" ]]
  [[ "$(cd src && mw command)" = "yarn" ]]
  [[ "$(cd src && mw command --action build)" = "yarn build" ]]
  [[ "$(cd src && mw command --action run)" = "yarn start" ]]
  [[ "$(cd src && mw command --action test)" = "yarn test" ]]
  [[ "$(cd src && mw command --action lint)" = "yarn lint" ]]
  [[ "$(cd src && mw command --action format)" = "yarn fmt" ]]
}

command-pnpm() {
  set -x
  # ./
  # +-- package.json
  # +-- pnpm-lock.yaml
  # +-- index.js
  # +-- src/
  #     +-- index.js
  mkdir -p src
  touch \
    package.json \
    pnpm-lock.yaml \
    index.js \
    src/index.js

  [[ "$(mw command)" = "pnpm" ]]
  [[ "$(mw command --action build)" = "pnpm build" ]]
  [[ "$(mw command --action run)" = "pnpm start" ]]
  [[ "$(mw command --action test)" = "pnpm test" ]]
  [[ "$(mw command --action lint)" = "pnpm lint" ]]
  [[ "$(mw command --action format)" = "pnpm fmt" ]]
  [[ "$(cd src && mw command)" = "pnpm" ]]
  [[ "$(cd src && mw command --action build)" = "pnpm build" ]]
  [[ "$(cd src && mw command --action run)" = "pnpm start" ]]
  [[ "$(cd src && mw command --action test)" = "pnpm test" ]]
  [[ "$(cd src && mw command --action lint)" = "pnpm lint" ]]
  [[ "$(cd src && mw command --action format)" = "pnpm fmt" ]]
}

command-deno() {
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

  [[ "$(mw command)" = "deno" ]]
  [[ "$(mw command --action build)" = "deno task build" ]]
  [[ "$(mw command --action run)" = "deno task start" ]]
  [[ "$(mw command --action test)" = "deno test" ]]
  [[ "$(mw command --action lint)" = "deno lint" ]]
  [[ "$(mw command --action format)" = "deno fmt" ]]
  [[ "$(cd src && mw command)" = "deno" ]]
  [[ "$(cd src && mw command --action build)" = "deno task build" ]]
  [[ "$(cd src && mw command --action run)" = "deno task start" ]]
  [[ "$(cd src && mw command --action test)" = "deno test" ]]
  [[ "$(cd src && mw command --action lint)" = "deno lint" ]]
  [[ "$(cd src && mw command --action format)" = "deno fmt" ]]
}

command-make() {

  set -x
  # ./
  # +-- Makefile
  # +-- src/
  #     +-- main.c
  mkdir -p src
  touch \
    Makefile \
    src/main.c

  [[ "$(mw command)" = "make" ]]
  [[ "$(mw command --action build)" = "make" ]]
  [[ "$(mw command --action run)" = "make run" ]]
  [[ "$(mw command --action test)" = "make test" ]]
  [[ "$(mw command --action lint)" = "make lint" ]]
  [[ "$(mw command --action format)" = "make fmt" ]]
  [[ "$(cd src && mw command)" = "make" ]]
  [[ "$(cd src && mw command --action build)" = "make" ]]
  [[ "$(cd src && mw command --action run)" = "make run" ]]
  [[ "$(cd src && mw command --action test)" = "make test" ]]
  [[ "$(cd src && mw command --action lint)" = "make lint" ]]
  [[ "$(cd src && mw command --action format)" = "make fmt" ]]
}

command-cargo() {
  set -x
  # ./
  # +-- Cargo.toml
  # +-- src/
  #     +-- main.rs
  mkdir -p src
  touch \
    Cargo.toml \
    src/main.rs

  [[ "$(mw command)" = "cargo" ]]
  [[ "$(mw command --action build)" = "cargo build" ]]
  [[ "$(mw command --action run)" = "cargo run" ]]
  [[ "$(mw command --action test)" = "cargo test" ]]
  [[ "$(mw command --action lint)" = "cargo clippy" ]]
  [[ "$(mw command --action format)" = "cargo fmt" ]]
  [[ "$(cd src && mw command)" = "cargo" ]]
  [[ "$(cd src && mw command --action build)" = "cargo build" ]]
  [[ "$(cd src && mw command --action run)" = "cargo run" ]]
  [[ "$(cd src && mw command --action test)" = "cargo test" ]]
  [[ "$(cd src && mw command --action lint)" = "cargo clippy" ]]
  [[ "$(cd src && mw command --action format)" = "cargo fmt" ]]
}

command-go() {
  set -x
  # ./
  # +-- go.mod
  # +-- main.go
  # +-- pkg/
  #     +-- pkg.go
  mkdir -p pkg
  touch \
    go.mod \
    main.go \
    pkg/pkg.go

  [[ "$(mw command)" = "go" ]]
  [[ "$(mw command --action build)" = "go build" ]]
  [[ "$(mw command --action run)" = "go run ." ]]
  [[ "$(mw command --action test)" = "go test ./..." ]]
  [[ "$(mw command --action lint)" = "golangci-lint run" ]]
  [[ "$(mw command --action format)" = "goimports -w ." ]]
  [[ "$(cd pkg && mw command)" = "go" ]]
  [[ "$(cd pkg && mw command --action build)" = "go build" ]]
  [[ "$(cd pkg && mw command --action run)" = "go run ." ]]
  [[ "$(cd pkg && mw command --action test)" = "go test ./..." ]]
  [[ "$(cd pkg && mw command --action lint)" = "golangci-lint run" ]]
  [[ "$(cd pkg && mw command --action format)" = "goimports -w ." ]]
}

command-unknown() {
  set -x
  # ./
  # +-- README.md
  # +-- src/
  #     +-- index.txt
  mkdir -p src
  touch \
    README.md \
    src/index.txt

  (! mw command)
  (cd src && ! mw command)
}

run-test command-npm
run-test command-yarn
run-test command-pnpm
run-test command-deno
run-test command-make
run-test command-cargo
run-test command-go
run-test command-unknown
