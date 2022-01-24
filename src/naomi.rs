use crate::object::*;
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

/** Main player for the game. Has additional methods compared to basic objects to allow for control
 * of the game state.
 */
pub struct Naomi {
    base: GenericObject,
    moving: bool,
    dir: Direction,
    scrw: i32,
    scrh: i32,
    pub select_obj_type: i32,
    select_obj: Option<Rc<RefCell<GenericObject>>>,
}

impl Object for Naomi {
    /** Currently the same as the GenericObject draw method.
     */
    fn draw(&self, rl: &mut RaylibDrawHandle) {
        self.base.draw(rl);
        if let Some(obj) = &self.select_obj {
            obj.borrow_mut().draw(rl);
        }
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
            self.base.update_depth();
        }
    }

    fn collide(&self, other: Option<&Vec<(i32, i32)>>) -> bool {
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
}

impl Naomi {
    pub fn new(pos: Position, id: i32, scrw: i32, scrh: i32) -> Self {
        Self {
            base: GenericObject::new(1, id, Some(pos)),
            moving: false,
            dir: Direction::Right,
            scrw,
            scrh,
            select_obj_type: 3,
            select_obj: None,
        }
    }

    /** Check for recent input from the user
     */
    pub fn handle_input(
        &mut self,
        rl: &mut RaylibHandle,
        next_id: i32,
    ) -> Option<Rc<RefCell<GenericObject>>> {
        if self.moving {
            return None;
        }

        let old_dir = self.dir.clone();

        if rl.is_key_down(KeyboardKey::KEY_RIGHT) {
            self.moving = true;
            self.dir = Direction::Right;
        } else if rl.is_key_down(KeyboardKey::KEY_LEFT) {
            self.moving = true;
            self.dir = Direction::Left;
        } else if rl.is_key_down(KeyboardKey::KEY_DOWN) {
            self.moving = true;
            self.dir = Direction::Down;
        } else if rl.is_key_down(KeyboardKey::KEY_UP) {
            self.moving = true;
            self.dir = Direction::Up;
        }

        if let Some(obj) = &self.select_obj {
            let mut obj = obj.borrow_mut();
            if old_dir != self.dir {
                let obj_position: Position = match self.dir {
                    Direction::Right => self.base.pos.offset(32, 16 - obj.height() / 2),
                    Direction::Up => self.base.pos.offset(16 - obj.width() / 2, -obj.height()),
                    Direction::Left => self.base.pos.offset(-obj.width(), 16 - obj.height() / 2),
                    Direction::Down => self.base.pos.offset(16 - obj.width() / 2, 48),
                };
                obj.pos = obj_position;
            }
        }

        if self.moving {
            let index = self.base.get_index();
            self.base.set_side(dir_to_u32(&self.dir));
            if index == 1 || index == 2 {
                self.base.set_index(2);
            }
            self.base.set_shift(self.base.get_default_shift());
        }

        if self.select_obj_type != 0 && rl.is_key_pressed(KeyboardKey::KEY_SPACE) {
            if let Some(obj) = &self.select_obj {
                let obj = obj.clone();
                self.select_obj = None;
                return Some(obj);
            } else {
                let mut obj = GenericObject::new(next_id, self.select_obj_type, None);
                let obj_position: Position = match self.dir {
                    Direction::Right => self.base.pos.offset(32, 16 - obj.height() / 2),
                    Direction::Up => self.base.pos.offset(16 - obj.width() / 2, -obj.height()),
                    Direction::Left => self.base.pos.offset(-obj.width(), 16 - obj.height() / 2),
                    Direction::Down => self.base.pos.offset(16 - obj.width() / 2, 48),
                };

                obj.pos = obj_position;

                self.select_obj = Some(Rc::new(RefCell::new(obj)));
            }
        }

        None
    }
}
