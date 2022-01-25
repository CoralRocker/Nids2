// UTIL.RS
// Use for functions dealing with objects and displaying textures and menus.
// Not for memory allocation or system-level things.

use raylib::prelude::*;
use std::cell::RefCell;
use std::rc;
use crate::game::*;
use crate::object::*;
use raylib::ffi::Rectangle as ffirect;
use std::ffi::CString;
use std::sync::Arc;
use std::collections::HashMap;
use raylib::consts::KeyboardKey::*;

/// Get Vector of all unique categories that contain objects.
pub fn get_all_types() -> Vec<String> {
    let mut result = Vec::new();

    for item in LOADED_TEXTURES
        .lock()
        .expect("Unable to lock LOADED_TEXTURES mutex")
        .iter()
    {
        let item = item.1;
        let item = &item.1;
        result.push(item.category.clone());
    }

    result.sort_unstable();
    result.dedup();

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
pub fn get_all_objects_sorted() -> HashMap<String, Vec<Arc<(Texture2D, ObjectConfig)>>> {
    let mut result = HashMap::new();

    let types = get_all_types();

    let all_objs = get_all_objects();

    for t in types.iter() {
        let mut objs = Vec::new();
        for obj in all_objs.iter() {
            if obj.1.category == *t {
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
    title: &String,
) {
    static mut UPPERCASE: bool = false;

    let spacing = 1.0;
    let fontsize = 20.0;
    // let txt_width = measure_text_ex(&font, text.as_str(), fontsize, spacing);

    // let disp = Some(CString::new(text.as_bytes()).expect("Failed to convert to CString!"));
    // let disp = disp.as_deref();
    let title = Some(CString::new(title.as_bytes()).expect("Failed to convert to CString!"));
    let title = title.as_deref();
    let t_bound = Rectangle {
        x: bounds.x,
        y: bounds.y + 8.0,
        width: bounds.width,
        height: bounds.height - 8.0,
    };
    rld.gui_group_box(t_bound, title);
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


pub fn max(a: i32, b: i32) -> i32 {
    if a > b {
        a
    } else {
        b
    }
}

/// Insert an object into the right depth in an object list.
pub fn insert_object(
    v: &mut Vec<Vec<rc::Rc<RefCell<dyn Object>>>>,
    obj: rc::Rc<RefCell<dyn Object>>,
) {
    let depth = obj.borrow().get_depth() as usize;
    v.get_mut(depth)
        .expect("Invalid depth for object!")
        .push(obj);
}

/// Check if object is already in the correct place in the objects list.
pub fn is_object_correctly_placed(
    v: &[Vec<rc::Rc<RefCell<dyn Object>>>],
    obj: rc::Rc<RefCell<dyn Object>>,
) -> bool {
    let iter = &v
        .get(obj.borrow().get_depth() as usize)
        .expect("Object depth is invalid!");

    iter.iter()
        .any(|x| -> bool { x.borrow().get_id() == obj.borrow().get_id() })
}

/** Find an object in the list by it's ID, remove it, and add it back at the correct depth. If the object is already in the correct position, this does nothing.
 */
pub fn update_object_in_list(
    v: &mut Vec<Vec<rc::Rc<RefCell<dyn Object>>>>,
    obj: rc::Rc<RefCell<dyn Object>>,
) {
    if is_object_correctly_placed(v, obj.clone()) {
        return;
    }
    let id = obj.borrow().get_id();
    for depth in v.iter_mut() {
        if let Some(p) = depth
            .iter()
            .position(|x| -> bool { x.borrow().get_id() == id })
        {
            depth.remove(p);
            break;
        }
    }
    insert_object(v, obj);
}

/// Return all objects in the objects list, flattened.
pub fn get_all_obj(v: &[Vec<rc::Rc<RefCell<dyn Object>>>]) -> Vec<rc::Rc<RefCell<dyn Object>>> {
    let mut res = Vec::new();

    for depth in v.iter() {
        res.append(&mut depth.clone());
    }

    res
}

/// Deprecated
pub fn get_viewport(scr_w: i32, scr_h: i32) -> Rectangle {
    rrect(0, 0, scr_w, scr_h - 256)
}

/// Draws a rounded rectangle with the default rgui style
pub fn ds_rounded_rectangle(
    rd: &mut RaylibDrawHandle,
    rec: impl Into<ffirect>,
    roundness: f32,
    segments: i32,
) {
    let D_BASE_COLOR_NORMAL = Color::get_color(rd.gui_get_style(
        GuiControl::DEFAULT,
        GuiDefaultProperty::BACKGROUND_COLOR as i32,
    ));

    rd.draw_rectangle_rounded(rec, roundness, segments, D_BASE_COLOR_NORMAL);
}

/// Draws a rounded rectangle outline with the default rgui style
pub fn ds_rounded_rectangle_lines(
    rd: &mut RaylibDrawHandle,
    rec: impl Into<ffirect>,
    roundness: f32,
    segments: i32,
    line_width: i32,
) {
    let D_BORDER_COLOR_NORMAL = Color::get_color(
        rd.gui_get_style(GuiControl::DEFAULT, GuiDefaultProperty::LINE_COLOR as i32),
    );

    rd.draw_rectangle_rounded_lines(rec, roundness, segments, line_width, D_BORDER_COLOR_NORMAL);
}

/// Draws a rounded button centered.
pub fn ds_rounded_button_centered(
    rd: &mut RaylibDrawHandle,
    font: &Font,
    rec: impl Into<ffirect>,
    text: Option<&str>,
) -> (bool, Vector2) {
    let mut rec = {
        let rec = rec.into();
        let rec: Rectangle = rec.into();
        rec
    };

    rec.x -= rec.width / 2.0;
    rec.y -= rec.height / 2.0;

    ds_rounded_button(rd, font, rec, text)
}

/// Draws a rounded button with the default rgui style
pub fn ds_rounded_button(
    rd: &mut RaylibDrawHandle,
    font: &Font,
    rec: impl Into<ffirect>,
    text: Option<&str>,
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

    if !rec.check_collision_point_rec(rd.get_mouse_position()) {
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
            Color::BLACK,
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

/// A Scrollable selection box with mouse control
pub fn ds_scroll_selection(
    rd: &mut RaylibDrawHandle,
    font: &Font,
    rec: Rectangle,
    selections: &Vec<String>,
    selection: &mut i32,
) -> bool {
    let active = rec.check_collision_point_rec(rd.get_mouse_position());
    let border_w = mutex_get(&BORDER_WIDTH);
    let mut item_rect = rrect(rec.x + 2.0, rec.y - 30.0, rec.width - 4.0, 30);

    rd.draw_rectangle_rounded(rec, 0.4, 5, mutex_get(&BASE_COLOR_NORMAL));
    rd.draw_rectangle_rounded_lines(rec, 0.4, 5, border_w, mutex_get(&BORDER_COLOR_NORMAL));

    let num_items = (rec.height - 12.0) as i32 / item_rect.height as i32;

    // Scroll Selection Logic
    if *selection >= selections.len() as i32 {
        *selection = selections.len() as i32 - num_items;
    }
    if *selection < 0 {
        *selection = 0;
    }

    for n in 0..num_items {
        item_rect.y += item_rect.height + 2.0;
        let (cx, cy) = rect_midpoint(item_rect);
        let txt = selections.get((n + *selection) as usize);
        if let Some(t) = txt {
            draw_text_centered(rd, font, t, cx, cy, 16, Color::BLACK);
        }
        if item_rect.check_collision_point_rec(rd.get_mouse_position()) {
            rd.draw_rectangle_rounded(item_rect, 0.5, 4, Color::WHITE.fade(0.40));
            if rd.is_mouse_button_pressed(MouseButton::MOUSE_LEFT_BUTTON) {
                *selection += n;
                return true;
            }
        }
    }

    if active {
        *selection += -rd.get_mouse_wheel_move() as i32;
        if *selection < 0 {
            *selection = 0;
        } else if *selection >= selections.len() as i32 {
            *selection = selections.len() as i32 - 1;
        }
    }

    false
}

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
    center_title: bool
) {
    if center_title {
        draw_text_centered(d, font, title, center.x as i32, (center.y - height/2.0) as i32, 16, Color::BLACK);
    }else{
        d.draw_text(title, center.x as i32, center.x as i32, 16, Color::BLACK);
    }

    *value = d.gui_slider_bar(rrect(center.x - width/2.0, center.y + height/2.0,
                                    width, height),
                                None, None, *value as f32, min_val, max_val) as i32;
}

