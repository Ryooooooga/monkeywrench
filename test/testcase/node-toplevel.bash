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
