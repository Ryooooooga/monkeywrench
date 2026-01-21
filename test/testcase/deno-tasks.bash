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
