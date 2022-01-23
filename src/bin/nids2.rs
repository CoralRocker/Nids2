#![allow(dead_code, unused_imports, unused_variables, non_snake_case)]

use nids2::{game, naomi, object};
use raylib::prelude::*;
use std::cell::RefCell;
use std::convert::{TryFrom, TryInto};
use std::rc;
use raylib::ffi::Rectangle as ffirect;
use std::ffi::CString;
use lazy_static::lazy_static;
use std::sync::{atomic, Mutex};

lazy_static! {
    static ref BASE_COLOR_NORMAL: Mutex<Color> = Mutex::new(Color::WHITE);
    static ref BORDER_COLOR_NORMAL: Mutex<Color> = Mutex::new(Color::WHITE);
    static ref BORDER_COLOR_FOCUSED: Mutex<Color> = Mutex::new(Color::WHITE);
    static ref BASE_COLOR_FOCUSED: Mutex<Color> = Mutex::new(Color::WHITE);
    static ref BORDER_COLOR_PRESSED: Mutex<Color> = Mutex::new(Color::WHITE);
    static ref BASE_COLOR_PRESSED: Mutex<Color> = Mutex::new(Color::WHITE);
    static ref BORDER_WIDTH: Mutex<i32> = Mutex::new(0);
}

// Sets the value in the given mutex to be equivalent or a copy of `val`
fn mutexSet<T: Clone>(
    m: &Mutex<T>, 
    val: T
) {
    *m.lock().expect("Unable to lock mutex given to MutexSet") = val.clone();
}

fn mutexGet<T: Clone>(
    m: &Mutex<T>
) -> T {
    m.lock().expect("Unable to lock mutexGet mutex").clone()
}

fn color_init(
    rd: &mut RaylibHandle
) {
    mutexSet(&BASE_COLOR_NORMAL, Color::get_color(rd.gui_get_style(GuiControl::BUTTON, GuiControlProperty::BASE_COLOR_NORMAL as i32)));
    mutexSet(&BORDER_COLOR_NORMAL, Color::get_color(rd.gui_get_style(GuiControl::BUTTON, GuiControlProperty::BORDER_COLOR_NORMAL as i32)));
    mutexSet(&BORDER_COLOR_FOCUSED, Color::get_color(rd.gui_get_style(GuiControl::BUTTON, GuiControlProperty::BORDER_COLOR_FOCUSED as i32)));
    mutexSet(&BASE_COLOR_FOCUSED, Color::get_color(rd.gui_get_style(GuiControl::BUTTON, GuiControlProperty::BASE_COLOR_FOCUSED as i32)));
    mutexSet(&BORDER_COLOR_PRESSED, Color::get_color(rd.gui_get_style(GuiControl::BUTTON, GuiControlProperty::BORDER_COLOR_PRESSED as i32)));
    mutexSet(&BASE_COLOR_PRESSED, Color::get_color(rd.gui_get_style(GuiControl::BUTTON, GuiControlProperty::BASE_COLOR_PRESSED as i32)));
    mutexSet(&BORDER_WIDTH, rd.gui_get_style(GuiControl::BUTTON, GuiControlProperty::BORDER_WIDTH as i32));
}

fn max(
    a: i32,
    b: i32
) -> i32 {
    if a > b {
        a
    } else {
        b
    }
}

fn insert_object(
    v: &mut Vec<Vec<rc::Rc<RefCell<dyn object::Object>>>>,
    obj: rc::Rc<RefCell<dyn object::Object>>,
) {
    let depth = obj.borrow().get_depth() as usize;
    v.get_mut(depth)
        .expect("Invalid depth for object!")
        .push(obj);
}

fn is_object_correctly_placed(
    v: &[Vec<rc::Rc<RefCell<dyn object::Object>>>],
    obj: rc::Rc<RefCell<dyn object::Object>>,
) -> bool {
    let iter = &v
        .get(obj.borrow().get_depth() as usize)
        .expect("Object depth is invalid!");

    iter.iter()
        .any(|x| -> bool { x.borrow().get_id() == obj.borrow().get_id() })
}

/** Find an object in the list by it's ID, remove it, and add it back at the correct depth. If the object is already in the correct position, this does nothing.
 */
