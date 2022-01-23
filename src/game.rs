#![allow(dead_code, unused_imports, unused_variables, non_snake_case)]

use lazy_static;
use raylib::consts::KeyboardKey::*;
use raylib::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::ffi::CString;
use std::fs;
use std::io::prelude::*;
use std::mem::drop;
use std::sync::{atomic, Arc, Mutex};
use toml;
use crate::object::*;
use std::cell::RefCell;
use std::rc;
use raylib::ffi::Rectangle as ffirect;

/**
 * Holds the data for an object type. Note that this cannot create an instance of an object; it just holds the default configuration and spritesheet information.
 */
#[derive(Serialize, Deserialize, Debug)]
pub struct ObjectConfig {
    pub name: String,
    pub id: i32,
    pub dim: (i32, i32),
    pub sides: i32,
    pub img_per_side: i32,
    pub category: String,
    pub image_speed: Option<i32>,
    pub default_b_box: Option<(i32, i32, i32, i32)>,
}

impl ObjectConfig {
    pub fn new() -> Self {
        Self {
            name: String::new(),
            id: 0,
            dim: (0, 0),
            sides: 0,
            img_per_side: 0,
            category: String::new(),
            image_speed: None,
            default_b_box: None,
        }
    }
}

impl Default for ObjectConfig {
    fn default() -> Self {
        Self::new()
    }
}

lazy_static::lazy_static! {
    /** Hash Map of object types that are loaded into memory. Each map element contains an Arc with a texture and an ObjectConfig object. It is static so that all modules which import game can use it.
     */
    pub static ref LOADED_TEXTURES: Mutex<HashMap<i32, Arc<(Texture2D, ObjectConfig)>>> = Mutex::new(HashMap::new());

    /** A static boolean used to tell if the game textures and data have been loaded yet.
     */
    static ref INITIALIZED: atomic::AtomicBool = atomic::AtomicBool::new(false);

}

/** Initializes the game memory. Should only be done once during the program. Call destroy() to clear the memory created by this. After destroying the game, it is possible to instantiate it again.
 */
pub fn init(rl: &mut RaylibHandle, rt: &RaylibThread) {
    if INITIALIZED.load(atomic::Ordering::Relaxed) {
        return;
    }

    for entry in fs::read_dir("obj/").unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        print!("Attempting load {}... ", path.to_str().unwrap());
        let imgpath = path.join("spr.png");
        let img = Image::load_image(
            imgpath
                .to_str()
                .expect("Unable to convert image path to string!"),
        )
        .expect("Unable to load image!");

        let mut objconf = fs::File::open(path.join("obj.toml").to_str().unwrap()).unwrap();
        let mut confstr = String::new();
        let _ = objconf.read_to_string(&mut confstr).unwrap();

        let obj: ObjectConfig =
            toml::from_str(confstr.as_str()).expect("Unable to parse TOML Object configuration!");

        LOADED_TEXTURES
            .lock()
            .expect("Unable to lock LOADED_TEXTURES mutex!")
            .insert(
                obj.id,
                Arc::new((
                    rl.load_texture_from_image(rt, &img)
                        .expect("Unable to laod texture from loaded imag!"),
                    obj,
                )),
            );
        println!("DONE");

        // Clean up
        drop(img);
    }

    INITIALIZED.store(true, atomic::Ordering::Relaxed);
}

/** Clear the memory that has been allocated for textures and object configurations. Essentially forces the game to forget all object types.
 */
pub fn destroy() {
    if INITIALIZED.load(atomic::Ordering::Relaxed) {
        return;
    }
    for item in LOADED_TEXTURES
        .lock()
        .expect("Unable to lock LOADED_TEXTURES mutex!")
        .iter_mut()
    {
        let item = item.1;
        while Arc::strong_count(item) > 1 {
            unsafe {
                Arc::decrement_strong_count(&item);
            }
        }
    }
    LOADED_TEXTURES
        .lock()
        .expect("Unable to lock LOADED_TEXTURES mutex!")
        .clear();

    INITIALIZED.store(false, atomic::Ordering::Relaxed);
}

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

