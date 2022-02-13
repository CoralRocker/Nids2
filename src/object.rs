#![allow(dead_code, unused_imports, unused_variables)]

use crate::game::*;
use crate::save::*;
use raylib::prelude::*;
use std::fmt;
use std::sync::{Arc, Mutex};

/** Simple struct to hold the position in screenspace of an object
 */
#[derive(Copy, Clone)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

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
        Position { x, y }
    }

    pub fn offset(&self, x_off: i32, y_off: i32) -> Self {
        Self::new(self.x + x_off, self.y + y_off)
    }

    pub fn to_rect(&self, width: i32, height: i32) -> Rectangle {
        rrect(self.x, self.y, width, height)
    }
}

impl Default for Position {
    fn default() -> Self {
        Self::new(0, 0)
    }
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

/** This trait defines what methods all objects are expected to implement.
 */
pub trait Object {
    fn draw(&self, rl: &mut RaylibTextureMode<RaylibDrawHandle>, debug: bool);
    fn do_step(&mut self, frame_no: i32);
    fn collide(&self, other: Option<&Rectangle>) -> bool;
    fn get_b_box(&self) -> Option<&Rectangle>;
    fn get_obj_rect(&self) -> Rectangle;
    fn get_depth(&self) -> i32;
    fn get_id(&self) -> i32;
    fn get_collision_rect(&self) -> Rectangle;
}


/** The base for all objects. Grabs data from the LOADED_TEXTURES static variable and uses it to initialize an object of a known type.
 */
pub struct GenericObject {
    pub obj_id: i32,
    id: i32,
    pub pos: Position,
    pub depth: i32,
    pub side: i32,
    pub side_index: i32,
    pub object_data: Arc<(Texture2D, ObjectConfig)>,
    pub side_shift_speed: i32,
    pub b_box: Option<Rectangle>,
    pub depthmod: i32,
    pub colormod: Color,
}


impl Object for GenericObject {
    /** Simply draw the current sprite on the screen at the object's position. No color tinting or anything at all
     */
    fn draw(&self, rl: &mut RaylibTextureMode<RaylibDrawHandle>, debug: bool) {
        let tex = &self.object_data.0;
        let obj = &self.object_data.1;
        let spr_rect = Rectangle {
            x: (obj.dim.0 * self.side_index) as f32,
            y: (obj.dim.1 * self.side) as f32,
            width: obj.dim.0 as f32,
            height: obj.dim.1 as f32,
        };

        rl.draw_texture_rec(tex, spr_rect, self.pos, self.colormod);
        if debug {
            rl.draw_rectangle_lines_ex(
                rrect(self.pos.x, self.pos.y, spr_rect.width, spr_rect.height),
                1,
                Color::BLACK,
            );
            rl.draw_line(
                self.pos.x,
                self.get_depth(),
                self.pos.x + spr_rect.width as i32,
                self.get_depth(),
                Color::RED,
            );
        }
    }

    /** Change the sprite if the object supports that.
     */
    fn do_step(&mut self, frame_no: i32) {
        if self.side_shift_speed != 0 && frame_no % self.side_shift_speed == 0 {
            self.inc_index();
        }
        self.depth = self.pos.y;
    }

    fn collide(&self, other: Option<&Rectangle>) -> bool {
        if self.b_box.is_none() {
            return false;
        }
        if let Some(bbox) = other {
            return bbox.check_collision_recs(
                &self
                    .b_box
                    .map(|r| {
                        rrect(
                            r.x + self.pos.x as f32,
                            r.y + self.pos.y as f32,
                            r.width,
                            r.height,
                        )
                    })
                    .unwrap(),
            );
        }
        false
    }

    fn get_b_box(&self) -> Option<&Rectangle> {
        self.b_box.as_ref()
    }

    fn get_depth(&self) -> i32 {
        self.depth + self.depthmod
    }

    fn get_id(&self) -> i32 {
        self.id
    }

    fn get_obj_rect(&self) -> Rectangle {
        rrect(
            self.pos.x,
            self.pos.y,
            self.object_data.1.dim.0,
            self.object_data.1.dim.1,
        )
    }

    fn get_collision_rect(&self) -> Rectangle {
        if let Some(rec) = self.b_box {
            rrect(
                rec.x + self.pos.x as f32,
                rec.y + self.pos.y as f32,
                rec.width,
                rec.height,
            )
        } else {
            self.get_obj_rect()
        }
    }
}

impl PartialEq for GenericObject {
    fn eq(&self, other: &Self) -> bool {
        self.get_id() == other.get_id()
    }
}

impl fmt::Display for GenericObject {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} id {}: {}, depth {}",
            self.object_data.1.name,
            self.id,
            self.pos,
            self.get_depth()
        )
    }
}