fn update_object_in_list(
    v: &mut Vec<Vec<rc::Rc<RefCell<dyn object::Object>>>>,
    obj: rc::Rc<RefCell<dyn object::Object>>,
) {
    if is_object_correctly_placed(v, obj.clone()) {
        return;
    }
    let id = obj.borrow().get_id();
    for depth in v.iter_mut() {
        if let Some(p) = depth
            .iter()
            .position(|x| -> bool { x.borrow().get_id() == id })
        {
            depth.remove(p);
            break;
        }
    }
    insert_object(v, obj);
}

fn get_all_obj(
    v: &[Vec<rc::Rc<RefCell<dyn object::Object>>>],
) -> Vec<rc::Rc<RefCell<dyn object::Object>>> {
    let mut res = Vec::new();

    for depth in v.iter() {
        res.append(&mut depth.clone());
    }

    res
}

fn get_viewport(scr_w: i32, scr_h: i32) -> Rectangle {
    rrect(0, 0, scr_w, scr_h - 256)
}

fn ds_rounded_rectangle(
    rd: &mut RaylibDrawHandle,
    rec: impl Into<ffirect>,
    roundness: f32,
    segments: i32,
) {
    let D_BASE_COLOR_NORMAL = Color::get_color(rd.gui_get_style(GuiControl::DEFAULT, GuiDefaultProperty::BACKGROUND_COLOR as i32));
    
    rd.draw_rectangle_rounded(rec, roundness, segments, D_BASE_COLOR_NORMAL);
    
}
fn ds_rounded_rectangle_lines(
    rd: &mut RaylibDrawHandle,
    rec: impl Into<ffirect>,
    roundness: f32,
    segments: i32,
    line_width: i32
) {
    let D_BORDER_COLOR_NORMAL = Color::get_color(rd.gui_get_style(GuiControl::DEFAULT, GuiDefaultProperty::LINE_COLOR as i32));
    
    rd.draw_rectangle_rounded_lines(rec, roundness, segments, line_width, D_BORDER_COLOR_NORMAL);
    
}

fn ds_rounded_button_centered(
    rd: &mut RaylibDrawHandle,
    rec: impl Into<ffirect>,
    text: Option<&str>
) -> (bool, Vector2) {
    let mut rec = {
        let rec = rec.into();
        let rec: Rectangle = rec.into();
        rec
    };
    
    rec.x -= rec.width/2.0;
    rec.y -= rec.height/2.0;
    
    ds_rounded_button(rd, rec, text)
}

fn ds_rounded_button(
    rd: &mut RaylibDrawHandle,
    rec: impl Into<ffirect>,
    text: Option<&str>,
) -> (bool, Vector2) {
    
    let mut rec = {
        let rec = rec.into();
        let rec: Rectangle = rec.into();
        rec
    };
    
    let border_w: f32 = mutexGet(&BORDER_WIDTH) as f32;

    // let mut rec: Rectangle = rec.into();
    rec.x += border_w;
    rec.y += border_w;
    rec.width -= 2.0 *border_w;
    rec.height -= 2.0 *border_w;
    
    let base_color: Color;
    let border_color: Color;
    
    let mut pressed = false;

    if !rec.check_collision_point_rec(rd.get_mouse_position()) {
        base_color = mutexGet(&BASE_COLOR_NORMAL);
        border_color = mutexGet(&BORDER_COLOR_NORMAL);
    } else {
        if rd.is_mouse_button_down(MouseButton::MOUSE_LEFT_BUTTON) {
            if rd.is_mouse_button_pressed(MouseButton::MOUSE_LEFT_BUTTON) {
                pressed = true;
            }
            base_color = mutexGet(&BASE_COLOR_PRESSED);
            border_color = mutexGet(&BORDER_COLOR_PRESSED);
        } else {
            base_color = mutexGet(&BASE_COLOR_FOCUSED);
            border_color = mutexGet(&BORDER_COLOR_FOCUSED);
        }
    }

    rd.draw_rectangle_rounded(
        rec,
        0.4,
        5,
        base_color
    );
    rd.draw_rectangle_rounded_lines(
        rec,
        0.4,
        5,
        border_w as i32,
        border_color
    );

    if let Some(s) = text {
        // let _ctext = CString::new(s);
        game::draw_text_centered(
            rd, 
            s, 
            (rec.x + rec.width/2.0) as i32, 
            (rec.y + rec.height/2.0) as i32, 
            16, 
            Color::BLACK
        );
    }


    (pressed, rvec2(rec.x, rec.y))
}

