# Web application developed with Rust/WebAssembly + Yew + Tailwindcss.

## To run the app:

Add the WebAssembly target::

```
$ rustup target add wasm32-unknown-unknown
```

Install Trunk to run the app:

```
$ cargo install trunk
```

Add your Youtube api key in the env.rs file:

```
pub static API_KEY: &str = "TU_API_KEY";
```

Run the app:

```
$ trunk serve
```

Build the application by running:

```
$ trunk build --release
```
