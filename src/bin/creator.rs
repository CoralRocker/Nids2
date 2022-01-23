#![allow(unused_imports)]

use nids2::game::*;
use raylib::consts::KeyboardKey::*;
use raylib::prelude::*;
use std::ffi::{CStr, CString};
use std::fs;
use std::io::prelude::*;
use std::ops::DerefMut;

fn get_next_id() -> i32 {
    let mut max_id = 0;
    for entry in fs::read_dir("obj/").unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        let mut objconf = fs::File::open(path.join("obj.toml").to_str().unwrap()).unwrap();
        let mut confstr = String::new();
        let _ = objconf.read_to_string(&mut confstr).unwrap();

        let obj: ObjectConfig =
            toml::from_str(confstr.as_str()).expect("Unable to parse TOML Object configuration!");
        if obj.id >= max_id {
            max_id = obj.id + 1;
        }
    }
    max_id
}


fn divisors_bar(num: i32) -> Result<Vec<i32>, String> {
    if num < 1 {
        return Err("Cannot get divisors of a negative number!".to_string());
    }
    let mut res = Vec::new();
    for i in 1..num {
        if num % i == 0 {
            res.push(i);
        }
    }
    Ok(res)
}

fn div_to_cstr(nvec: &Vec<i32>) -> CString {
    let mut res_str = String::new();
    for num in nvec.iter() {
        res_str.push_str(format!("{};", num).as_str());
    }
    res_str.pop();
    CString::new(res_str).expect("Failed to create CString from create vector string")
}

fn side_selector(
    rld: &mut RaylibDrawHandle,
    bounds: Rectangle,
    options: &CString,
    title: &CString,
    selected_index: &mut i32,
) {
    let box_bound = rrect(
        bounds.x,
        bounds.y + 12.0,
        bounds.width,
        bounds.height - 12.0,
    );
    let gui_bound = rrect(
        bounds.x,
        bounds.y + 28.0,
        bounds.width,
        bounds.height - 28.0,
    );
    rld.gui_group_box(box_bound, Some(title.as_c_str()));
    *selected_index = rld.gui_combo_box(gui_bound, Some(options.as_c_str()), *selected_index);
}

fn anim_selector(
    rld: &mut RaylibDrawHandle,
    bounds: Rectangle,
    title: &CString,
    min: i32,
    max: i32,
    val: &mut i32,
) {
    let box_bound = rrect(
        bounds.x,
        bounds.y + 12.0,
        bounds.width,
        bounds.height - 12.0,
    );
    let gui_bound = rrect(
        bounds.x,
        bounds.y + 28.0,
        bounds.width,
        bounds.height - 28.0,
    );
    rld.gui_group_box(box_bound, Some(title.as_c_str()));
    *val = rld.gui_scroll_bar(gui_bound, *val, min, max);
}

fn anim_frame(
    rld: &mut RaylibDrawHandle,
    spritesheet: &Texture2D,
    width: i32,
    height: i32,
    side: i32,
    subimage: i32,
    pos: Vector2,
) {
    let spr_rect = rrect(width * subimage, height * side, width, height);

    rld.draw_texture_rec(spritesheet, spr_rect, pos, Color::WHITE);
}

#[derive(Debug)]
struct CreatedObject {
    conf: ObjectConfig,
    image_name: String,
}

