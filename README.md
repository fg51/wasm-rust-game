# wasm-rust-game

## 1. walk-the-dog

```sh
$ mkdir walk-the-dog && cd walk-the-dog
$ npm init rust-webpack
$ pnpm install
$ pnpm run start
```

for Error: error:0308010C:ditigal envelope routines::unsuported

```sh
$ NODE_OPTIONS=--openssl-legacy-provider pnpm run start
```


### dyn_into

dyn_into is defined via trait wasm_bindgen::JsCast.
dyn_into cast from the static type of RUST to the dynamic type of JAVASCRIPT

### get_context
get-context() -> Result<Option<Object>>