fn rel_rect(rec: Rectangle, x: i32, y: i32, width: f32, height: f32) -> Rectangle {
    rrect(
        rec.x + x as f32,
        rec.y + y as f32,
        width,
        height,
    )
}

fn rect_midpoint(rec: Rectangle) -> (i32, i32) {
    ((rec.x + rec.width/2.0) as i32, (rec.y + rec.height/2.0) as i32)
}


fn ds_scroll_selection(
    rd: &mut RaylibDrawHandle,
    rec: Rectangle,
    selections: &Vec<String>,
    selection: &mut i32
) -> bool {
    let active = rec.check_collision_point_rec(rd.get_mouse_position());
    let border_w = mutexGet(&BORDER_WIDTH);
    let mut item_rect = rrect(rec.x + 2.0, rec.y - 20.0, rec.width - 4.0, 20);

    rd.draw_rectangle_rounded(
        rec,
        0.4,
        5,
        mutexGet(&BASE_COLOR_NORMAL)
    );
    rd.draw_rectangle_rounded_lines(
        rec,
        0.4,
        5,
        border_w,
        mutexGet(&BORDER_COLOR_NORMAL)
    );
    
    let num_items = (rec.height - 12.0) as i32 / 20;
    
    // Scroll Selection Logic
    if *selection >= selections.len() as i32 {
        *selection = selections.len() as i32 - num_items;
    }
    if *selection < 0 {
        *selection = 0;
    }
    
    for n in 0..num_items {
        item_rect.y += 22.0;
        let (cx, cy) = rect_midpoint(item_rect);
        let txt = selections.get((n + *selection) as usize);
        if let Some(t) = txt {
            game::draw_text_centered(rd,
                                     t,
                                     cx,
                                     cy,
                                     12,
                                     Color::BLACK
                        );
        }
        if item_rect.check_collision_point_rec(rd.get_mouse_position()) {
            rd.draw_rectangle_rounded(item_rect,
                                      0.5,
                                      4,
                                      Color::WHITE.fade(0.40)
                                    );
            if rd.is_mouse_button_pressed(MouseButton::MOUSE_LEFT_BUTTON) {
                *selection += n;
                return true;
            }
        }
    }
    
    

    if active {
        *selection += -rd.get_mouse_wheel_move() as i32;
        if *selection < 0 {
            *selection = 0;
        } else if *selection >= selections.len() as i32 {
            *selection = selections.len() as i32 -1;
        }
    }

    false    
}

#[derive(PartialEq)]
enum MenuSelections {
    MenuClosed,
    TypeSelect,
    ItemSelect,
    Options,
    SaveExit,
}

