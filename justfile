publisher := "slipwayhq"
name := "svg"

build configuration="debug":
  rm -rf artifacts
  mkdir -p artifacts/{{publisher}}.{{name}}

  slipway wit > wit/slipway.wit

  cd src && cargo build --target wasm32-wasip2 {{ if configuration == "release" { "--release" } else { "" } }}

  cp target/wasm32-wasip2/{{configuration}}/slipway_{{name}}.wasm artifacts/{{publisher}}.{{name}}/slipway_component.wasm
  cp slipway_component.json artifacts/{{publisher}}.{{name}}

  slipway package artifacts/{{publisher}}.{{name}}

test:
  cargo test
