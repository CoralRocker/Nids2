// UTIL.RS
// Use for functions dealing with objects and displaying textures and menus.
// Not for memory allocation or system-level things.

use crate::game::*;
use raylib::consts::KeyboardKey::*;
use raylib::ffi::Rectangle as ffirect;
use raylib::prelude::*;
use std::collections::HashMap;
use std::ffi::CString;
use std::sync::Arc;

/// Get Vector of all unique categories that contain objects.
pub fn get_all_types(only_placeable: bool) -> Vec<String> {
    let mut result = Vec::new();

    for item in LOADED_TEXTURES
        .lock()
        .expect("Unable to lock LOADED_TEXTURES mutex")
        .iter()
    {
        let item = item.1; // get Value from (K, V) pair
        let item = &item.1; // get &ObjectConfig from Arc<(Texture2D, ObjectConfig>
        if only_placeable && !(item.category.eq("sys") || item.category.eq("player")) {
            result.push(item.category.clone());
        }else if !only_placeable {
            result.push(item.category.clone());
        }
    }

    result.sort_unstable();
    result.dedup(); // Remove consecutive duplicates from sorted types vector.

    result
}

/// Get Vector of all object types' info.
pub fn get_all_objects() -> Vec<Arc<(Texture2D, ObjectConfig)>> {
    let mut result = Vec::new();
    for item in LOADED_TEXTURES
        .lock()
        .expect("Unable to lock LOADED_TEXTURES mutex")
        .iter()
    {
        result.push(item.1.clone());
    }

    result
}

/// Get Hashmap with all object types sorted by their categories.
pub fn get_all_objects_sorted(only_placeable: bool) -> HashMap<String, Vec<Arc<(Texture2D, ObjectConfig)>>> {
    let mut result = HashMap::new();

    let types = get_all_types(only_placeable);

    let all_objs = get_all_objects();

    for t in types.iter() {
        let mut objs = Vec::new();
        for obj in all_objs.iter() {
            if obj.1.category.eq(t) {
                objs.push(obj.clone());
            }
        }
        result.insert(t.clone(), objs);
    }

    result
}

/** Creates a safe text-input box which respects backspaces, shifts, and most regular characters.
   Must be given a keyboardKey reference because RaylibDrawHandle doesn't implement DerefMut
*/
pub fn advanced_input(
    rld: &mut RaylibDrawHandle,
    key: &Option<KeyboardKey>,
    font: impl AsRef<raylib::ffi::Font>,
    bounds: Rectangle,
    text: &mut String,
    title: &str,
) {
    static mut UPPERCASE: bool = false; // Used to store shift keys for next run of function

    let spacing = 1.0;
    let fontsize = 20.0;
    let title = Some(CString::new(title).expect("Failed to convert to CString!"));
    let title = title.as_deref(); // Get Option<&CStr> from Option<CString>
    let t_bound = Rectangle {
        x: bounds.x,
        y: bounds.y + 8.0,
        width: bounds.width,
        height: bounds.height - 8.0,
    };
    rld.gui_group_box(t_bound, title);
    // Draw pre-existing text
    rld.draw_text_ex(
        &font,
        text.as_str(),
        rvec2(bounds.x as i32, bounds.y as i32 + 32),
        fontsize,
        spacing,
        Color::BLACK,
    );

    // Ensure that the cursor is in the vicinity of the box to be drawn into
    if !bounds.check_collision_point_rec(rld.get_mouse_position()) {
        return;
    }

    // Small closure to ensure that shifts are properly applied
    let mut push_uppercase = |c: char| {
        if unsafe { UPPERCASE } {
            text.push(c.to_ascii_uppercase());
            unsafe {
                UPPERCASE = false;
            }
        } else {
            text.push(c);
        }
    };

    if let Some(k) = key {
        match k {
            KEY_A => push_uppercase('a'),
            KEY_B => push_uppercase('b'),
            KEY_C => push_uppercase('c'),
            KEY_D => push_uppercase('d'),
            KEY_E => push_uppercase('e'),
            KEY_F => push_uppercase('f'),
            KEY_G => push_uppercase('g'),
            KEY_H => push_uppercase('h'),
            KEY_I => push_uppercase('i'),
            KEY_J => push_uppercase('j'),
            KEY_K => push_uppercase('k'),
            KEY_L => push_uppercase('l'),
            KEY_M => push_uppercase('m'),
            KEY_N => push_uppercase('n'),
            KEY_O => push_uppercase('o'),
            KEY_P => push_uppercase('p'),
            KEY_Q => push_uppercase('q'),
            KEY_R => push_uppercase('r'),
            KEY_S => push_uppercase('s'),
            KEY_T => push_uppercase('t'),
            KEY_U => push_uppercase('u'),
            KEY_V => push_uppercase('v'),
            KEY_W => push_uppercase('w'),
            KEY_X => push_uppercase('x'),
            KEY_Y => push_uppercase('y'),
            KEY_Z => push_uppercase('z'),

            // KEY_SPACE => text.push(' '),
            KEY_MINUS => {
                if unsafe { UPPERCASE } {
                    text.push('_');
                    unsafe {
                        UPPERCASE = false;
                    }
                }
            }

            KEY_BACKSPACE => {
                text.pop();
            }
            KEY_LEFT_SHIFT => unsafe {
                UPPERCASE = true;
            },

            _ => (),
        };
    }
}

