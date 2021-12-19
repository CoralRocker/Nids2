use crate::object;
use raylib::prelude::*;

pub enum Direction {
    Right,
    Up,
    Left,
    Down,
}

pub fn dir_to_u32(dir: &Direction) -> u32 {
    return match dir {
        Direction::Right => 0,
        Direction::Up => 1,
        Direction::Left => 2,
        Direction::Down => 3,
    };
}

/** Main player for the game. Has additional methods compared to basic objects to allow for control
 * of the game state.
 */
pub struct Naomi {
    base: object::GenericObject,
    moving: bool,
    dir: Direction,
    scrw: i32,
    scrh: i32,
}

impl object::Object for Naomi {
    /** Currently the same as the GenericObject draw method.
     */
    fn draw(&self, rl: &mut RaylibDrawHandle) {
        self.base.draw(rl);
    }
    
    /** Executes the GenericObject's do_step method and then executes movement logic for the
     * player.
     */
    fn do_step(&mut self, frame_no: i32){
        self.base.do_step(frame_no);
        if self.moving {
            
            match self.dir {
                Direction::Right => {
                    self.base.move_x(1, self.scrw);
                },
                Direction::Up => {
                    self.base.move_y(-1, self.scrh);
                },
                Direction::Left => {
                    self.base.move_x(-1, self.scrw);
                },
                Direction::Down => {
                    self.base.move_y(1, self.scrh);
                },
            }
            if self.base.pos_aligned(16) {
                self.moving = false;
                self.base.set_shift(0);
            }
            self.base.update_depth();
        }
    }

    fn collide(&self, other: Option<&Vec<(i32,i32)>>) -> bool {
        self.base.collide(other)
    }
    
    fn get_b_box(&self) -> Option<&Vec<(i32, i32)>> {
        self.base.get_b_box()
    }

    fn get_depth(&self) -> i32 { return self.base.get_depth(); }
    
    fn get_id(&self) -> i32 { return self.base.get_id(); }
}

impl Naomi {
    pub fn new(pos: object::Position, id: i32, scrw: i32, scrh: i32) -> Self {
        Self {
            base: object::GenericObject::new(1, id, Some(pos)),
            moving: false,
            dir: Direction::Right,
            scrw,
            scrh,
        }
    }
    
    /** Check for recent input from the user
     */
    pub fn handle_input(&mut self, rl: &mut RaylibHandle) {    
        if self.moving { return; }

        if rl.is_key_down(KeyboardKey::KEY_RIGHT) {
            self.moving = true;
            self.dir = Direction::Right;
        }else if rl.is_key_down(KeyboardKey::KEY_LEFT) {
            self.moving = true;
            self.dir = Direction::Left;
        }else if rl.is_key_down(KeyboardKey::KEY_DOWN) {
            self.moving = true;
            self.dir = Direction::Down;
        }else if rl.is_key_down(KeyboardKey::KEY_UP) {
            self.moving = true;
            self.dir = Direction::Up;
        }

        if self.moving {
            let index = self.base.get_index();
            self.base.set_side(dir_to_u32(&self.dir));
            if index == 1 || index == 2 {
                self.base.set_index(2);
            }
            self.base.set_shift(self.base.get_default_shift());
        }
    }
}
