use crate::object;
use raylib::prelude::*;
use std::cell::{RefCell, RefMut};

pub struct Naomi {
    base: object::GenericObject,
}

impl object::Object for Naomi {
    fn draw(&self, rl: &mut RaylibDrawHandle) {
        self.base.draw(rl);
    }

    fn do_step(&mut self, frame_no: i32){
        self.base.do_step(frame_no);
    }
}

impl Naomi {
    pub fn new(pos: object::Position, id: i32) -> Self {
        Self {
            base: object::GenericObject::new(1, id, Some(pos)),
        }
    }

    pub fn handle_input(&mut self, rl: &mut RaylibHandle) {
        match rl.get_key_pressed() {
            Some(KeyboardKey::KEY_RIGHT) => self.base.set_side(0),
            Some(KeyboardKey::KEY_LEFT) => self.base.set_side(2),
            Some(KeyboardKey::KEY_DOWN) => self.base.set_side(3),
            Some(KeyboardKey::KEY_UP) => self.base.set_side(1),
            _ => () ,
        }
    }
}