/// Print `text` using the default font with given fontsize. Text is centered on x and y.
pub fn draw_text_centered(
    rld: &mut RaylibDrawHandle,
    font: &Font,
    text: &str,
    posx: i32,
    posy: i32,
    fontsize: i32,
    clr: Color,
) {
    // let fnt: WeakFont = rld.get_font_default();

    let spacing = 1.0;
    let txtsize = measure_text_ex(font, text, fontsize as f32, spacing);
    rld.draw_text_ex(
        font,
        text,
        rvec2(
            posx as f32 - txtsize.x as f32 / 2.0,
            posy as f32 - txtsize.y as f32 / 2.0,
        ),
        fontsize as f32,
        spacing,
        clr,
    );
}

// Return largest of `a` and `b`.
pub fn max(a: i32, b: i32) -> i32 {
    if a > b {
        a
    } else {
        b
    }
}

/// Draws a rounded rectangle with the default rgui style
pub fn ds_rounded_rectangle(
    rd: &mut RaylibDrawHandle,
    rec: impl Into<ffirect>,
    roundness: f32,
    segments: i32,
) {
    let base_color = mutex_get(&BASE_COLOR_NORMAL);
    //     Color::get_color(rd.gui_get_style(
    //     GuiControl::DEFAULT,
    //     GuiDefaultProperty::BACKGROUND_COLOR as i32,
    // ));

    rd.draw_rectangle_rounded(rec, roundness, segments, base_color);
}

/// Draws a rounded rectangle outline with the default rgui style
pub fn ds_rounded_rectangle_lines(
    rd: &mut RaylibDrawHandle,
    rec: impl Into<ffirect>,
    roundness: f32,
    segments: i32,
    line_width: i32,
) {
    let border_color = mutex_get(&BORDER_COLOR_NORMAL);

    rd.draw_rectangle_rounded_lines(rec, roundness, segments, line_width, border_color);
}

/// Draws a rounded button centered.
/// Returns a boolean for whether it was pressed and a vector2 which points to the top left corner of the drawn button.
pub fn ds_rounded_button_centered(
    rd: &mut RaylibDrawHandle,
    font: &Font,
    rec: impl Into<ffirect>,
    text: Option<&str>,
    active: bool,
) -> (bool, Vector2) {
    let mut rec = {
        let rec = rec.into();
        let rec: Rectangle = rec.into();
        rec
    };

    rec.x -= rec.width / 2.0;
    rec.y -= rec.height / 2.0;

    ds_rounded_button(rd, font, rec, text, active)
}

