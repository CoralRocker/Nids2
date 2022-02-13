use crate::object::*;
use crate::save::*;
use raylib::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Clone, PartialEq)]
pub enum Direction {
    Right,
    Up,
    Left,
    Down,
}

pub fn dir_to_u32(dir: &Direction) -> u32 {
    match dir {
        Direction::Right => 0,
        Direction::Up => 1,
        Direction::Left => 2,
        Direction::Down => 3,
    }
}

pub fn dir_to_i32(dir: &Direction) -> i32 {
    dir_to_u32(dir) as i32
}

/// Type alias because me is lazy
type GenObj = Rc<RefCell<GenericObject>>;


/** Main player for the game. Has additional methods compared to basic objects to allow for control
 * of the game state.
 */
pub struct Naomi {
    pub base: GenericObject,
    pub moving: bool,
    pub ghost: bool,
    pub dir: Direction,
    pub scrw: i32,
    pub scrh: i32,
    pub select_obj_type: i32,
    pub select_obj: Option<GenObj>,
    pub colormod: color::Color,
}

impl Object for Naomi {
    /** Currently the same as the GenericObject draw method.
     */
    fn draw(&self, rl: &mut RaylibTextureMode<RaylibDrawHandle>, debug: bool) {
        self.base.draw(rl, debug);
        // if let Some(obj) = &self.select_obj {
        //     obj.borrow_mut().draw(rl);
        // }
    }

    /** Executes the GenericObject's do_step method and then executes movement logic for the
     * player.
     */
    fn do_step(&mut self, frame_no: i32) {
        self.base.do_step(frame_no);

        if self.moving {
            match self.dir {
                Direction::Right => {
                    self.base.move_x(1, self.scrw);
                    if let Some(obj) = &self.select_obj {
                        obj.borrow_mut().move_x_unchecked(1);
                    }
                }
                Direction::Up => {
                    self.base.move_y(-1, self.scrh);
                    if let Some(obj) = &self.select_obj {
                        obj.borrow_mut().move_y_unchecked(-1);
                    }
                }
                Direction::Left => {
                    self.base.move_x(-1, self.scrw);
                    if let Some(obj) = &self.select_obj {
                        obj.borrow_mut().move_x_unchecked(-1);
                    }
                }
                Direction::Down => {
                    self.base.move_y(1, self.scrh);
                    if let Some(obj) = &self.select_obj {
                        obj.borrow_mut().move_y_unchecked(1);
                    }
                }
            }
            if self.base.pos_aligned(16) {
                self.moving = false;
                self.base.set_shift(0);
            }
        }
        self.base.update_depth();
    }

    fn collide(&self, other: Option<&Rectangle>) -> bool {
        self.base.collide(other)
    }

    fn get_b_box(&self) -> Option<&Rectangle> {
        self.base.get_b_box()
    }

    fn get_depth(&self) -> i32 {
        self.base.get_depth()
    }

    fn get_id(&self) -> i32 {
        self.base.get_id()
    }

    fn get_obj_rect(&self) -> Rectangle {
        self.base.get_obj_rect()
    }

    fn get_collision_rect(&self) -> Rectangle {
        self.base.get_collision_rect()
    }
}

impl Naomi {
    pub fn new(pos: Position, id: i32, scrw: i32, scrh: i32) -> Self {
        let mut result = Self {
            base: GenericObject::new(1, id, Some(pos)),
            moving: false,
            ghost: false,
            dir: Direction::Right,
            scrw,
            scrh,
            select_obj_type: 0,
            select_obj: None,
            colormod: Color::WHITE,
        };
        result.base.set_shift(0);
        result
    }
    pub fn get_scrw(&self) -> i32 {
        self.scrw
    }
    pub fn get_scrh(&self) -> i32 {
        self.scrh
    }
    pub fn is_spot_free(
        &self,
        spot: Rectangle,
        objects: &[GenObj],
    ) -> bool {
        if self.ghost { return true; }

        for obj in objects.iter() {
            if let Some(o) = &self.select_obj {
                if *o.borrow() == *obj.borrow(){
                    continue;
                }
            }
            if obj.borrow().collide(Some(&spot)){
                return false;
            }
        }

        true
    }
    
