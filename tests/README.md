# rhai-env

Rhai Env is a plugin extension for manipulating env variables inside Rhai lang

## Usage

### `Cargo.toml`

```toml
[dependencies]
rhai-env = "0.1"
```

### [Rhai] script

```js
let variable = get_env("ENV_VARIABLE")

set_env("ENV_VARIABLE", variable)
```