fn main() {
    let mut scr_w = 640;
    let mut scr_h = 480;

    let (mut handle, thread) = raylib::init()
        .title("NIDS2 Object Creator")
        .size(scr_w, scr_h)
        .resizable()
        .build();

    handle.set_target_fps(60);
    handle.gui_load_style(Some(rstr!("candy.rgs")));

    let mut object_mode = false;
    let mut spritesheet = handle
        .load_texture_from_image(&thread, &Image::gen_image_color(1, 1, Color::WHITE))
        .expect("Fucky");
    let mut obj = CreatedObject {
        conf: ObjectConfig::new(),
        image_name: String::new(),
    };

    let mut side_options_str: CString = CString::new("").expect("Uhhhhhhhh oops");
    let mut subimage_options_str: CString = CString::new("").expect("Uhhhhhhhh oops");
    let mut subimage_options = Vec::new();
    let mut side_options = Vec::new();
    let mut subimage = 0;
    let mut side = 0;
    let mut anim_speed = 0;
    let mut cur_subimg = 0;
    let mut err: Option<(String, i32)> = None;

    let font = handle
        .load_font(&thread, "fonts/Oxygen-Regular.ttf")
        .expect("Unable to load font!");
    handle.gui_set_font(&font);
    handle.gui_unlock();
    handle.gui_enable();
    handle.gui_set_style(
        GuiControl::DEFAULT,
        GuiDefaultProperty::TEXT_SIZE as i32,
        20,
    );

    let mut frame_count = 0;

    while !handle.window_should_close() {
        frame_count += 1;

        if handle.is_window_resized() {
            scr_w = handle.get_screen_width();
            scr_h = handle.get_screen_height();
        }

        if handle.is_file_dropped() {
            object_mode = true;
            let fnames = handle.get_dropped_files();
            let fname = fnames.first().expect("Error getting dropped file name!");
            let img = Image::load_image(fname)
                .expect("Unable to load dropped image or dropped image is not an image...");
            spritesheet = handle
                .load_texture_from_image(&thread, &img)
                .expect("Unable to create texture from image!");
            handle.clear_dropped_files();
            obj.image_name = fname.clone();
            obj.conf = ObjectConfig::new();

            side_options =
                divisors_bar(spritesheet.height()).expect("Unable to get divisors for spritesheet");
            side_options_str = div_to_cstr(&side_options);
            subimage_options =
                divisors_bar(spritesheet.width()).expect("Unable to get divisors for spritesheet");
            subimage_options_str = div_to_cstr(&subimage_options);

            side = 0;
            subimage = 0;
            cur_subimg = 0;
            anim_speed = 0;
        }

        let k = handle.get_key_pressed();
        let mut d = handle.begin_drawing(&thread);

        d.clear_background(Color::SKYBLUE);
        if !object_mode {
            draw_text_centered(
                &mut d,
                // &font,
                "Drop a PNG File to start creating an object!",
                scr_w / 2,
                scr_h / 2,
                24,
                Color::BLACK,
            );
        } else {
            // draw texture appropriately to fit on screen.
            let mut sprsht_rec = rrect(0, 0, spritesheet.width(), spritesheet.height());
            let src_rect = sprsht_rec;
            if src_rect.width > (scr_w/2) as f32 {
                sprsht_rec.height *= (scr_w/2) as f32 / sprsht_rec.width;
                sprsht_rec.width = (scr_w/2) as f32;
            }
            if sprsht_rec.height > (scr_h/2) as f32{
                sprsht_rec.width *= (scr_h/2) as f32 / sprsht_rec.height;
                sprsht_rec.height = (scr_h/2) as f32;
            }
            d.draw_texture_pro(&spritesheet, src_rect, sprsht_rec, rvec2(0,0), 0.0, Color::WHITE);
            

            d.gui_panel(Rectangle {
                x: (scr_w / 2) as f32,
                y: 0 as f32,
                width: (scr_w / 2) as f32,
                height: scr_h as f32,
            });
            advanced_input(
                &mut d,
                &k,
                &font,
                rrect(scr_w / 2, 0, scr_w / 2, 64),
                &mut obj.conf.name,
                &"Enter The Object Name".to_string(),
            );
            advanced_input(
                &mut d,
                &k,
                &font,
                rrect(scr_w / 2, 64, scr_w / 2, 64),
                &mut obj.conf.category,
                &"Enter The Object Category".to_string(),
            );
            side_selector(
                &mut d,
                rrect(scr_w / 2, 128, scr_w / 2, 64),
                &side_options_str,
                &CString::new("Select number of sides").unwrap(),
                &mut side,
            );
            side_selector(
                &mut d,
                rrect(scr_w / 2, 194, scr_w / 2, 64),
                &subimage_options_str,
                &CString::new("Select number of subimages").unwrap(),
                &mut subimage,
            );
            anim_selector(
                &mut d,
                rrect(scr_w / 2, 258, scr_w / 2, 64),
                &CString::new("Select animation speed").unwrap(),
                0,
                64,
                &mut anim_speed,
            );

            d.draw_text_ex(
                &font,
                format!(
                    "Image Size: {} px by {} px",
                    spritesheet.width(),
                    spritesheet.height()
                )
                .as_str(),
                rvec2(0, scr_h - 32),
                24.0,
                1.0,
                Color::BLACK,
            );
            {
                let spr_w = spritesheet.width() / subimage_options.get(subimage as usize).unwrap();
                let spr_h = spritesheet.height() / side_options.get(side as usize).unwrap();
                if anim_speed > 0 && frame_count % anim_speed == 0 {
                    cur_subimg += 1;
                    if cur_subimg == *subimage_options.get(subimage as usize).unwrap() {
                        cur_subimg = 0;
                    }
                }
                let pos = rvec2(scr_w / 4 * 3 - spr_w / 2, scr_h - spr_h);
                anim_frame(&mut d, &spritesheet, spr_w, spr_h, side, cur_subimg, pos);
            }

            if d.gui_button(
                rrect(scr_w / 2, 322, scr_w / 2, 64),
                Some(CString::new("Save and Exit").unwrap().as_c_str()),
            ) {
            
                let path = format!("obj/{}", obj.conf.name);
                if fs::read_dir(path).is_err() {
                    obj.conf.id = get_next_id();
                    let spr_w = spritesheet.width() / subimage_options.get(subimage as usize).unwrap();
                    let spr_h = spritesheet.height() / side_options.get(side as usize).unwrap();
                    obj.conf.dim = (spr_w, spr_h);
                    obj.conf.sides = *side_options.get(side as usize).unwrap();
                    obj.conf.img_per_side = *subimage_options.get(subimage as usize).unwrap();
                    if anim_speed > 0 {
                        obj.conf.image_speed = Some(anim_speed);
                    } else {
                        obj.conf.image_speed = None;
                    }
                    obj.conf.default_b_box = None;

                    let path = format!("obj/{}", obj.conf.name);
                    let _ = fs::DirBuilder::new().create(&path);
                    let toml = toml::to_string(&obj.conf).unwrap();
                    let mut file = fs::File::create(path.clone() + "/obj.toml").unwrap();
                    let _ = file.write(toml.as_bytes());
                    let _ = fs::copy(&obj.image_name, path.clone() + "/spr.png");
                    object_mode = false;
                }else{
                    err = Some(("Object With This Name Already Exists!".to_string(), frame_count));
                } 
            }

            if let Some((e, f)) = &err {
                let trans_frame = f + 180;
                let max_frame = trans_frame + 60;
                let fade;
                if *f >= trans_frame && *f < max_frame {
                    fade = -(trans_frame-f)as f32 / 60.0;
                }else{
                    fade = 1.0;
                }

                let color = Color::DARKPURPLE.fade(fade);
                if frame_count  < max_frame {
                    draw_text_centered(&mut d,/* &font, */ &e, scr_w/4 * 3, 386, 24, color);
                }else{
                    err = None; 
                }

            }
        }
    }
}
