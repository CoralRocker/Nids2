use std::fs;
use std::io::prelude::*;
use raylib::prelude::*;
use serde::{Serialize, Deserialize};
use toml;
use std::collections::HashMap;
use lazy_static;
use std::sync::{atomic, Arc, Mutex};
use std::mem::drop;

#[derive(Serialize, Deserialize, Debug)]
pub struct ObjectConfig { 
    pub name: String,
    pub id: i32,
    pub dim: (i32, i32),
    pub sides: i32,
    pub img_per_side: i32,
    pub category: String,
    pub image_speed: Option<i32>,
}

lazy_static::lazy_static! {
    pub static ref LOADED_TEXTURES: Mutex<HashMap<i32, Arc<(Texture2D, ObjectConfig)>>> = Mutex::new(HashMap::new());
    static ref INITIALIZED: atomic::AtomicBool = atomic::AtomicBool::new(false);
}

pub fn init(rl: &mut RaylibHandle, rt: &RaylibThread) {
    if INITIALIZED.load(atomic::Ordering::Relaxed) {
        return;
    }

    for entry in fs::read_dir("obj/").unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        print!("Attempting load {}... ", path.to_str().unwrap());
        let imgpath = path.join("spr.png");
        let img = Image::load_image(imgpath.to_str().expect("Unable to convert image path to string!")).expect("Unable to load image!");
               
        let mut objconf = fs::File::open(path.join("obj.toml").to_str().unwrap()).unwrap();
        let mut confstr = String::new();
        let _ = objconf.read_to_string(&mut confstr).unwrap();
        
        let obj: ObjectConfig = toml::from_str(confstr.as_str()).expect("Unable to parse TOML Object configuration!");

        LOADED_TEXTURES.lock().expect("Unable to lock LOADED_TEXTURES mutex!").insert(obj.id, Arc::new((rl.load_texture_from_image(rt, &img).expect("Unable to laod texture from loaded imag!"), obj)));
        print!("DONE\n");

        // Clean up
        drop(img);
    }
    
    INITIALIZED.store(true, atomic::Ordering::Relaxed);
}

pub fn destroy() {
    if INITIALIZED.load(atomic::Ordering::Relaxed) {
        return;
    }
    for item in LOADED_TEXTURES.lock().expect("Unable to lock LOADED_TEXTURES mutex!").iter_mut(){
        let item = item.1;
        while Arc::strong_count(&item) > 1 {
            unsafe { Arc::decrement_strong_count(&item); }
        }
    }
    LOADED_TEXTURES.lock().expect("Unable to lock LOADED_TEXTURES mutex!").clear();

    INITIALIZED.store(false, atomic::Ordering::Relaxed);
    

}
