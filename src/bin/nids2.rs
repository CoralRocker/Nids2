#![allow(dead_code, unused_imports, unused_variables, non_snake_case)]

use lazy_static::lazy_static;
use nids2::naomi::*;
use nids2::object::*;
use nids2::save::*;
use nids2::{game, naomi, object, util};
use raylib::ffi::Rectangle as ffirect;
use raylib::prelude::*;
use std::cell::RefCell;
use std::convert::{TryFrom, TryInto};
use std::ffi::CString;
use std::fs;
use std::io::Read;
use std::rc;
use std::sync::{atomic, Mutex};
use std::ops::DerefMut;

/// Type alias because me is lazy
type GenObj = rc::Rc<RefCell<object::GenericObject>>;

fn save_to_file(fname: &str, objs: &[GenObj], player: &Naomi) {
    let mut result = objs.to_bytes();
    result.extend(player.to_bytes().iter());
    fs::write(fname, result).unwrap();
}

fn load_from_file(
    fname: &str,
    objs: &mut Vec<GenObj>,
    player: &mut Naomi,
) {
    let mut file = fs::File::open(fname).unwrap();
    let mut result = Vec::new();
    let size = file.read_to_end(&mut result).unwrap();
    let objs_res =
        Vec::<GenObj>::from_bytes(result.as_slice()).unwrap();
    let plyr_res = Naomi::from_bytes(&result[objs_res.1..]).unwrap();

    *objs = objs_res.0;
    *player = plyr_res.0;
    // println!("Loaded {} objects: ", objs.len());
    // for obj in objs.iter() {
    //     println!("\t{}", obj.borrow());
    // }
}

#[derive(PartialEq)]
enum MenuSelections {
    MenuClosed,
    TypeSelect,
    ItemSelect,
    ColorSelect,
    SaveExit,
}


