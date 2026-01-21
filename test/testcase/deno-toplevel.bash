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
  [[ "$(cd src && mw deno toplevel)" = "$PWD/src" ]]
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
  [[ "$(cd src && mw deno toplevel)" = "$PWD" ]]
}

run-test deno-toplevel-no-deno-json
run-test deno-toplevel-with-deno-json