fn main() {
    let scr_w = 640;
    let scr_h = 480;

    let (mut rl, thread) = raylib::init()
        .size(scr_w, scr_h)
        .title("Hello, World")
        .build();

    game::init(&mut rl, &thread);

    rl.gui_load_style(Some(rstr!("candy.rgs")));
    let font = rl.load_font(&thread, "v5easter.ttf").unwrap();
    rl.gui_set_font(font);
    rl.set_target_fps(60);
    color_init(&mut rl);
    let mut frame_no: i32 = 0;
    let mut pause = false;
    let mut menu_selection = MenuSelections::MenuClosed;
    let mut opt_selection = 0;   
    let mut selected_item = 0;
    
    let types_vec = game::get_all_types();
    let sorted_objs = game::get_all_objects_sorted();

    let background_tiles = {
        let mut bckg = Image::gen_image_color(scr_w, scr_h, Color::WHITE);
        let tile = Image::load_image("data/spr_tile.png").expect("Unable to open tile sprite!");
        let tile_rect = rrect(0, 0, tile.width(), tile.height());
        let tile_h = scr_w / tile.width();
        let tile_v = scr_h / tile.height();
        let mut draw_rect = tile_rect;

        for i in 0..tile_v {
            draw_rect.y = (i*tile.height()) as f32;
            for j in 0..tile_h {
                draw_rect.x = (j*tile.width()) as f32;
                bckg.draw(&tile, tile_rect, draw_rect, Color::WHITE);
            }
        }

        rl.load_texture_from_image(&thread, &bckg).expect("Unable load texture from image!")
    };

    let mut objects: Vec<Vec<rc::Rc<RefCell<dyn object::Object>>>> = Vec::new();
    objects.resize(scr_h as usize, Vec::new());
    let naomi = rc::Rc::new(RefCell::new(naomi::Naomi::new(
        object::Position::new(0, 0),
        1,
        scr_w,
        scr_h,
    )));
    insert_object(&mut objects, naomi.clone());

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
        
        if rl.is_key_pressed(KeyboardKey::KEY_P) {
            pause = !pause;
        }

        if !pause { 
            if let Some(obj) = naomi.borrow_mut().handle_input(&mut rl, frame_no) {
                let mut depth = obj.borrow().depth;
                if depth < 0 { 
                    depth = 0; 
                }else if depth > scr_h {
                    depth = scr_h-1;
                }
                objects.get_mut(depth as usize)
                    .unwrap()
                    .push(obj.clone());
                
            }
        }
        

        let mut d = rl.begin_drawing(&thread);
        // d.clear_background(Color::RAYWHITE);
        d.draw_texture(&background_tiles, 0, 0, Color::WHITE);
        for depth in objects.iter() {
            for obj in depth.iter() {
                obj.borrow().draw(&mut d);
            }
        }

        if pause {
            let menu_height = 128;
            let menu_bkgd_color = Color::from_hex("E0E645").unwrap().fade(0.75);
            let menu_frgd_color = Color::from_hex("EBD33B").unwrap();
            // d.gui_label_button(rrect(0, scr_h - menu_height, scr_w, menu_height), Some(rstr!("Pause Menu")));
            
            let (furn_button, furn_vec) = ds_rounded_button_centered(
                &mut d,
                rrect(scr_w as f32 * 0.25, scr_h - menu_height/2, scr_w/4, menu_height/2),
                Some("Select Furniture")
            );
            let (opt_button, opt_vec)  = ds_rounded_button_centered(
                &mut d,
                rrect(scr_w as f32 * 0.5, scr_h - menu_height/2, scr_w/4, menu_height/2),
                Some("Options")
            );

            let (exit_button, exit_vec) = ds_rounded_button_centered(
                &mut d,
                rrect(scr_w as f32 *0.75, scr_h - menu_height/2, scr_w/4, menu_height/2),
                Some("Save and Exit")
            );

            if furn_button {
                menu_selection = if menu_selection == MenuSelections::MenuClosed { MenuSelections::TypeSelect } else { MenuSelections::MenuClosed };
            }else if opt_button  {
                menu_selection = MenuSelections::Options;
            }else if exit_button {
                menu_selection = MenuSelections::SaveExit;
            }
            
            match menu_selection {
                MenuSelections::TypeSelect => {
                    // let v = vec!["hi", "hello", "goodbye", "tchao", "ciao", "'sta matina"];
                    let scroll_height = 80.0;
                    if ds_scroll_selection(
                        &mut d,
                        rrect(furn_vec.x, furn_vec.y - scroll_height, scr_w/4, scroll_height),
                        &types_vec,
                        &mut opt_selection
                    ) {
                        menu_selection = MenuSelections::ItemSelect;
                    }
                },
                MenuSelections::Options => (),
                MenuSelections::SaveExit => (),
                MenuSelections::ItemSelect => {
                    let vec_ref = sorted_objs.get(types_vec.get(opt_selection as usize).unwrap()).unwrap();
                    let scroll_height = 80.0;
                    let mut name_vec = Vec::new();
                    for item in vec_ref.iter() {
                        name_vec.push(item.1.name.clone());
                    }
                    if ds_scroll_selection(
                        &mut d,
                        rrect(furn_vec.x, furn_vec.y - scroll_height, scr_w/4, scroll_height),
                        &name_vec,
                        &mut selected_item
                    ) {
                        naomi.borrow_mut().select_obj_type = vec_ref.get(selected_item as usize).unwrap().1.id;
                    }
                },
                _ => (),
            };

        }
        
    }

    game::destroy();
}
