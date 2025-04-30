publisher := "slipwayhq"
name := "svg"

build configuration="debug": clear-components wit && (cargo-build configuration) (assemble configuration) package
build-ci: clear-components && (cargo-build "release") (assemble "release") package-ci
  rustup target add wasm32-wasip2

clear-components:
  rm -rf components
  mkdir -p components/{{publisher}}.{{name}}

cargo-build configuration="debug":
  cd src && cargo build --target wasm32-wasip2 {{ if configuration == "release" { "--release" } else { "" } }}

assemble configuration="debug":
  cp target/wasm32-wasip2/{{configuration}}/slipway_{{name}}.wasm components/{{publisher}}.{{name}}/run.wasm
  cp src/slipway_component.json components/{{publisher}}.{{name}}

wit:
  slipway wit > wit/slipway.wit

package:
  slipway package components/{{publisher}}.{{name}}

package-ci:
  docker run --rm -v "$(pwd)/components":/workspace -w /workspace slipwayhq/slipway:latest slipway package {{publisher}}.{{name}}

test: build
  cargo test

release version:
  git tag -a "{{version}}" -m "Release {{version}}"
  git push origin "{{version}}"
