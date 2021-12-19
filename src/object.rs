#![allow(dead_code, unused_imports, unused_variables)]

use raylib::prelude::*;
use crate::game::*;
use std::sync::{Arc, Mutex};

/** Simple struct to hold the position in screenspace of an object
 */
#[derive(Copy, Clone)]
pub struct Position {x: i32, y: i32}

/** So that we don't have to perform conversions when drawing using position instead of Vector2. 
 */
impl From<Position> for raylib::ffi::Vector2 {
    fn from(pos: Position) -> Self {
        raylib::ffi::Vector2 {
            x: pos.x as f32,
            y: pos.y as f32,
        }
    }
}
impl Position {
    /** Create a new position with the given x and y coords.
     */
    pub fn new(x: i32, y: i32) -> Self {
        Position {
            x, y,
        }
    }
}

/** This trait defines what methods all objects are expected to implement.
 */
pub trait Object {
    fn draw(&self, rl: &mut RaylibDrawHandle);
    fn do_step(&mut self, frame_no: i32);
    fn collide(&self, other: Option<&Vec<(i32, i32)>>) -> bool;
    fn get_b_box(&self) -> Option<&Vec<(i32, i32)>>;
    fn get_depth(&self) -> i32;
    fn get_id(&self) -> i32;
}

/** The base for all objects. Grabs data from the LOADED_TEXTURES static variable and uses it to initialize an object of a known type.
 */
pub struct GenericObject {
    obj_id: i32,
    id: i32,
    pub pos: Position,
    pub depth: i32,
    pub side: i32,
    pub side_index: i32,
    object_data: Arc<(Texture2D, ObjectConfig)>,
    pub side_shift_speed: i32,
    pub b_box: Option<Vec<(i32, i32)>>,
}

impl Object for GenericObject {
    /** Simply draw the current sprite on the screen at the object's position. No color tinting or anything at all
     */
    fn draw(&self, rl: &mut RaylibDrawHandle) {
        let tex = &self.object_data.0;
        let obj = &self.object_data.1;
        let spr_rect = Rectangle { x: (obj.dim.0 * self.side_index) as f32, y: (obj.dim.1 * self.side) as f32, width: obj.dim.0 as f32, height: obj.dim.1 as f32 }; 
        
        rl.draw_texture_rec(tex, spr_rect, self.pos, Color::WHITE);
         
    }
    
    /** Change the sprite if the object supports that.
     */
    fn do_step(&mut self, frame_no: i32) {
        if self.side_shift_speed != 0 && frame_no % self.side_shift_speed == 0 {
            self.inc_index();
        }
    }

    fn collide(&self, other: Option<&Vec<(i32,i32)>>) -> bool {
        if other.is_none() { return false; }
        return true;
    }

    fn get_b_box(&self) -> Option<&Vec<(i32, i32)>> {
        return self.b_box.as_ref();
    }

    fn get_depth(&self) -> i32 { return self.depth; }
    
    fn get_id(&self) -> i32 { return self.id; }
}

impl PartialEq for GenericObject {
    fn eq(&self, other: &Self) -> bool {
        return self.get_id() == other.get_id();
    }
}

impl GenericObject {
    /** Create a new instance of an object of given type with a unique ID. The position the object is created in is either given by the user or is (0,0).
     */
    pub fn new(id: i32, obj_type: i32, pos: Option<Position>) -> Self {
        let data = Arc::clone(&LOADED_TEXTURES.lock().expect("Unable to lock LOADED_TEXTURES mutex!").get(&obj_type).expect("Bad object type ID!"));
        Self {
            obj_id : obj_type,
            id,
            pos : pos.unwrap_or(Position{x:0, y:0}),
            depth : pos.unwrap_or(Position::new(0, 0)).y,
            side : 0,
            side_index : 0,
            object_data : Arc::clone(&data),
            side_shift_speed : data.1.image_speed.unwrap_or(0),
            b_box :  match &data.1.default_b_box {
                Some(v) => Some(v.clone()),
                None => None,
            },
        }
    }
    
    /** Set which sprite side the object is using
     */
    pub fn set_side(&mut self, side: u32) -> &mut Self {
        self.side = side as i32;
        self.side_index = 0;
        if self.side >= self.object_data.1.sides {
            panic!("Attempting to set side greater than maximum!");
        }
        return self;
    }

    /** Set which sprite subimage to display
     */
    pub fn set_index(&mut self, index: u32) -> &mut Self {
        self.side_index = index as i32;
        if self.side_index >= self.object_data.1.img_per_side {
            panic!("Attempting to set image index greater than maximum!");
        }
        return self;
    }
    
    /** Set the frequency with which the sprite subimage is incremented. Lower values increment faster and 0 does not increment.
     */
    pub fn set_shift(&mut self, shift: i32) -> &mut Self {
        self.side_shift_speed = shift;
        return self;
    }
    
    /** Increment the sprite subimage, wrapping back to 0 if it reaches the last subimage.
     */
    pub fn inc_index(&mut self) -> &mut Self {
        self.side_index += 1;
        if self.side_index >= self.object_data.1.img_per_side {
            self.side_index = 0;
        }
        return self;
    }

    /** Decrement the sprite subimage, wrapping to the last subimage if it attempts to go before the fist subimage.
     */
    pub fn dec_index(&mut self) -> &mut Self {
        self.side_index -= 1;
        if self.side_index < 0 {
            self.side_index = self.object_data.1.img_per_side - 1;
        }
        return self;
    }
    
    /** Increment the current sprite side, wrapping back around if it reaches the last side.
     */
    pub fn inc_side(&mut self) -> &mut Self {
        self.side += 1;
        if self.side >= self.object_data.1.sides {
            self.side = 0;
        }
        return self;
    }
    
    /** Decrement the current sprite side, wrapping to the end if it reaches the zeroth side.
     */
    pub fn dec_side(&mut self) -> &mut Self {
        self.side -= 1;
        if self.side < 0 {
            self.side = self.object_data.1.sides - 1;
        }
        return self;    
    }
    
    /** Move the object by a given amount in the x direction, not reaching the max value. Has range [0, max)
     */
    pub fn move_x(&mut self, amt: i32, max: i32) -> &mut Self {
        self.pos.x += amt;
        if self.pos.x >= max {
            self.pos.x = max - 1;
        }else if self.pos.x < 0 {
            self.pos.x = 0;
        }
        return self;
    }

    /** Move the object by a given amount in the y direction, not reaching the max value. Has range [0, max)
     */
    pub fn move_y(&mut self, amt: i32, max: i32) -> &mut Self {
        self.pos.y += amt;
        if self.pos.y >= max {
            self.pos.y = max - 1;
        }else if self.pos.y < 0 {
            self.pos.y = 0;
        }
        return self;
    }
    
    /** Check if the object's position is aligned with a given grid.
     */
    pub fn pos_aligned(&self, grid: i32) -> bool {
        self.pos.x % grid == 0 && self.pos.y % grid == 0
    }
    
    /** Return the name of the object's type.
     */
    pub fn get_name(&self) -> String {
        return self.object_data.1.name.clone();
    }

    pub fn get_side(&self) -> i32 {
        return self.side;
    }
    pub fn get_index(&self) -> i32 {
        return self.side_index;
    }
    pub fn get_default_shift(&self) -> i32 {
        return self.object_data.1.image_speed.unwrap_or(0);
    }

    pub fn update_depth(&mut self) -> &mut Self {
        self.depth = self.pos.y;
        return self;
    }
}
