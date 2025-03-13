publisher := "slipwayhq"
name := "svg"

build configuration="debug":
  rm -rf components
  mkdir -p components/{{publisher}}.{{name}}

  slipway wit > wit/slipway.wit

  cd src && cargo build --target wasm32-wasip2 {{ if configuration == "release" { "--release" } else { "" } }}

  cp target/wasm32-wasip2/{{configuration}}/slipway_{{name}}.wasm components/{{publisher}}.{{name}}/run.wasm
  cp slipway_component.json components/{{publisher}}.{{name}}

  slipway package components/{{publisher}}.{{name}}

test: build
  cargo test