impl GenericObject {
    /** Create a new instance of an object of given type with a unique ID. The position the object is created in is either given by the user or is (0,0).
     */
    pub fn new(id: i32, obj_type: i32, pos: Option<Position>) -> Self {
        let data = Arc::clone(
            LOADED_TEXTURES
                .lock()
                .expect("Unable to lock LOADED_TEXTURES mutex!")
                .get(&obj_type)
                .expect("Bad object type ID!"),
        );
        Self {
            obj_id: obj_type,
            id,
            pos: pos.unwrap_or_default(),
            depth: pos.unwrap_or_default().y,
            side: 0,
            side_index: 0,
            object_data: Arc::clone(&data),
            side_shift_speed: data.1.image_speed.unwrap_or(0),
            b_box: data
                .1
                .default_b_box
                .as_ref()
                .map(|v| rrect(v.0, v.1, v.2, v.3)),
            depthmod: 0,
            colormod: Color::WHITE,
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
        self
    }

    /** Set which sprite subimage to display
     */
    pub fn set_index(&mut self, index: u32) -> &mut Self {
        self.side_index = index as i32;
        if self.side_index >= self.object_data.1.img_per_side {
            panic!("Attempting to set image index greater than maximum!");
        }
        self
    }

    /** Set the frequency with which the sprite subimage is incremented. Lower values increment faster and 0 does not increment.
     */
    pub fn set_shift(&mut self, shift: i32) -> &mut Self {
        self.side_shift_speed = shift;
        self
    }

    /** Increment the sprite subimage, wrapping back to 0 if it reaches the last subimage.
     */
    pub fn inc_index(&mut self) -> &mut Self {
        self.side_index += 1;
        if self.side_index >= self.object_data.1.img_per_side {
            self.side_index = 0;
        }
        self
    }

    /** Decrement the sprite subimage, wrapping to the last subimage if it attempts to go before the fist subimage.
     */
    pub fn dec_index(&mut self) -> &mut Self {
        self.side_index -= 1;
        if self.side_index < 0 {
            self.side_index = self.object_data.1.img_per_side - 1;
        }
        self
    }

    /** Increment the current sprite side, wrapping back around if it reaches the last side.
     */
    pub fn inc_side(&mut self) -> &mut Self {
        self.side += 1;
        if self.side >= self.object_data.1.sides {
            self.side = 0;
        }
        self
    }

    /** Decrement the current sprite side, wrapping to the end if it reaches the zeroth side.
     */
    pub fn dec_side(&mut self) -> &mut Self {
        self.side -= 1;
        if self.side < 0 {
            self.side = self.object_data.1.sides - 1;
        }
        self
    }

    /** Move the object by a given amount in the x direction, not reaching the max value. Has range [0, max)
     */
    pub fn move_x(&mut self, amt: i32, max: i32) -> &mut Self {
        self.pos.x += amt;
        if self.pos.x >= max {
            self.pos.x = max - 1;
        } else if self.pos.x < 0 {
            self.pos.x = 0;
        }
        self
    }

    pub fn move_x_unchecked(&mut self, amt: i32) -> &mut Self {
        self.pos.x += amt;
        self
    }

    /** Move the object by a given amount in the y direction, not reaching the max value. Has range [0, max)
     */
    pub fn move_y(&mut self, amt: i32, max: i32) -> &mut Self {
        self.pos.y += amt;
        if self.pos.y >= max {
            self.pos.y = max - 1;
        } else if self.pos.y < 0 {
            self.pos.y = 0;
        }
        self
    }

    pub fn move_y_unchecked(&mut self, amt: i32) -> &mut Self {
        self.pos.y += amt;
        self
    }

    /** Check if the object's position is aligned with a given grid.
     */
    pub fn pos_aligned(&self, grid: i32) -> bool {
        self.pos.x % grid == 0 && self.pos.y % grid == 0
    }

    /** Return the name of the object's type.
     */
    pub fn get_name(&self) -> String {
        self.object_data.1.name.clone()
    }

    pub fn get_side(&self) -> i32 {
        self.side
    }
    pub fn get_index(&self) -> i32 {
        self.side_index
    }
    pub fn get_default_shift(&self) -> i32 {
        self.object_data.1.image_speed.unwrap_or(0)
    }

    pub fn update_depth(&mut self) -> &mut Self {
        self.depth = self.pos.y;
        self
    }

    pub fn width(&self) -> i32 {
        self.object_data.1.dim.0
    }
    pub fn height(&self) -> i32 {
        self.object_data.1.dim.1
    }
}

/* SAVE IMPLEMENTATIONS */
impl Saveable<Self> for Position {
    fn to_bytes(&self) -> Vec<u8> {
        let mut result = self.x.to_bytes();
        result.extend(self.y.to_bytes().iter());
        result
    }

    fn from_bytes(bytes: &[u8]) -> Result<SaveInfo<Self>, Box<dyn std::error::Error>> {
        let result = Position {
            x: i32::from_bytes(&bytes[0..4])?.0,
            y: i32::from_bytes(&bytes[4..8])?.0,
        };
        Ok(SaveInfo(result, 8))
    }
}

impl Saveable<Self> for GenericObject {
    fn to_bytes(&self) -> Vec<u8> {
        let mut result = self.get_id().to_bytes();
        result.extend(self.obj_id.to_bytes());
        result.extend(self.pos.to_bytes());
        result.extend(self.side_index.to_bytes());
        result.extend(self.side.to_bytes());
        result.extend(self.colormod.to_bytes());
        result.extend(self.depthmod.to_bytes());
        result
    }

    fn from_bytes(bytes: &[u8]) -> Result<SaveInfo<Self>, Box<dyn std::error::Error>> {
        let id = i32::from_bytes(&bytes[0..4])?;
        let obj_id = i32::from_bytes(&bytes[4..8])?;
        let pos = Position::from_bytes(&bytes[8..16])?;
        let side_index = i32::from_bytes(&bytes[16..20])?;
        let side = i32::from_bytes(&bytes[20..24])?;
        let colormod = Color::from_bytes(&bytes[24..28])?;
        let depthmod = i32::from_bytes(&bytes[28..32])?;
        let mut obj = GenericObject::new(id.0, obj_id.0, Some(pos.0));
        obj.side_index = side_index.0;
        obj.side = side.0;
        obj.colormod = colormod.0;
        obj.depthmod = depthmod.0;
        Ok(SaveInfo(obj, 32))
    }
}
