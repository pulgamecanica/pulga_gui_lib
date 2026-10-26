//! Drives a [`Ui`] with synthetic input, no GPU involved.

use super::*;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen_test::wasm_bindgen_test;

const SCREEN: Rect = Rect::new(0.0, 0.0, 800.0, 600.0);

fn input_at(x: f32, y: f32) -> Input {
    Input {
        mouse: (x, y),
        dt: 1.0 / 60.0,
        ..Input::default()
    }
}

fn press(mut i: Input) -> Input {
    i.down[0] = true;
    i.pressed[0] = true;
    i
}

fn hold(mut i: Input) -> Input {
    i.down[0] = true;
    i
}

fn release(mut i: Input) -> Input {
    i.released[0] = true;
    i
}

fn frame<R>(ui: &mut Ui, input: Input, f: impl FnOnce(&mut Ui) -> R) -> R {
    ui.begin_frame(input, SCREEN);
    let r = f(ui);
    ui.end_frame();
    r
}

/// Clicks at (x, y): one frame pressing, one releasing. Returns the second
/// frame's result.
fn click<R>(ui: &mut Ui, x: f32, y: f32, mut f: impl FnMut(&mut Ui) -> R) -> R {
    frame(ui, press(input_at(x, y)), &mut f);
    frame(ui, release(input_at(x, y)), f)
}

fn typing(text: &str, keys: &[UiKey]) -> Input {
    let mut i = input_at(400.0, 500.0);
    i.text = text.into();
    i.keys = keys.to_vec();
    i
}

#[cfg_attr(not(target_arch = "wasm32"), test)]
#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
fn button_clicks_on_press_and_release_inside() {
    let mut ui = Ui::default();
    let pressed = frame(&mut ui, press(input_at(10.0, 10.0)), |ui| ui.button("Go"));
    assert!(pressed.pressed && !pressed.clicked);
    let released = frame(&mut ui, release(input_at(10.0, 10.0)), |ui| ui.button("Go"));
    assert!(released.clicked);
}

#[cfg_attr(not(target_arch = "wasm32"), test)]
#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
fn dragging_off_before_release_is_not_a_click() {
    let mut ui = Ui::default();
    frame(&mut ui, press(input_at(10.0, 10.0)), |ui| ui.button("Go"));
    let held = frame(&mut ui, hold(input_at(10.0, 300.0)), |ui| ui.button("Go"));
    assert!(held.dragging, "the button keeps the drag off its rect");
    let r = frame(&mut ui, release(input_at(10.0, 300.0)), |ui| {
        ui.button("Go")
    });
    assert!(!r.clicked);
}

#[cfg_attr(not(target_arch = "wasm32"), test)]
#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
fn toggle_flips_its_value() {
    let mut ui = Ui::default();
    let mut on = false;
    let r = click(&mut ui, 10.0, 10.0, |ui| ui.toggle(&mut on, "Sound"));
    assert!(on && r.changed);
}

#[cfg_attr(not(target_arch = "wasm32"), test)]
#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
fn text_input_types_deletes_submits_and_reverts() {
    let mut ui = Ui::default();
    let mut name = String::from("ab");
    click(&mut ui, 10.0, 10.0, |ui| ui.text_input("n", &mut name, ""));
    assert!(ui.wants_keyboard());
    let r = frame(&mut ui, typing("cd", &[UiKey::Backspace]), |ui| {
        ui.text_input("n", &mut name, "")
    });
    assert!(r.changed);
    assert_eq!(name, "abc");
    let r = frame(&mut ui, typing("", &[UiKey::Enter]), |ui| {
        ui.text_input("n", &mut name, "")
    });
    assert!(r.submitted && !ui.wants_keyboard());

    click(&mut ui, 10.0, 10.0, |ui| ui.text_input("n", &mut name, ""));
    frame(&mut ui, typing("zz", &[]), |ui| {
        ui.text_input("n", &mut name, "")
    });
    frame(&mut ui, typing("", &[UiKey::Escape]), |ui| {
        ui.text_input("n", &mut name, "")
    });
    assert_eq!(name, "abc", "escape restores the text from before editing");
}

