# PulgaGuiLib

GUI widget library for the `42run` project: `Label`, `Button`, `Selector`
today, more to come.

Widget logic (layout, hover/press state, selection) lives in plain Rust with
no GL or windowing dependency — that's what makes it testable on both native
and `wasm32-unknown-unknown`. A widget describes itself as a list of
`DrawCommand`s (`src/widget.rs`); a backend turns those into pixels. The only
backend today is the native GL gallery example below; `PulgaWinLib` itself is
a native-only dependency (`[target.'cfg(not(target_arch = "wasm32"))']` in
`Cargo.toml`) since its `winit`/`glutin` stack doesn't target wasm32.

## Usage

```sh
make build      # cargo build
make test       # cargo test (native)
make coverage   # cargo tarpaulin --lib
make gallery    # open a window showing one of each widget (needs a display)
make doc        # cargo doc --open
make            # make help
```

## WASM tests

Widget-logic tests also run for real inside headless Firefox via
`wasm32-unknown-unknown`, in a container (no wasm toolchain/wasm-pack/driver
installed on the host — this machine has no rustup and a tiny home quota):

```sh
make -C ../42Run wasm-image   # once, builds the shared test image
make wasm-test                 # runs the widget tests in headless Firefox
```

`should_panic` tests (not supported by `wasm_bindgen_test`) stay native-only;
everything else runs on both targets via a
`#[cfg_attr(not(target_arch = "wasm32"), test)] #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]`
pair per test function.

Part of the [42Run](../42Run) workspace of independent projects.
