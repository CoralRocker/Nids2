#![allow(dead_code, unused_imports, unused_variables, non_snake_case)]

use lazy_static::lazy_static;
use nids2::{game, naomi, object, util};
use raylib::ffi::Rectangle as ffirect;
use raylib::prelude::*;
use std::cell::RefCell;
use std::convert::{TryFrom, TryInto};
use std::ffi::CString;
use std::rc;
use std::sync::{atomic, Mutex};

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
    game::color_init(&mut rl);
    let mut frame_no: i32 = 0;
    let mut pause = false;
    let mut menu_selection = MenuSelections::MenuClosed;
    let mut opt_selection = 0;
    let mut selected_item = 0;

    let types_vec = util::get_all_types();
    let sorted_objs = util::get_all_objects_sorted();

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

    let mut objects: Vec<Vec<rc::Rc<RefCell<dyn object::Object>>>> = Vec::new();
    objects.resize(scr_h as usize, Vec::new());
    let naomi = rc::Rc::new(RefCell::new(naomi::Naomi::new(
        object::Position::new(0, 0),
        1,
        scr_w,
        scr_h,
    )));
    util::insert_object(&mut objects, naomi.clone());

    while !rl.window_should_close() {
        frame_no += 1;

        for depth in objects.iter() {
            for obj in depth.iter() {
                obj.borrow_mut().do_step(frame_no);
            }
        }
        for obj in util::get_all_obj(&objects).iter() {
            util::update_object_in_list(&mut objects, obj.clone());
        }

        if rl.is_key_pressed(KeyboardKey::KEY_P) {
            pause = !pause;
        }

        if !pause {
            if let Some(obj) = naomi.borrow_mut().handle_input(&mut rl, frame_no) {
                let mut depth = obj.borrow().depth;
                if depth < 0 {
                    depth = 0;
                } else if depth > scr_h {
                    depth = scr_h - 1;
                }
                objects.get_mut(depth as usize).unwrap().push(obj.clone());
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

            let (furn_button, furn_vec) = util::ds_rounded_button_centered(
                &mut d,
                rrect(
                    scr_w as f32 * 0.25,
                    scr_h - menu_height / 2,
                    scr_w / 4,
                    menu_height / 2,
                ),
                Some("Select Furniture"),
            );
            let (opt_button, opt_vec) = util::ds_rounded_button_centered(
                &mut d,
                rrect(
                    scr_w as f32 * 0.5,
                    scr_h - menu_height / 2,
                    scr_w / 4,
                    menu_height / 2,
                ),
                Some("Options"),
            );

            let (exit_button, exit_vec) = util::ds_rounded_button_centered(
                &mut d,
                rrect(
                    scr_w as f32 * 0.75,
                    scr_h - menu_height / 2,
                    scr_w / 4,
                    menu_height / 2,
                ),
                Some("Save and Exit"),
            );

            if furn_button {
                menu_selection = if menu_selection == MenuSelections::MenuClosed {
                    MenuSelections::TypeSelect
                } else {
                    MenuSelections::MenuClosed
                };
            } else if opt_button {
                menu_selection = MenuSelections::Options;
            } else if exit_button {
                menu_selection = MenuSelections::SaveExit;
            }

            match menu_selection {
                MenuSelections::TypeSelect => {
                    // let v = vec!["hi", "hello", "goodbye", "tchao", "ciao", "'sta matina"];
                    let scroll_height = 80.0;
                    if util::ds_scroll_selection(
                        &mut d,
                        rrect(
                            furn_vec.x,
                            furn_vec.y - scroll_height,
                            scr_w / 4,
                            scroll_height,
                        ),
                        &types_vec,
                        &mut opt_selection,
                    ) {
                        menu_selection = MenuSelections::ItemSelect;
                    }
                }
                MenuSelections::Options => (),
                MenuSelections::SaveExit => (),
                MenuSelections::ItemSelect => {
                    let vec_ref = sorted_objs
                        .get(types_vec.get(opt_selection as usize).unwrap())
                        .unwrap();
                    let scroll_height = 80.0;
                    let mut name_vec = Vec::new();
                    for item in vec_ref.iter() {
                        name_vec.push(item.1.name.clone());
                    }
                    if util::ds_scroll_selection(
                        &mut d,
                        rrect(
                            furn_vec.x,
                            furn_vec.y - scroll_height,
                            scr_w / 4,
                            scroll_height,
                        ),
                        &name_vec,
                        &mut selected_item,
                    ) {
                        naomi.borrow_mut().select_obj_type =
                            vec_ref.get(selected_item as usize).unwrap().1.id;
                    }
                }
                _ => (),
            };
        }
    }

    game::destroy();
}
