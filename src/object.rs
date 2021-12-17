#![allow(dead_code, unused_imports, unused_variables)]

use raylib::prelude::*;
use crate::game::*;
use std::sync::{Arc, Mutex};

#[derive(Copy, Clone)]
pub struct Position {x: i32, y: i32}

impl From<Position> for raylib::ffi::Vector2 {
    fn from(pos: Position) -> Self {
        raylib::ffi::Vector2 {
            x: pos.x as f32,
            y: pos.y as f32,
        }
    }
}
impl Position {
    pub fn new(x: i32, y: i32) -> Self {
        Position {
            x, y,
        }
    }
}

pub trait Object {
    fn draw(&self, rl: &mut RaylibDrawHandle);
    fn do_step(&mut self, frame_no: i32);
}

pub struct GenericObject {
    obj_id: i32,
    id: i32,
    pos: Position,
    side: i32,
    side_index: i32,
    object_data: Arc<(Texture2D, ObjectConfig)>,
    side_shift_speed: i32,
}

impl Object for GenericObject {
    fn draw(&self, rl: &mut RaylibDrawHandle) {
        let tex = &self.object_data.0;
        let obj = &self.object_data.1;
        let spr_rect = Rectangle { x: (obj.dim.0 * self.side_index) as f32, y: (obj.dim.1 * self.side) as f32, width: obj.dim.0 as f32, height: obj.dim.1 as f32 }; 
        
        rl.draw_texture_rec(tex, spr_rect, self.pos, Color::WHITE);
         
    }

    fn do_step(&mut self, frame_no: i32) {
        if self.side_shift_speed != 0 && frame_no % self.side_shift_speed == 0 {
            self.inc_index();
        }
    }
}

impl GenericObject {
    pub fn new(id: i32, obj_type: i32, pos: Option<Position>) -> Self {
        let data = Arc::clone(&LOADED_TEXTURES.lock().expect("Unable to lock LOADED_TEXTURES mutex!").get(&obj_type).expect("Bad object type ID!"));
        Self {
            obj_id : obj_type,
            id,
            pos : pos.unwrap_or(Position{x:0, y:0}),
            side : 0,
            side_index : 0,
            object_data : Arc::clone(&data),
            side_shift_speed : data.1.image_speed.unwrap_or(0),
        }
    }
    
    pub fn set_side(&mut self, side: u32) {
        self.side = side as i32;
        self.side_index = 0;
        if self.side >= self.object_data.1.sides {
            panic!("Attempting to set side greater than maximum!");
        }
    }
    pub fn set_index(&mut self, index: u32) {
        self.side_index = index as i32;
        if self.side_index >= self.object_data.1.img_per_side {
            panic!("Attempting to set image index greater than maximum!");
        }
    }

    pub fn set_shift(&mut self, shift: i32) -> &mut Self {
        self.side_shift_speed = shift;
        return self;
    }
    
    pub fn inc_index(&mut self) -> &mut Self {
        self.side_index += 1;
        if self.side_index >= self.object_data.1.img_per_side {
            self.side_index = 0;
        }
        return self;
    }
    pub fn dec_index(&mut self) -> &mut Self {
        self.side_index -= 1;
        if self.side_index < 0 {
            self.side_index = self.object_data.1.img_per_side - 1;
        }
        return self;
    }
    
    pub fn inc_side(&mut self) -> &mut Self {
        self.side += 1;
        if self.side >= self.object_data.1.sides {
            self.side = 0;
        }
        return self;
    }
    pub fn dec_side(&mut self) -> &mut Self {
        self.side -= 1;
        if self.side < 0 {
            self.side = self.object_data.1.sides - 1;
        }
        return self;    
    }

}