#[cfg_attr(not(target_arch = "wasm32"), test)]
#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
fn drag_value_drags_in_steps() {
    let mut ui = Ui::default();
    let mut v = 0.0f32;
    frame(&mut ui, press(input_at(100.0, 10.0)), |ui| {
        ui.drag_value("x", None, &mut v, 0.25, 2)
    });
    // 40 px right at 4 px per step: 10 steps of 0.25.
    let r = frame(&mut ui, hold(input_at(140.0, 10.0)), |ui| {
        ui.drag_value("x", None, &mut v, 0.25, 2)
    });
    assert!(r.changed);
    assert!((v - 2.5).abs() < 1e-5, "{v}");
    frame(&mut ui, release(input_at(140.0, 10.0)), |ui| {
        ui.drag_value("x", None, &mut v, 0.25, 2)
    });
    assert!(!ui.wants_keyboard(), "a drag doesn't open the text editor");
}

#[cfg_attr(not(target_arch = "wasm32"), test)]
#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
fn drag_value_click_edits_as_text() {
    let mut ui = Ui::default();
    let mut v = 1.0f32;
    click(&mut ui, 100.0, 10.0, |ui| {
        ui.drag_value("x", None, &mut v, 0.25, 2)
    });
    assert!(ui.wants_keyboard());
    let erase = [
        UiKey::End,
        UiKey::Backspace,
        UiKey::Backspace,
        UiKey::Backspace,
        UiKey::Backspace,
    ];
    frame(&mut ui, typing("", &erase), |ui| {
        ui.drag_value("x", None, &mut v, 0.25, 2)
    });
    frame(&mut ui, typing("-3.5", &[]), |ui| {
        ui.drag_value("x", None, &mut v, 0.25, 2)
    });
    let r = frame(&mut ui, typing("", &[UiKey::Enter]), |ui| {
        ui.drag_value("x", None, &mut v, 0.25, 2)
    });
    assert!(r.changed && r.submitted);
    assert_eq!(v, -3.5);
}

#[cfg_attr(not(target_arch = "wasm32"), test)]
#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
fn dropdown_opens_and_picks() {
    let mut ui = Ui::default();
    let mut sel = 0;
    let options = ["Straight", "Left", "Right"];
    click(&mut ui, 50.0, 10.0, |ui| {
        ui.dropdown("kind", &mut sel, &options)
    });
    // Popup rows start under the field (row 30 + gap 4 + pad 4); second row:
    let row_y = 30.0 + 4.0 + 4.0 + 30.0 + 2.0 + 10.0;
    let r = click(&mut ui, 50.0, row_y, |ui| {
        ui.dropdown("kind", &mut sel, &options)
    });
    assert!(r.changed);
    assert_eq!(sel, 1);
}

#[cfg_attr(not(target_arch = "wasm32"), test)]
#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
fn open_popup_blocks_widgets_under_it() {
    let mut ui = Ui::default();
    let mut sel = 0;
    let options = ["A", "B", "C"];
    let below = |ui: &mut Ui, sel: &mut usize| {
        ui.dropdown("d", sel, &options);
        ui.button("Under")
    };
    click(&mut ui, 50.0, 10.0, |ui| below(ui, &mut sel));
    // The popup now covers the button that sits right under the field.
    frame(&mut ui, input_at(50.0, 50.0), |ui| below(ui, &mut sel));
    let r = frame(&mut ui, input_at(50.0, 50.0), |ui| below(ui, &mut sel));
    assert!(!r.hovered);
}

