// GAME.RS
// Use for system-level things such as statics, unsafe code, and memory loading at
// the start of the game. Should not deal with high-level objects and no drawing.

#![allow(dead_code, unused_imports, unused_variables, non_snake_case)]

use crate::object::*;
use crate::*;
use lazy_static;
use raylib::consts::KeyboardKey::*;
use raylib::ffi::Rectangle as ffirect;
use raylib::prelude::*;
use serde::{Deserialize, Serialize};
use std::cell::RefCell;
use std::collections::HashMap;
use std::ffi::CString;
use std::fs;
use std::io::prelude::*;
use std::mem::drop;
use std::rc;
use std::sync::{atomic, Arc, Mutex};
use toml;

/**
 * Holds the data for an object type. Note that this cannot create an instance of an object; it just holds the default configuration and spritesheet information.
 */
#[derive(Serialize, Deserialize, Debug, Clone)]
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

lazy_static::lazy_static! {
    pub static ref BASE_COLOR_NORMAL: Mutex<Color> = Mutex::new(Color::WHITE);
    pub static ref BORDER_COLOR_NORMAL: Mutex<Color> = Mutex::new(Color::WHITE);
    pub static ref BORDER_COLOR_FOCUSED: Mutex<Color> = Mutex::new(Color::WHITE);
    pub static ref BASE_COLOR_FOCUSED: Mutex<Color> = Mutex::new(Color::WHITE);
    pub static ref BORDER_COLOR_PRESSED: Mutex<Color> = Mutex::new(Color::WHITE);
    pub static ref BASE_COLOR_PRESSED: Mutex<Color> = Mutex::new(Color::WHITE);
    pub static ref BASE_COLOR_DISABLED: Mutex<Color> = Mutex::new(Color::WHITE);
    pub static ref BORDER_COLOR_DISABLED: Mutex<Color> = Mutex::new(Color::WHITE);
    pub static ref TEXT_COLOR_DISABLED: Mutex<Color> = Mutex::new(Color::WHITE);
    pub static ref BORDER_WIDTH: Mutex<i32> = Mutex::new(0);
}

/// Sets the value in the given mutex to be equivalent or a copy of `val`
pub fn mutex_set<T: Clone>(m: &Mutex<T>, val: T) {
    *m.lock().expect("Unable to lock mutex given to MutexSet") = val;
}

/// Extract a copy of something from a mutex.
pub fn mutex_get<T: Clone>(m: &Mutex<T>) -> T {
    m.lock().expect("Unable to lock mutex_get mutex").clone()
}

/// Initializes all the default color pallette colors.
pub fn color_init(rd: &mut RaylibHandle) {
    mutex_set(
        &BASE_COLOR_NORMAL,
        Color::get_color(rd.gui_get_style(
            GuiControl::BUTTON,
            GuiControlProperty::BASE_COLOR_NORMAL as i32,
        )),
    );
    mutex_set(
        &BORDER_COLOR_NORMAL,
        Color::get_color(rd.gui_get_style(
            GuiControl::BUTTON,
            GuiControlProperty::BORDER_COLOR_NORMAL as i32,
        )),
    );
    mutex_set(
        &BORDER_COLOR_FOCUSED,
        Color::get_color(rd.gui_get_style(
            GuiControl::BUTTON,
            GuiControlProperty::BORDER_COLOR_FOCUSED as i32,
        )),
    );
    mutex_set(
        &BASE_COLOR_FOCUSED,
        Color::get_color(rd.gui_get_style(
            GuiControl::BUTTON,
            GuiControlProperty::BASE_COLOR_FOCUSED as i32,
        )),
    );
    mutex_set(
        &BORDER_COLOR_PRESSED,
        Color::get_color(rd.gui_get_style(
            GuiControl::BUTTON,
            GuiControlProperty::BORDER_COLOR_PRESSED as i32,
        )),
    );
    mutex_set(
        &BASE_COLOR_PRESSED,
        Color::get_color(rd.gui_get_style(
            GuiControl::BUTTON,
            GuiControlProperty::BASE_COLOR_PRESSED as i32,
        )),
    );
    mutex_set(
        &BORDER_WIDTH,
        rd.gui_get_style(GuiControl::BUTTON, GuiControlProperty::BORDER_WIDTH as i32),
    );

    mutex_set(
        &BORDER_COLOR_DISABLED,
        Color::get_color(rd.gui_get_style(
                GuiControl::BUTTON, 
                GuiControlProperty::BORDER_COLOR_DISABLED as i32
        )),
    );

    mutex_set(
        &BASE_COLOR_DISABLED,
        Color::get_color(rd.gui_get_style(
                GuiControl::BUTTON, 
                GuiControlProperty::BASE_COLOR_DISABLED as i32
        )),
    );
    mutex_set(
        &TEXT_COLOR_DISABLED,
        Color::get_color(rd.gui_get_style(
                GuiControl::BUTTON, 
                GuiControlProperty::TEXT_COLOR_DISABLED as i32
        )),
    );
}