    /// Takes in an object and sets its position correctly.
    pub fn grab_object(
        &self,
        obj: GenObj
    ) {
        let mut obj = obj.borrow_mut();
        let obj_position: Position = match self.dir {
            Direction::Right => self.base.pos.offset(32, 16 - obj.height() / 2),
            Direction::Up => self.base.pos.offset(16 - obj.width() / 2, -obj.height()),
            Direction::Left => self.base.pos.offset(-obj.width(), 16 - obj.height() / 2),
            Direction::Down => self.base.pos.offset(16 - obj.width() / 2, 48),
        };

        obj.pos = obj_position;
    }

    /** Check for recent input from the user
     */
    pub fn handle_input(
        &mut self,
        rl: &mut RaylibHandle,
        next_id: &mut i32,
        objects: &mut Vec<GenObj>,
    ) -> Option<GenObj> {
        if self.moving {
            return None;
        }

        let old_dir = self.dir.clone();
        let is_change_dir = rl.is_key_down(KeyboardKey::KEY_LEFT_SHIFT)
            || rl.is_key_down(KeyboardKey::KEY_RIGHT_SHIFT);
        if rl.is_key_down(KeyboardKey::KEY_RIGHT) {
            self.dir = Direction::Right;
            if !is_change_dir
                && self.is_spot_free(
                    self.base
                        .b_box
                        .map(|r| {
                            rrect(
                                r.x + 16. + self.base.pos.x as f32,
                                r.y + self.base.pos.y as f32,
                                r.width,
                                r.height,
                            )
                        })
                        .unwrap(),
                    objects,
                )
            {
                self.moving = true;
            }
        } else if rl.is_key_down(KeyboardKey::KEY_LEFT) {
            self.dir = Direction::Left;
            if !is_change_dir
                && self.is_spot_free(
                    self.base
                        .b_box
                        .map(|r| {
                            rrect(
                                r.x - 16. + self.base.pos.x as f32,
                                r.y + self.base.pos.y as f32,
                                r.width,
                                r.height,
                            )
                        })
                        .unwrap(),
                    objects,
                )
            {
                self.moving = true;
            }
        } else if rl.is_key_down(KeyboardKey::KEY_DOWN) {
            self.dir = Direction::Down;
            if !is_change_dir
                && self.is_spot_free(
                    self.base
                        .b_box
                        .map(|r| {
                            rrect(
                                r.x + self.base.pos.x as f32,
                                r.y + 16. + self.base.pos.y as f32,
                                r.width,
                                r.height,
                            )
                        })
                        .unwrap(),
                    objects,
                )
            {
                self.moving = true;
            }
        } else if rl.is_key_down(KeyboardKey::KEY_UP) {
            self.dir = Direction::Up;
            if !is_change_dir
                && self.is_spot_free(
                    self.base
                        .b_box
                        .map(|r| {
                            rrect(
                                r.x + self.base.pos.x as f32,
                                r.y - 16. + self.base.pos.y as f32,
                                r.width,
                                r.height,
                            )
                        })
                        .unwrap(),
                    objects,
                )
            {
                self.moving = true;
            }
        }

        if let Some(o) = &self.select_obj {
            match rl.get_key_pressed() {
                Some(KeyboardKey::KEY_Q) => {
                    o.borrow_mut().dec_index();
                }
                Some(KeyboardKey::KEY_E) => {
                    o.borrow_mut().inc_index();
                }
                Some(KeyboardKey::KEY_W) => {
                    o.borrow_mut().depthmod += 1;
                }
                Some(KeyboardKey::KEY_S) => {
                    o.borrow_mut().depthmod -= 1;
                }
                Some(KeyboardKey::KEY_A) => {
                    o.borrow_mut().dec_side();
                }
                Some(KeyboardKey::KEY_D) => {
                    o.borrow_mut().inc_side();
                }
                _ => (),
            };

            if rl.is_key_pressed(KeyboardKey::KEY_ESCAPE) {
                let obj = o.clone();
                self.select_obj = None;
                return Some(obj);
            }
            
            if old_dir != self.dir {
                self.grab_object(o.clone());
            }

        }

        if old_dir != self.dir {
            self.base.set_side(dir_to_u32(&self.dir));
        }

        if self.moving {
            let index = self.base.get_index();
            if index == 1 || index == 2 {
                self.base.set_index(2);
            }
            self.base.set_shift(self.base.get_default_shift());
        }
        
        if rl.is_key_pressed(KeyboardKey::KEY_G) {
            self.ghost = !self.ghost;
            if self.ghost {
                self.base.colormod = self.base.colormod.fade(0.75);
            }else{
                self.base.colormod = Color::WHITE;
            }
        }

        if self.select_obj_type != 0 && rl.is_key_pressed(KeyboardKey::KEY_SPACE) {
            if self.select_obj.is_some() {
                self.select_obj = None;
            } else {
                let mut obj = GenericObject::new(*next_id, self.select_obj_type, None);
                *next_id += 1;
                obj.colormod = self.colormod; // Set object colormod to our selected color

                let obj = Rc::new(RefCell::new(obj));

                self.grab_object(obj.clone());
                self.select_obj = Some(obj.clone());

                objects.push(obj);
            }
        }

        None
    }
}