#[cfg_attr(not(target_arch = "wasm32"), test)]
#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
fn modal_blocks_the_base_layer() {
    let mut ui = Ui::default();
    let draw = |ui: &mut Ui| {
        let r = ui.button("Behind");
        ui.modal("Sure?", 300.0, |ui| ui.label("Really"));
        r
    };
    frame(&mut ui, input_at(10.0, 10.0), draw);
    let r = frame(&mut ui, input_at(10.0, 10.0), draw);
    assert!(!r.hovered);
    assert!(ui.wants_pointer());
}

#[cfg_attr(not(target_arch = "wasm32"), test)]
#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
fn scroll_area_scrolls_with_the_wheel_and_clamps() {
    let mut ui = Ui::default();
    let content = |ui: &mut Ui| {
        ui.scroll_area("list", 100.0, |ui| {
            (0..20)
                .map(|i| ui.button(&format!("Row {i}")).rect)
                .collect::<Vec<_>>()
        })
    };
    let before = frame(&mut ui, input_at(50.0, 50.0), content);
    let mut wheel = input_at(50.0, 50.0);
    wheel.wheel = -1.0; // toward the user: scroll down
    frame(&mut ui, wheel, content);
    let after = frame(&mut ui, input_at(50.0, 50.0), content);
    assert!(after[0].y < before[0].y);
    for _ in 0..100 {
        let mut w = input_at(50.0, 50.0);
        w.wheel = -5.0;
        frame(&mut ui, w, content);
    }
    let end = frame(&mut ui, input_at(50.0, 50.0), content);
    // Clamped: the last row's bottom sits at the area's bottom.
    assert!(
        (end[19].bottom() - 100.0).abs() < 1.0,
        "{}",
        end[19].bottom()
    );
}

#[cfg_attr(not(target_arch = "wasm32"), test)]
#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
fn columns_split_the_width_equally() {
    let mut ui = Ui::default();
    let rects = frame(&mut ui, input_at(0.0, 0.0), |ui| {
        ui.put(Rect::new(0.0, 0.0, 312.0, 100.0), |ui| {
            ui.columns(3, |ui| {
                [
                    ui.button("a").rect,
                    ui.button("b").rect,
                    ui.button("c").rect,
                ]
            })
        })
    });
    // (312 - 2 gaps of 6) / 3 = 100 each.
    assert_eq!(
        rects.map(|r| (r.x, r.w)),
        [(0.0, 100.0), (106.0, 100.0), (212.0, 100.0)]
    );
}

#[cfg_attr(not(target_arch = "wasm32"), test)]
#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
fn push_id_separates_repeated_labels() {
    let mut ui = Ui::default();
    let ids = frame(&mut ui, input_at(0.0, 0.0), |ui| {
        [0, 1].map(|row| ui.push_id(row, |ui| ui.button("Delete").id))
    });
    assert_ne!(ids[0], ids[1]);
}

#[cfg_attr(not(target_arch = "wasm32"), test)]
#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
fn panels_claim_the_pointer() {
    let mut ui = Ui::default();
    frame(&mut ui, input_at(50.0, 50.0), |ui| {
        ui.panel("p", Rect::new(0.0, 0.0, 200.0, 600.0), |_| ())
    });
    assert!(ui.wants_pointer());
    frame(&mut ui, input_at(500.0, 50.0), |ui| {
        ui.panel("p", Rect::new(0.0, 0.0, 200.0, 600.0), |_| ())
    });
    assert!(!ui.wants_pointer());
}

#[cfg_attr(not(target_arch = "wasm32"), test)]
#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
fn list_row_chevron_toggles_without_selecting() {
    let mut ui = Ui::default();
    let mut open = false;
    let r = click(&mut ui, 5.0, 10.0, |ui| {
        ui.list_row("g", 0, Some(&mut open), None, "Group", false, None)
    });
    assert!(open && !r.clicked);
    let r = click(&mut ui, 150.0, 10.0, |ui| {
        ui.list_row("g", 0, Some(&mut open), None, "Group", false, None)
    });
    assert!(open && r.clicked);
}