/// Draws a rounded button with the default rgui style
/// Returns a boolean for whether it was pressed and a vector2 which points to the top left corner of the drawn button.
pub fn ds_rounded_button(
    rd: &mut RaylibDrawHandle,
    font: &Font,
    rec: impl Into<ffirect>,
    text: Option<&str>,
    active: bool,
) -> (bool, Vector2) {
    let mut rec = {
        let rec = rec.into();
        let rec: Rectangle = rec.into();
        rec
    };

    let border_w: f32 = mutex_get(&BORDER_WIDTH) as f32;

    // let mut rec: Rectangle = rec.into();
    rec.x += border_w;
    rec.y += border_w;
    rec.width -= 2.0 * border_w;
    rec.height -= 2.0 * border_w;

    let base_color: Color;
    let border_color: Color;

    let mut pressed = false;

    if !active {
        base_color = mutex_get(&BASE_COLOR_DISABLED);
        border_color = mutex_get(&BORDER_COLOR_DISABLED);
    } else if !rec.check_collision_point_rec(rd.get_mouse_position()) {
        base_color = mutex_get(&BASE_COLOR_NORMAL);
        border_color = mutex_get(&BORDER_COLOR_NORMAL);
    } else if rd.is_mouse_button_down(MouseButton::MOUSE_LEFT_BUTTON) {
        if rd.is_mouse_button_pressed(MouseButton::MOUSE_LEFT_BUTTON) {
            pressed = true;
        }
        base_color = mutex_get(&BASE_COLOR_PRESSED);
        border_color = mutex_get(&BORDER_COLOR_PRESSED);
    } else {
        base_color = mutex_get(&BASE_COLOR_FOCUSED);
        border_color = mutex_get(&BORDER_COLOR_FOCUSED);
    }

    rd.draw_rectangle_rounded(rec, 0.4, 5, base_color);
    rd.draw_rectangle_rounded_lines(rec, 0.4, 5, border_w as i32, border_color);

    if let Some(s) = text {
        // let _ctext = CString::new(s);
        draw_text_centered(
            rd,
            font,
            s,
            (rec.x + rec.width / 2.0) as i32,
            (rec.y + rec.height / 2.0) as i32,
            16,
            if active {
                mutex_get(&TEXT_COLOR_DISABLED)
            } else {
                Color::BLACK
            },
        );
    }

    (pressed, rvec2(rec.x, rec.y))
}

/// Adds `x` and `y` to `rec.x` and `rec.y`.
pub fn rel_rect(rec: Rectangle, x: i32, y: i32, width: f32, height: f32) -> Rectangle {
    rrect(rec.x + x as f32, rec.y + y as f32, width, height)
}

/// Get centerpoint of a rectangle
pub fn rect_midpoint(rec: Rectangle) -> (i32, i32) {
    (
        (rec.x + rec.width / 2.0) as i32,
        (rec.y + rec.height / 2.0) as i32,
    )
}

/// A scroll selection box that automatically determines the right height to draw itself to display
/// a given number of items. If the given number of items is invalid, it will display 1 item.
pub fn ds_scroll_selection_auto(
    rd: &mut RaylibDrawHandle,
    font: &Font,
    rec: Rectangle,
    mut amt_display: i32,
    selections: &[String],
    selection: &mut i32,
    top_item_index: &mut i32,
) -> bool {
    if amt_display < 0 && amt_display > selections.len() as i32 {
        amt_display = 1;
    }
    let mut rec = rrect(rec.x, rec.y, rec.width, amt_display * 30 + 12);
    rec.y -= rec.height;

    ds_scroll_selection(rd, font, rec, selections, selection, top_item_index)
}

/// A Scrollable selection box with mouse control
pub fn ds_scroll_selection(
    rd: &mut RaylibDrawHandle,
    font: &Font,
    rec: Rectangle,
    selections: &[String],
    selection: &mut i32,
    top_item_index: &mut i32,
) -> bool {
    ds_scroll_selection_ex(rd, font, rec, selections, selection, top_item_index, 16)
}

/// A Scrollable selection box with extended configuration
pub fn ds_scroll_selection_ex(
    rd: &mut RaylibDrawHandle,
    font: &Font,
    rec: Rectangle,
    selections: &[String],
    selection: &mut i32,
    top_item_index: &mut i32,
    fontsize: i32,
) -> bool {
    let active = rec.check_collision_point_rec(rd.get_mouse_position());
    let border_w = mutex_get(&BORDER_WIDTH);
    let mut item_rect = rrect(rec.x + 2.0, rec.y - 30.0, rec.width - 4.0, 30);

    rd.draw_rectangle_rounded(rec, 0.1, 5, mutex_get(&BASE_COLOR_NORMAL));
    rd.draw_rectangle_rounded_lines(rec, 0.1, 5, border_w, mutex_get(&BORDER_COLOR_NORMAL));

    let num_items = (rec.height - 12.0) as i32 / item_rect.height as i32;

    // Scroll Selection Logic
    if *top_item_index >= selections.len() as i32 {
        *top_item_index = selections.len() as i32 - num_items;
    }
    if *top_item_index < 0 {
        *top_item_index = 0;
    }

    for n in 0..num_items {
        item_rect.y += item_rect.height + 2.0;
        let (cx, cy) = rect_midpoint(item_rect);
        let txt = selections.get((n + *top_item_index) as usize);
        if let Some(t) = txt {
            draw_text_centered(rd, font, t, cx, cy, fontsize, Color::BLACK);
        }
        if n + *top_item_index == *selection {
            rd.draw_rectangle_rounded(item_rect, 0.5, 4, Color::GRAY.fade(0.40));
        }
        if item_rect.check_collision_point_rec(rd.get_mouse_position()) {
            if txt.is_some() {
                rd.draw_rectangle_rounded(item_rect, 0.5, 4, Color::WHITE.fade(0.40));
            }

            if rd.is_mouse_button_pressed(MouseButton::MOUSE_LEFT_BUTTON)
                && *top_item_index + n < selections.len() as i32
            {
                *selection = n + *top_item_index;
                return true;
            }
        }
    }

    if active {
        *top_item_index += -rd.get_mouse_wheel_move() as i32;
        if *top_item_index < 0 {
            *top_item_index = 0;
        } else if *top_item_index >= selections.len() as i32 {
            *top_item_index = selections.len() as i32 - 1;
        }
    }

    false
}