/* SAVE IMPLEMENTATIONS */
impl Saveable<Self> for Direction {
    fn to_bytes(&self) -> Vec<u8> {
        dir_to_i32(self).to_bytes()
    }
    fn from_bytes(bytes: &[u8]) -> Result<SaveInfo<Self>, Box<dyn std::error::Error>> {
        match i32::from_bytes(bytes)?.0 {
            0 => Ok(SaveInfo(Direction::Right, 4)),
            1 => Ok(SaveInfo(Direction::Up, 4)),
            2 => Ok(SaveInfo(Direction::Left, 4)),
            3 => Ok(SaveInfo(Direction::Down, 4)),
            _ => Err("invalid direction read".into()),
        }
    }
}

impl Saveable<Self> for Naomi {
    fn to_bytes(&self) -> Vec<u8> {
        let mut result = self.base.to_bytes();
        result.extend(self.moving.to_bytes());
        result.extend(self.ghost.to_bytes());
        result.extend(self.dir.to_bytes());
        result.extend(self.select_obj_type.to_bytes());
        result.extend(self.select_obj.to_bytes());
        result.extend(self.get_scrw().to_bytes());
        result.extend(self.get_scrh().to_bytes());
        result.extend(self.colormod.to_bytes());
        result
    }
    fn from_bytes(bytes: &[u8]) -> Result<SaveInfo<Self>, Box<dyn std::error::Error>> {
        let mut bytes_read = 0;
        let base = {
            let base = GenericObject::from_bytes(&bytes[bytes_read..])?;
            bytes_read += base.1;
            base.0
        };
        let moving = {
            let base = bool::from_bytes(&bytes[bytes_read..])?;
            bytes_read += base.1;
            base.0
        };
        let ghost = {
            let base = bool::from_bytes(&bytes[bytes_read..])?;
            bytes_read += base.1;
            base.0
        };
        let dir = {
            let base = Direction::from_bytes(&bytes[bytes_read..])?;
            bytes_read += base.1;
            base.0
        };
        let select_obj_type = {
            let base = i32::from_bytes(&bytes[bytes_read..])?;
            bytes_read += base.1;
            base.0
        };
        let select_obj = {
            let base = Option::<GenObj>::from_bytes(&bytes[bytes_read..])?;
            bytes_read += base.1;
            base.0
        };
        let scrw = {
            let base = i32::from_bytes(&bytes[bytes_read..])?;
            bytes_read += base.1;
            base.0
        };
        let scrh = {
            let base = i32::from_bytes(&bytes[bytes_read..])?;
            bytes_read += base.1;
            base.0
        };
        let colormod = {
            let base = Color::from_bytes(&bytes[bytes_read..])?;
            bytes_read += base.1;
            base.0
        };
        
        let mut naomi = Naomi {
            base,
            moving,
            ghost,
            dir,
            scrw,
            scrh,
            select_obj_type,
            select_obj,
            colormod,
        };
        naomi.base.set_shift(0);
        Ok(SaveInfo(naomi, bytes_read))
    }
}