// fn simple_input(
//     rld: &mut RaylibDrawHandle,
//     bounds: Rectangle,
//     text: &mut String,
//     buf: &mut [u8]
// ) {
//
//     let res = rld.gui_text_box(bounds, buf, bounds.check_collision_point_rec(rld.get_mouse_position()));
//     if res {
//         println!("Recieved a valid update");
//         *text = String::from_utf8(buf.to_vec())
//             .expect("Invalid UTF-8!")
//             .to_lowercase();
//         text.truncate(text.find('\0').expect("Not a null-terminated string!"));
//         println!("Text is now: {}", text);
//     }
// }
pub fn draw_text_centered(
    rld: &mut RaylibDrawHandle,
//    font: Option<impl AsRef<raylib::ffi::Font>>,
    text: &str,
    posx: i32,
    posy: i32,
    fontsize: i32,
    clr: Color,
) {
    let fnt: WeakFont = rld.get_font_default();

    let spacing = 1.0;
    let txtsize = measure_text_ex(&fnt, text, fontsize as f32, spacing);
    rld.draw_text_ex(
        &fnt,
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

lazy_static::lazy_static! {
    static ref BASE_COLOR_NORMAL: Mutex<Color> = Mutex::new(Color::WHITE);
    static ref BORDER_COLOR_NORMAL: Mutex<Color> = Mutex::new(Color::WHITE);
    static ref BORDER_COLOR_FOCUSED: Mutex<Color> = Mutex::new(Color::WHITE);
    static ref BASE_COLOR_FOCUSED: Mutex<Color> = Mutex::new(Color::WHITE);
    static ref BORDER_COLOR_PRESSED: Mutex<Color> = Mutex::new(Color::WHITE);
    static ref BASE_COLOR_PRESSED: Mutex<Color> = Mutex::new(Color::WHITE);
    static ref BORDER_WIDTH: Mutex<i32> = Mutex::new(0);
}

// Sets the value in the given mutex to be equivalent or a copy of `val`
pub fn mutex_set<T: Clone>(
    m: &Mutex<T>, 
    val: T
) {
    *m.lock().expect("Unable to lock mutex given to MutexSet") = val.clone();
}

pub fn mutex_get<T: Clone>(
    m: &Mutex<T>
) -> T {
    m.lock().expect("Unable to lock mutex_get mutex").clone()
}

pub fn color_init(
    rd: &mut RaylibHandle
) {
    mutex_set(&BASE_COLOR_NORMAL, Color::get_color(rd.gui_get_style(GuiControl::BUTTON, GuiControlProperty::BASE_COLOR_NORMAL as i32)));
    mutex_set(&BORDER_COLOR_NORMAL, Color::get_color(rd.gui_get_style(GuiControl::BUTTON, GuiControlProperty::BORDER_COLOR_NORMAL as i32)));
    mutex_set(&BORDER_COLOR_FOCUSED, Color::get_color(rd.gui_get_style(GuiControl::BUTTON, GuiControlProperty::BORDER_COLOR_FOCUSED as i32)));
    mutex_set(&BASE_COLOR_FOCUSED, Color::get_color(rd.gui_get_style(GuiControl::BUTTON, GuiControlProperty::BASE_COLOR_FOCUSED as i32)));
    mutex_set(&BORDER_COLOR_PRESSED, Color::get_color(rd.gui_get_style(GuiControl::BUTTON, GuiControlProperty::BORDER_COLOR_PRESSED as i32)));
    mutex_set(&BASE_COLOR_PRESSED, Color::get_color(rd.gui_get_style(GuiControl::BUTTON, GuiControlProperty::BASE_COLOR_PRESSED as i32)));
    mutex_set(&BORDER_WIDTH, rd.gui_get_style(GuiControl::BUTTON, GuiControlProperty::BORDER_WIDTH as i32));
}

pub fn max(
    a: i32,
    b: i32
) -> i32 {
    if a > b {
        a
    } else {
        b
    }
}

pub fn insert_object(
    v: &mut Vec<Vec<rc::Rc<RefCell<dyn Object>>>>,
    obj: rc::Rc<RefCell<dyn Object>>,
) {
    let depth = obj.borrow().get_depth() as usize;
    v.get_mut(depth)
        .expect("Invalid depth for object!")
        .push(obj);
}

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

pub fn get_all_obj(
    v: &[Vec<rc::Rc<RefCell<dyn Object>>>],
) -> Vec<rc::Rc<RefCell<dyn Object>>> {
    let mut res = Vec::new();

    for depth in v.iter() {
        res.append(&mut depth.clone());
    }

    res
}

pub fn get_viewport(scr_w: i32, scr_h: i32) -> Rectangle {
    rrect(0, 0, scr_w, scr_h - 256)
}

pub fn ds_rounded_rectangle(
    rd: &mut RaylibDrawHandle,
    rec: impl Into<ffirect>,
    roundness: f32,
    segments: i32,
) {
    let D_BASE_COLOR_NORMAL = Color::get_color(rd.gui_get_style(GuiControl::DEFAULT, GuiDefaultProperty::BACKGROUND_COLOR as i32));
    
    rd.draw_rectangle_rounded(rec, roundness, segments, D_BASE_COLOR_NORMAL);
    
}
pub fn ds_rounded_rectangle_lines(
    rd: &mut RaylibDrawHandle,
    rec: impl Into<ffirect>,
    roundness: f32,
    segments: i32,
    line_width: i32
) {
    let D_BORDER_COLOR_NORMAL = Color::get_color(rd.gui_get_style(GuiControl::DEFAULT, GuiDefaultProperty::LINE_COLOR as i32));
    
    rd.draw_rectangle_rounded_lines(rec, roundness, segments, line_width, D_BORDER_COLOR_NORMAL);
    
}

pub fn ds_rounded_button_centered(
    rd: &mut RaylibDrawHandle,
    rec: impl Into<ffirect>,
    text: Option<&str>
) -> (bool, Vector2) {
    let mut rec = {
        let rec = rec.into();
        let rec: Rectangle = rec.into();
        rec
    };
    
    rec.x -= rec.width/2.0;
    rec.y -= rec.height/2.0;
    
    ds_rounded_button(rd, rec, text)
}

pub fn ds_rounded_button(
    rd: &mut RaylibDrawHandle,
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
    rec.width -= 2.0 *border_w;
    rec.height -= 2.0 *border_w;
    
    let base_color: Color;
    let border_color: Color;
    
    let mut pressed = false;

    if !rec.check_collision_point_rec(rd.get_mouse_position()) {
        base_color = mutex_get(&BASE_COLOR_NORMAL);
        border_color = mutex_get(&BORDER_COLOR_NORMAL);
    } else {
        if rd.is_mouse_button_down(MouseButton::MOUSE_LEFT_BUTTON) {
            if rd.is_mouse_button_pressed(MouseButton::MOUSE_LEFT_BUTTON) {
                pressed = true;
            }
            base_color = mutex_get(&BASE_COLOR_PRESSED);
            border_color = mutex_get(&BORDER_COLOR_PRESSED);
        } else {
            base_color = mutex_get(&BASE_COLOR_FOCUSED);
            border_color = mutex_get(&BORDER_COLOR_FOCUSED);
        }
    }

    rd.draw_rectangle_rounded(
        rec,
        0.4,
        5,
        base_color
    );
    rd.draw_rectangle_rounded_lines(
        rec,
        0.4,
        5,
        border_w as i32,
        border_color
    );

    if let Some(s) = text {
        // let _ctext = CString::new(s);
        draw_text_centered(
            rd, 
            s, 
            (rec.x + rec.width/2.0) as i32, 
            (rec.y + rec.height/2.0) as i32, 
            16, 
            Color::BLACK
        );
    }


    (pressed, rvec2(rec.x, rec.y))
}

pub fn rel_rect(rec: Rectangle, x: i32, y: i32, width: f32, height: f32) -> Rectangle {
    rrect(
        rec.x + x as f32,
        rec.y + y as f32,
        width,
        height,
    )
}

pub fn rect_midpoint(rec: Rectangle) -> (i32, i32) {
    ((rec.x + rec.width/2.0) as i32, (rec.y + rec.height/2.0) as i32)
}


pub fn ds_scroll_selection(
    rd: &mut RaylibDrawHandle,
    rec: Rectangle,
    selections: &Vec<String>,
    selection: &mut i32
) -> bool {
    let active = rec.check_collision_point_rec(rd.get_mouse_position());
    let border_w = mutex_get(&BORDER_WIDTH);
    let mut item_rect = rrect(rec.x + 2.0, rec.y - 20.0, rec.width - 4.0, 20);

    rd.draw_rectangle_rounded(
        rec,
        0.4,
        5,
        mutex_get(&BASE_COLOR_NORMAL)
    );
    rd.draw_rectangle_rounded_lines(
        rec,
        0.4,
        5,
        border_w,
        mutex_get(&BORDER_COLOR_NORMAL)
    );
    
    let num_items = (rec.height - 12.0) as i32 / 20;
    
    // Scroll Selection Logic
    if *selection >= selections.len() as i32 {
        *selection = selections.len() as i32 - num_items;
    }
    if *selection < 0 {
        *selection = 0;
    }
    
    for n in 0..num_items {
        item_rect.y += 22.0;
        let (cx, cy) = rect_midpoint(item_rect);
        let txt = selections.get((n + *selection) as usize);
        if let Some(t) = txt {
            draw_text_centered(rd,
                                     t,
                                     cx,
                                     cy,
                                     12,
                                     Color::BLACK
                        );
        }
        if item_rect.check_collision_point_rec(rd.get_mouse_position()) {
            rd.draw_rectangle_rounded(item_rect,
                                      0.5,
                                      4,
                                      Color::WHITE.fade(0.40)
                                    );
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
            *selection = selections.len() as i32 -1;
        }
    }

    false    
}


