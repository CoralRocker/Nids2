#![allow(dead_code, unused_imports)]

use std::convert::{ TryFrom, TryInto };
use raylib::prelude::*;
use nids2::{game, naomi, object};
use std::rc;
use std::cell::RefCell;

fn insert_object(v: &mut Vec<Vec<rc::Rc<RefCell<dyn object::Object>>>>, obj: rc::Rc<RefCell<dyn object::Object>>){
    let depth = obj.borrow().get_depth() as usize;
    v.get_mut(depth)
        .expect("Invalid depth for object!")
        .push(obj); 
}

fn is_object_correctly_placed(v: &Vec<Vec<rc::Rc<RefCell<dyn object::Object>>>>, obj: rc::Rc<RefCell<dyn object::Object>>) -> bool {
    let iter = &v.get(obj.borrow().get_depth() as usize).expect("Object depth is invalid!");
    match iter.iter().find(|&x|->bool{ x.borrow().get_id() == obj.borrow().get_id() }){
        Some(_) => true,
        None => false,
    }
}

/** Find an object in the list by it's ID, remove it, and add it back at the correct depth. If the object is already in the correct position, this does nothing.
 */
fn update_object_in_list(v: &mut Vec<Vec<rc::Rc<RefCell<dyn object::Object>>>>, obj: rc::Rc<RefCell<dyn object::Object>>) {
    if is_object_correctly_placed(v, obj.clone()) { return }
    let id = obj.borrow().get_id();
    for depth in v.iter_mut() {
        match depth.iter().position(|x| -> bool { return x.borrow().get_id() == id; }){
            Some(p) => { depth.remove(p); break; },
            None => (),
        }
    }
    insert_object(v, obj);
}

fn get_all_obj(v: &Vec<Vec<rc::Rc<RefCell<dyn object::Object>>>>) -> Vec<rc::Rc<RefCell<dyn object::Object>>> {
    let mut res = Vec::new();  
    
    for depth in v.iter() {
        res.append(&mut depth.clone());
    }

    return res;
}

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

    
    let mut objects: Vec<Vec<rc::Rc<RefCell<dyn object::Object>>>> = Vec::new();
    objects.resize(scr_h as usize, Vec::new());
    let naomi = rc::Rc::new(RefCell::new(naomi::Naomi::new(object::Position::new(0,0),1, scr_w, scr_h)));
    insert_object(&mut objects, naomi.clone());
    insert_object(&mut objects, rc::Rc::new(RefCell::new(object::GenericObject::new(2, 2, Some(object::Position::new(42, 42))))));

    while !rl.window_should_close() {
        frame_no += 1;
         
        for depth in objects.iter() {
            for obj in depth.iter() {
                obj.borrow_mut().do_step(frame_no);
            }
        }
        for obj in get_all_obj(&objects).iter() {
            update_object_in_list(&mut objects, obj.clone());
        }

        naomi.borrow_mut().handle_input(&mut rl); 

        let mut d = rl.begin_drawing(&thread); 
        {
            for depth in objects.iter() {
                for obj in depth.iter() {
                    obj.borrow().draw(&mut d);
                }
            }
            d.clear_background(Color::WHITE);
        }
    }

    game::destroy();
}