fn main() {
    let scr_w = 640;
    let scr_h = 640;
    let debug = false;

    /* GAME SCREEN AND STATIC INITIALIZATION */
    let (mut rl, thread) = raylib::init()
        .size(scr_w, scr_h)
        .title("Hello, World")
        .build();

    game::init(&mut rl, &thread);

    rl.gui_load_style(Some(rstr!("candy.rgs")));
    let font = rl.load_font(&thread, "v5easter.ttf").unwrap();
    rl.gui_set_font(&font);
    rl.set_target_fps(60);
    game::color_init(&mut rl);
    rl.set_exit_key(None);

    let mut id_counter = 1;

    /* Game Loop Variables */
    let mut frame_no: i32 = 0;
    let mut pause = false;
    let mut exit = false;
    let mut menu_selection = MenuSelections::MenuClosed;
    let mut opt_selection = 0;
    let mut opt_scroll_index = 0;
    let mut selected_item = 0;
    let mut selected_item_scroll_index = 0;
    let mut drag: Option<(GenObj, Vector2, Vector2)> = None; // Hold whether or not an object drag was detected

    /* Constant Object Type Vectors */
    let types_vec = util::get_all_types(true);
    let sorted_objs = util::get_all_objects_sorted(true);

    /* Color Selection Vector */
    let color_wheel = vec![
        Color::WHITE,
        Color::RAYWHITE,
        Color::LIGHTGRAY,
        Color::GRAY,
        Color::DARKGRAY,
        Color::BLACK,
        Color::YELLOW,
        Color::GOLD,
        Color::ORANGE,
        Color::PINK,
        Color::RED,
        Color::MAROON,
        Color::MAGENTA,
        Color::SKYBLUE,
        Color::BLUE,
        Color::DARKBLUE,
        Color::PURPLE,
        Color::VIOLET,
        Color::DARKPURPLE,
        Color::BEIGE,
        Color::BROWN,
        Color::DARKBROWN,
        Color::DARKGREEN,
        Color::GREEN,
        Color::LIME,
    ];
    let color_wheel_str: Vec<String> = vec![
        "White",
        "RayWhite",
        "Light Gray",
        "Gray",
        "Dark Gray",
        "Black",
        "Yellow",
        "Gold",
        "Orange",
        "Pink",
        "Red",
        "Maroon",
        "Magenta",
        "Sky Blue",
        "Blue",
        "Dark Blue",
        "Purple",
        "Violet",
        "Dark Purple",
        "Beige",
        "Brown",
        "Dark Brown",
        "Dark Green",
        "Green",
        "Lime Green",
    ]
    .iter()
    .map(|s| s.to_string())
    .collect();

    // Generate tiled background at the start of the game program.
    let background_tiles = {
        let mut bckg = Image::gen_image_color(scr_w, scr_h, Color::WHITE);
        let tile = Image::load_image("data/spr_tile.png").expect("Unable to open tile sprite!");
        let tile_rect = rrect(0, 0, tile.width(), tile.height());
        let tile_h = scr_w / tile.width();
        let tile_v = scr_h / tile.height();
        let mut draw_rect = tile_rect;

        for i in 0..tile_v {
            draw_rect.y = (i * tile.height()) as f32;
            for j in 0..tile_h {
                draw_rect.x = (j * tile.width()) as f32;
                bckg.draw(&tile, tile_rect, draw_rect, Color::WHITE);
            }
        }

        rl.load_texture_from_image(&thread, &bckg)
            .expect("Unable load texture from image!")
    };

    let mut obj_refactor: Vec<GenObj> = Vec::new();

    // Create Naomi Player Object
    let mut naomi = naomi::Naomi::new(object::Position::new(64, 64), 1, scr_w, scr_h);
    id_counter += 1;
    
    // Load save file or create appropriate new game setup
    if fs::File::open("data/nids.sav").is_ok() {
        /* Load All Objects */
        println!("Loading nids.sav file...");
        let mut all_obj = Vec::new();
        load_from_file("data/nids.sav", &mut all_obj, &mut naomi);
        for obj in all_obj.iter() {
            obj_refactor.push(obj.clone());
        }
        obj_refactor.sort_unstable_by_key(|a| a.borrow().get_depth());
        
        // Calculate next valid ID to assign items
        let mut max_id = -1;
        for obj in &obj_refactor {
            if obj.borrow().get_id() > max_id {
                max_id = obj.borrow().get_id();
            }
        }
        id_counter = max_id + 1;

    } else {
        //
        // Create Walls
        //
        for x in (32..scr_w - 32).step_by(32) {
            obj_refactor.push(rc::Rc::new(RefCell::new(GenericObject::new(
                id_counter,
                0,
                Some(Position::new(x, 0)),
            ))));

            id_counter += 1;
        }
        for y in (0..scr_h).step_by(64) {
            obj_refactor.push(rc::Rc::new(RefCell::new(GenericObject::new(
                id_counter,
                9,
                Some(Position::new(0, y)),
            ))));
            id_counter += 1;
            obj_refactor.push(rc::Rc::new(RefCell::new(GenericObject::new(
                id_counter,
                9,
                Some(Position::new(scr_w - 32, y)),
            ))));
            if y == (scr_h as f32 * (1.5/3.)) as i32 {
                for x in 32..(scr_w as f32 * (4./7.))as i32 {
                    obj_refactor.push(rc::Rc::new(RefCell::new(GenericObject::new(
                        id_counter,
                        0,
                        Some(Position::new(x, y)),
                    ))));

                    id_counter += 1;
                    
                }
            }
            id_counter += 1;
        }
    }
    

    let mut target = rl.load_render_texture(&thread, scr_w as u32, scr_h as u32).unwrap(); 

    /* GAME LOOP */
    while !exit {
        frame_no += 1; // Frame counter

        // Do Required Actions for all objects on screen
        for obj in obj_refactor.iter() {
            obj.borrow_mut().do_step(frame_no);
        }
        naomi.do_step(frame_no); // Naomi object is updated seperately for drawing reasons

        // Pause key
        if rl.is_key_pressed(KeyboardKey::KEY_P)
            || (rl.is_key_pressed(KeyboardKey::KEY_ESCAPE)
                && menu_selection == MenuSelections::MenuClosed
                && naomi.select_obj.is_none())
        {
            pause = !pause;
        }

        // Handle player input && game logic if game is not paused
        if !pause {
            // naomi::handle_input returns an object if one was placed down. This transfers
            // ownership of the object from naomi to the main object vector
            if let Some(r) = naomi.handle_input(&mut rl, &mut id_counter, &mut obj_refactor) {
                let target_id = r.borrow().get_id();
                println!("Removing obj {}", r.borrow());
                obj_refactor.retain(|obj| obj.borrow().get_id() != r.borrow().get_id());
            }
        

            /* DRAG GESTURE DETECTION */
            // Pick up an object and move it somewhere
            if rl.is_mouse_button_pressed(MouseButton::MOUSE_LEFT_BUTTON)
            {

                // Determine if mouse landed on selectable item
                // Select based off of bounding box, or if None, sprite_area.
                // TODO: Pixel-perfect collisions?
                let pos = rl.get_mouse_position();

                obj_refactor.iter().rev().find(|&obj| {
                    if obj
                        .borrow()
                        .get_collision_rect()
                        .check_collision_point_rec(pos)
                        && !obj.borrow().object_data.1.category.eq("sys")
                    {
                        if naomi.select_obj.is_none() {
                            naomi.select_obj = Some(obj.clone());
                            naomi.select_obj_type = obj.borrow().obj_id;
                        }
                        drag = Some((obj.clone(), pos, pos));

                        return true;
                    }
                    false
                });
            }else if rl.is_mouse_button_down(MouseButton::MOUSE_LEFT_BUTTON) {
                // DRAG DETECTED! 
                let pos = rl.get_mouse_position();
                if let Some((obj, old_pos, first_pos)) = &mut drag {
                    let delta = pos - *old_pos;
                    if obj.borrow().get_collision_rect().check_collision_point_rec(pos) {
                        *old_pos = pos; 
                        let mut obj = obj.borrow_mut();
                        obj.pos.x += delta.x as i32;
                        obj.pos.y += delta.y as i32;
                    }else{
                        rl.set_mouse_position(*old_pos);
                        // { // Preserve our original non-mut obj ptr so we can compare it
                        //     let mut obj = obj.borrow_mut();
                        //     obj.pos.x -= obj.pos.x % 4;
                        //     obj.pos.y -= obj.pos.y % 4;
                        // } // Mut borrow expires here
                        // // Remove object from naomi's posession once moved.
                        // if rl.get_mouse_position() != *first_pos {
                        //     if let Some(nobj) = &naomi.select_obj {
                        //         if *nobj.borrow() == *obj.borrow() {
                        //             naomi.select_obj = None;
                        //         }   
                        //     }
                        // }else{ // Object was selected...
                        //     naomi.grab_object(obj.clone());
                        // }
                        // drag = None;
                    }

                    let scroll = rl.get_mouse_wheel_move();
                    if scroll != 0. {
                        let target_clr = obj.borrow().colormod;
                        let mut clr_pos: i32 = color_wheel.iter().position(|&c| c == target_clr).unwrap_or(0) as i32;
                        if scroll < 0. {
                            clr_pos -= 1;
                            if clr_pos < 0 {
                                clr_pos = (color_wheel.len() - 1) as i32;
                            }
                        }else{
                            clr_pos += 1;
                            if clr_pos >= color_wheel.len() as i32{
                                clr_pos = 0;
                            }
                        }
                        obj.borrow_mut().colormod = color_wheel[clr_pos as usize];
                    }
                }
            }else {
                if let Some((obj, old_pos, first_pos)) = drag {
                    { // Preserve our original non-mut obj ptr so we can compare it
                        let mut obj = obj.borrow_mut();
                        obj.pos.x -= obj.pos.x % 4;
                        obj.pos.y -= obj.pos.y % 4;
                    } // Mut borrow expires here
                    // Remove object from naomi's posession once moved.
                    if rl.get_mouse_position() != first_pos {
                        if let Some(nobj) = &naomi.select_obj {
                            if *nobj.borrow() == *obj.borrow() {
                                naomi.select_obj = None;
                            }   
                        }
                    }else{ // Object was selected...
                        naomi.grab_object(obj.clone());
                    }
                    drag = None;
                }
            }
        }
        
        // Sort objects by depth. Unstable is better for nearly-sorted lists, which this is.
        obj_refactor.sort_unstable_by_key(|a| a.borrow().get_depth());

        /* DRAW SECTION */
        let mut d = rl.begin_drawing(&thread);
        {
            let mut d = d.begin_texture_mode(&thread, &mut target);

            // Draw floor
            d.draw_texture(&background_tiles, 0, 0, Color::WHITE);

            {
                // Draw all objects onto the screen. Naomi object gets drawn at the correct depth
                let target_depth = naomi.get_depth();
                let mut naomi_drawn = false;
                for obj in obj_refactor.iter() {
                    // Ensure naomi is drawn once, and first at it's depth.
                    if !naomi_drawn && obj.borrow().get_depth() >= target_depth {
                        naomi.draw(&mut d, debug);
                        naomi_drawn = true;
                    }
                    obj.borrow().draw(&mut d, debug);
                }
                if !naomi_drawn { // Backup naomi drawing
                    naomi.draw(&mut d, debug);
                }
            }
        }
        
        // Draw render target
        d.draw_texture_pro(&target,
                           rrect(0,0, target.width(), -target.height()),
                           rrect(0,0, target.width(), target.height()),
                           rvec2(0,0),
                           0.,
                           Color::WHITE);
        
        /* PAUSE MENU SECTION */
        if pause {
            let menu_height = 128;

            /* Draw Base Buttons */
            let (furn_button, furn_vec) = util::ds_rounded_button_centered(
                &mut d,
                &font,
                rrect(
                    scr_w as f32 * 0.25,
                    scr_h - menu_height / 2,
                    scr_w / 4,
                    menu_height / 2,
                ),
                Some("Select Furniture"),
                true,
            );
            let (clr_button, clr_vec) = util::ds_rounded_button_centered(
                &mut d,
                &font,
                rrect(
                    scr_w as f32 * 0.5,
                    scr_h - menu_height / 2,
                    scr_w / 4,
                    menu_height / 2,
                ),
                Some("Color Selection"),
                true,
            );

            let (exit_button, exit_vec) = util::ds_rounded_button_centered(
                &mut d,
                &font,
                rrect(
                    scr_w as f32 * 0.75,
                    scr_h - menu_height / 2,
                    scr_w / 4,
                    menu_height / 2,
                ),
                Some("Save and Exit"),
                true,
            );

            /* Handle Button Returns */
            if furn_button {
                menu_selection = if menu_selection == MenuSelections::MenuClosed {
                    MenuSelections::TypeSelect
                } else {
                    MenuSelections::MenuClosed
                };
            } else if clr_button {
                menu_selection = MenuSelections::ColorSelect;
            } else if exit_button {
                menu_selection = MenuSelections::SaveExit;
            }

            /* Close Certain Menus When Escape Is Pressed */
            /* Switch Menus With Keyboard Keys */
            if d.is_key_pressed(KeyboardKey::KEY_ESCAPE) {
                match menu_selection {
                    MenuSelections::ItemSelect => menu_selection = MenuSelections::TypeSelect,
                    MenuSelections::TypeSelect => menu_selection = MenuSelections::MenuClosed,
                    MenuSelections::ColorSelect => menu_selection = MenuSelections::MenuClosed,
                    MenuSelections::SaveExit => menu_selection = MenuSelections::MenuClosed,
                    MenuSelections::MenuClosed => (), // Already processed KEY_ESCAPE, don't process here.
                }
            }

            /* Draw The Proper Submenu */
            match menu_selection {
                MenuSelections::TypeSelect => {
                    if util::ds_scroll_selection_auto(
                        &mut d,
                        &font,
                        rrect(furn_vec.x, furn_vec.y, scr_w / 4, 0),
                        3,
                        &types_vec,
                        &mut opt_selection,
                        &mut opt_scroll_index,
                    ) {
                        menu_selection = MenuSelections::ItemSelect;
                    }
                }
                MenuSelections::ItemSelect => {
                    let vec_ref = sorted_objs
                        .get(types_vec.get(opt_selection as usize).unwrap())
                        .unwrap();
                    let mut name_vec = Vec::new();
                    for item in vec_ref.iter() {
                        name_vec.push(item.1.name.clone());
                    }
                    if util::ds_scroll_selection_auto(
                        &mut d,
                        &font,
                        rrect(furn_vec.x, furn_vec.y, scr_w / 4, 0),
                        3,
                        &name_vec,
                        &mut selected_item,
                        &mut selected_item_scroll_index,
                    ) {
                        naomi.select_obj_type = vec_ref.get(selected_item as usize).unwrap().1.id;
                        menu_selection = MenuSelections::MenuClosed;
                    }
                }
                MenuSelections::ColorSelect => {
                    if util::ds_scroll_selection_auto(
                        &mut d,
                        &font,
                        rrect(clr_vec.x, clr_vec.y, scr_w / 4, 0),
                        3,
                        &color_wheel_str,
                        &mut opt_selection,
                        &mut opt_scroll_index,
                    ) {
                        naomi.colormod = color_wheel[opt_selection as usize];
                        if let Some(o) = &naomi.select_obj {
                            o.borrow_mut().colormod = naomi.colormod;
                            menu_selection = MenuSelections::MenuClosed;
                        }
                    }
                }
                MenuSelections::SaveExit => {
                    save_to_file("data/nids.sav", &obj_refactor, &naomi);
                    exit = true;
                }
                _ => (),
            };
        }
    }

    // Clean up initialized memory
    // If objects vector is used after this, bad things may occur as the memory will no longer be
    // valid.
    game::destroy();
}
