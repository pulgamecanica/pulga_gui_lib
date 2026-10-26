# pulga_gui_lib

GUI toolkit for the `42run` project: an **immediate-mode UI** with the
widgets a game editor needs, a customizable **theme**, a built-in bitmap
**font** and pixel **icons** — all defined in code (no font files, font
libraries or image decoders).

```rust
ui.begin_frame(input, screen);
ui.panel("inspector", rect, |ui| {
    ui.heading("Transform");
    ui.columns(3, |ui| {
        ui.drag_value("x", Some(("X", ui.theme.axis[0])), &mut pos[0], 0.25, 2);
        // ...
    });
    if ui.button_with(ButtonKind::Danger, Some(Icon::Trash), "Delete").clicked {
        // ...
    }
});
let commands = ui.end_frame(); // -> GuiRenderer
```

## Design

- **Immediate mode** (`src/ui/`): the app describes its UI every frame;
  each widget call draws and returns a `Response` (hovered, clicked,
  dragged, changed, submitted). Hot/active/focus ids, layout frames
  (`horizontal`, `columns`, `indent`, `put`), clipping, and layers (base,
  modal, popup, tooltip) so popups draw on top and block input under them.
- **Widgets**: label, caption, heading, separator, badge, buttons (normal,
  primary, danger, ghost, icon), toggle, checkbox, segmented control, color
  swatches, text input, drag-to-edit number field (click to type), list/tree
  rows with disclosure, panel, bar, scroll area, dropdown, menu, modal
  dialog, tooltips, toast notifications.
- **Theme** (`src/theme.rs`): every color and metric in one struct; `dark()`
  and `light()` built in, any field overridable at runtime.
- **Renderer-independent output** (`src/draw.rs`): widgets emit
  `DrawCommand`s (rects, rounded rects, shadows, text, icons, clips). The
  native backend is `src/render.rs` (`GuiRenderer`, on `pulga_win_lib`'s
  `Canvas2d`, which renders rounded corners and shadows with an SDF).
- **Input** (`src/input.rs`): a plain per-frame snapshot the host fills from
  its own window events.
- **Font** (`src/font.rs`): proportional 5×9 pixel font with lowercase and
  descenders; **icons** (`src/icons.rs`): 9×9 pixel art, same atlas.

Logic, layout, font and icons have no GL or windowing dependency
(`pulga_win_lib` is a native-only dependency in `Cargo.toml`), so the tests
drive a `Ui` with synthetic input on native and in a browser via wasm32.

## Usage

```sh
make build      # cargo build
make test       # cargo test (native)
make coverage   # cargo tarpaulin --lib
make gallery    # every widget on one screen (needs a display)
make doc        # cargo doc --open
make            # make help
```

`PULGA_SCREENSHOT=gallery.png make gallery` saves a screenshot instead of
staying open (see `pulga_win_lib::screenshot`).

## WASM tests

Widget-logic tests also run for real inside headless Firefox via
`wasm32-unknown-unknown`, in a container (no wasm toolchain/wasm-pack/driver
installed on the host — this machine has no rustup and a tiny home quota):

```sh
make -C .. wasm-image   # once, builds the shared test image
make wasm-test          # runs the widget tests in headless Firefox
```

Every test runs on both targets via a
`#[cfg_attr(not(target_arch = "wasm32"), test)] #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]`
pair per test function.

Part of the [42Run](..) workspace.
