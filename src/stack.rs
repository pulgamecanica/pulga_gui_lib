use crate::geometry::Rect;
use crate::widget::Widget;

/// Lays widgets out top-to-bottom inside `bounds`, each given `height`
/// (full width, fixed row height, with `gap` between rows).
pub fn layout_vertical(bounds: Rect, widgets: &mut [&mut dyn Widget], height: f32, gap: f32) {
    let mut y = bounds.y;
    for widget in widgets {
        widget.set_bounds(Rect::new(bounds.x, y, bounds.w, height));
        y += height + gap;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::label::Label;
    #[cfg(target_arch = "wasm32")]
    use wasm_bindgen_test::wasm_bindgen_test;

    #[cfg_attr(not(target_arch = "wasm32"), test)]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
    fn stacks_widgets_with_gap() {
        let mut a = Label::new("a");
        let mut b = Label::new("b");
        let mut widgets: Vec<&mut dyn Widget> = vec![&mut a, &mut b];
        layout_vertical(Rect::new(0.0, 0.0, 200.0, 100.0), &mut widgets, 20.0, 5.0);

        assert_eq!(a.bounds(), Rect::new(0.0, 0.0, 200.0, 20.0));
        assert_eq!(b.bounds(), Rect::new(0.0, 25.0, 200.0, 20.0));
    }
}