/// Draw a slider width a given width and height, centered w.r.t. X and Y axes and
/// With the title text above it.
pub fn ds_draw_slider_centered(
    d: &mut RaylibDrawHandle,
    font: &Font,
    title: &str,
    center: Vector2,
    width: f32,
    height: f32,
    value: &mut i32,
    min_val: f32,
    max_val: f32,
    center_title: bool,
) {
    if center_title {
        draw_text_centered(
            d,
            font,
            title,
            center.x as i32,
            (center.y - height / 2.0) as i32,
            16,
            Color::BLACK,
        );
    } else {
        d.draw_text(title, center.x as i32, center.x as i32, 16, Color::BLACK);
    }

    *value = d.gui_slider_bar(
        rrect(
            center.x - width / 2.0,
            center.y + height / 2.0,
            width,
            height,
        ),
        None,
        None,
        *value as f32,
        min_val,
        max_val,
    ) as i32;
}

/// Draw a toggle with the default style, rounded corners, and centered at the position given by
/// the rec's x and y.
pub fn ds_draw_toggle_rounded_centered(
    d: &mut RaylibDrawHandle,
    font: &Font,
    text: Option<&str>,
    rec: Rectangle,
    val: &mut bool,
) {
    let rec = rrect(
        rec.x - rec.width / 2.,
        rec.y - rec.height / 2.,
        rec.width,
        rec.height,
    );
    ds_draw_toggle_rounded(d, font, text, rec, val);
}

/// Draw a toggle with the default style. Toggle has rounded corners (as opposed to RGui's toggle)
pub fn ds_draw_toggle_rounded(
    d: &mut RaylibDrawHandle,
    font: &Font,
    text: Option<&str>,
    rec: Rectangle,
    val: &mut bool,
) {
    let border_color;
    let base_color;
    let text_color;

    let rec = rrect(rec.x + 2., rec.y + 2., rec.width - 4., rec.height - 4.);
    let active = rec.check_collision_point_rec(d.get_mouse_position());

    if *val {
        border_color = mutex_get(&BORDER_COLOR_PRESSED);
        base_color = mutex_get(&BASE_COLOR_PRESSED);
    } else if active {
        border_color = mutex_get(&BORDER_COLOR_FOCUSED);
        base_color = mutex_get(&BASE_COLOR_FOCUSED);
    } else {
        border_color = mutex_get(&BORDER_COLOR_NORMAL);
        base_color = mutex_get(&BASE_COLOR_NORMAL);
    }
    text_color = Color::BLACK;

    d.draw_rectangle_rounded_lines(rec, 0.2, 5, 2, border_color);
    d.draw_rectangle_rounded(rec, 0.2, 5, base_color);
    if let Some(txt) = text {
        draw_text_centered(
            d,
            font,
            txt,
            (rec.x + rec.width / 2.) as i32,
            (rec.y + rec.height / 2.) as i32,
            16,
            text_color,
        );
        // d.draw_text_ex(font,
        //                txt,
        //                rvec2(rec.x, rec.y),
        //                16.,
        //                1.,
        //                text_color);
    }

    if active && d.is_mouse_button_pressed(MouseButton::MOUSE_LEFT_BUTTON) {
        *val = !*val;
    }
}
