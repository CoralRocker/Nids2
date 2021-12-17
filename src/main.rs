#![allow(dead_code, unused_imports)]

use std::convert::{ TryFrom, TryInto };
use raylib::prelude::*;
use nids2::{game, naomi, object};
use std::rc;
use std::cell::RefCell;

fn main() {
    let scr_w = 640;
    let scr_h = 480;

    let (mut rl, thread) = raylib::init()
        .size(scr_w.try_into().unwrap(), scr_h.try_into().unwrap())
        .title("Hello, World")
        .build();
    
    game::init(&mut rl, &thread);
    
    rl.set_target_fps(60);
    let mut frame_no: i32 = 0;

    
    let mut objects: Vec<rc::Rc<RefCell<dyn object::Object>>> = Vec::new();
    let naomi = rc::Rc::new(RefCell::new(naomi::Naomi::new(object::Position::new(0,0),1)));
    objects.push(naomi.clone());

    while !rl.window_should_close() {
        frame_no += 1;

        if rl.is_mouse_button_pressed(MouseButton::MOUSE_LEFT_BUTTON) {
            let mouse_pos = rl.get_mouse_position();
        }
        
        for obj in objects.iter_mut() {
            obj.borrow_mut().do_step(frame_no);
        }
        naomi.borrow_mut().handle_input(&mut rl); 

        let mut d = rl.begin_drawing(&thread); 
        {
            for obj in objects.iter() {
                obj.borrow().draw(&mut d); 
            }
            d.clear_background(Color::WHITE);
        }
    }

    game::destroy();
}

