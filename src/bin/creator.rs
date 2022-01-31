#![allow(unused_imports)]

use nids2::game::*;
use nids2::util::*;
use raylib::consts::KeyboardKey::*;
use raylib::prelude::*;
use std::ffi::{CStr, CString};
use std::fs;
use std::io::prelude::*;
use std::iter::*;
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

// Scales a rectangle to the given width and height. Preserves aspect
// ratio. Scales width before height. Scales both larger and smaller.
pub fn scale_to(src: &mut Rectangle, width: f32, height: f32) {
    if src.width < width {
        let factor = width / src.width;
        src.height *= factor;
        src.width = width;
    } else if src.height < height {
        let factor = height / src.height;
        src.width *= factor;
        src.height = height;
    }

    if src.width > width {
        let factor = width / src.width;
        src.height *= factor;
        src.width = width;
    } else if src.height > height {
        let factor = height / src.height;
        src.width *= factor;
        src.height = height;
    }
}

pub fn center_in(src: &mut Rectangle, center: Rectangle) {
    src.x = center.x + center.width / 2.0 - src.width / 2.0;
    src.y = center.y + center.height / 2.0 - src.height / 2.0;
}

#[derive(Debug)]
struct CreatedObject {
    conf: ObjectConfig,
    image_name: String,
}

fn find_obj(
    name: &str,
    vec: &Vec<std::sync::Arc<(Texture2D, ObjectConfig)>>,
) -> Option<std::sync::Arc<(Texture2D, ObjectConfig)>> {
    for tup in vec.iter() {
        let conf = &tup.1;
        if name == conf.name.as_str() {
            return Some(tup.clone());
        }
    }
    None
}

fn find_i32(target: i32, vec: &Vec<i32>) -> Option<usize> {
    for i in 0..vec.len() {
        if target == *vec.get(i).unwrap() {
            return Some(i);
        }
    }

    None
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

    nids2::game::init(&mut handle, &thread);
    color_init(&mut handle);

    let mut object_mode = false;
    let mut bounding_box_mode = false;
    let mut obj_preview_mode = false;
    let mut edit_existing_object = false;
    let mut animating = false;

    let mut spritesheet = handle
        .load_texture_from_image(&thread, &Image::gen_image_color(1, 1, Color::WHITE))
        .expect("Fucky");
    let mut obj = CreatedObject {
        conf: ObjectConfig::new(),
        image_name: String::new(),
    };

    let all_obj = get_all_objects();

    let mut side_options_str: CString = CString::new("").expect("Uhhhhhhhh oops");
    let mut subimage_options_str: CString = CString::new("").expect("Uhhhhhhhh oops");
    let mut subimage_options = Vec::new();
    let mut side_options = Vec::new();
    let mut subimage = 0;
    let mut side = 0;
    let mut anim_speed = 0;
    let mut cur_subimg = 0;
    let mut edit_object = 0;
    let mut top_item_index = 0;
    let mut preview_subimage = 0;

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

            side = 0; // find_i32(obj.conf.sides, &side_options).unwrap_or(0);
            subimage = 0; //find_i32(obj.conf.img_per_side, &subimage_options).unwrap_or(0);
            cur_subimg = 0;
            anim_speed = 0;
        }

        let k = handle.get_key_pressed();
        let mut d = handle.begin_drawing(&thread);

        d.clear_background(Color::SKYBLUE);

        /* OBJECT SELECTION / DRAG-DROP MENU
         * Presents the option of picking from the set
         * of pre-existing object, or to drag in a PNG
         * file and create a new object from scratch.
         * Loads existing objects statically using the
         * same mechanisms as the game does.
         * */
        if !object_mode {
            let menu_rect = rrect(16, 16, scr_w - 32, scr_h - 32);
            // Draw Explanation of Screen
            draw_text_centered(
                &mut d,
                &font,
                "Drag and Drop a PNG file onto the window to create a new object!",
                scr_w / 2,
                64,
                24,
                Color::BLACK,
            );
            draw_text_centered(
                &mut d,
                &font,
                "Or, Select An Existing Object to Edit Using Your Mouse!",
                scr_w / 2,
                96,
                24,
                Color::BLACK,
            );

            // Get all existing objects by collecting the read_dir iterator
            let mut items = std::fs::read_dir("obj/")
                .expect("Unable to read obj/")
                .map(|res| {
                    res.map(|e| {
                        String::from(
                            (e.path().strip_prefix(std::path::Path::new("obj/")).unwrap())
                                .to_str()
                                .unwrap(),
                        )
                    })
                })
                .collect::<Result<Vec<_>, std::io::Error>>()
                .expect("Unable to collect obj iter");
            items.sort();

            // Existing Object Selection Scroll Bar
            if ds_scroll_selection(
                &mut d,
                &font,
                rrect(
                    menu_rect.x + 16.0,
                    128.0,
                    240,
                    (menu_rect.y + menu_rect.height) - 128.0,
                ),
                &items,
                &mut edit_object,
                &mut top_item_index,
            ) {
                obj_preview_mode = true;
            }

            /* DRAW OBJECT PREVIEW AND SUBIMAGE VIEWER
             * allows the user to see their selected
             * object scaled up to 320 x 320 pixels. They
             * can select different subimages and view them
             * too.
             * TODO: Animate animated objects if necessary.
             * TODO: Provide a toggle for animation? */
            if obj_preview_mode {
                // Only preview if we are on a valid object.
                // TODO: when `if let Some(_) = Option && condition` expressions are released,
                // refactor section
                if let Some(preview_obj) =
                    find_obj(items.get(edit_object as usize).unwrap(), &all_obj)
                {
                    let mut image_rect = rrect(0, 0, preview_obj.1.dim.0, preview_obj.1.dim.1);
                    let src_rect = rrect(
                        preview_subimage as f32 * image_rect.width,
                        0,
                        image_rect.width,
                        image_rect.height,
                    );

                    let frame_rect = rrect(menu_rect.x + 260.0, 128, 320, 320);
                    scale_to(&mut image_rect, 320.0, 320.0);
                    center_in(&mut image_rect, frame_rect);

                    // Frame
                    ds_rounded_rectangle_lines(&mut d, frame_rect, 0.05, 16, 3);
                    // Sprite
                    d.draw_texture_pro(
                        &preview_obj.0,
                        src_rect,
                        image_rect,
                        rvec2(0, 0),
                        0.0,
                        Color::WHITE,
                    );

                    // Warning Messsage
                    draw_text_centered(
                        &mut d,
                        &font,
                        "Image Is Scaled To Fit In The Frame",
                        frame_rect.x as i32 + 160,
                        (frame_rect.y + frame_rect.height) as i32 + 32,
                        16,
                        Color::BLACK,
                    );
                    draw_text_centered(
                        &mut d,
                        &font,
                        "NOT TO REAL SIZE",
                        frame_rect.x as i32 + 160,
                        (frame_rect.y + frame_rect.height) as i32 + 12,
                        16,
                        Color::BLACK,
                    );


                    // Previous Subimage and Next Subimage buttons
                    if ds_rounded_button_centered(
                        &mut d,
                        &font,
                        rrect(
                            frame_rect.x + (1.0 / 8.0) * frame_rect.width,
                            frame_rect.y + frame_rect.height + 12.0,
                            48,
                            16,
                        ),
                        Some("prev"),
                        preview_obj.1.img_per_side > 1 && !animating,
                    )
                    .0
                    {
                        preview_subimage -= 1;
                        if preview_subimage < 0 {
                            preview_subimage = preview_obj.1.img_per_side - 1;
                        }
                    }

                    if ds_rounded_button_centered(
                        &mut d,
                        &font,
                        rrect(
                            frame_rect.x + (7.0 / 8.0) * frame_rect.width,
                            frame_rect.y + frame_rect.height + 12.0,
                            48,
                            16,
                        ),
                        Some("next"),
                        preview_obj.1.img_per_side > 1 && !animating,
                    )
                    .0
                    {
                        preview_subimage += 1;
                        if preview_subimage >= preview_obj.1.img_per_side {
                            preview_subimage = 0;
                        }
                    }

                    // If we need to animate, draw animation toggle and animate.
                    if let Some(speed) = preview_obj.1.image_speed {
                        // animating = d.gui_toggle(
                        //     rrect(
                        //         frame_rect.x + 160. - 92.,
                        //         frame_rect.y + frame_rect.height + 88.,
                        //         184,
                        //         24,
                        //     ),
                        //     Some(rstr!("toggle animation")),
                        //     animating,
                        // );
                        ds_draw_toggle_rounded_centered(&mut d,
                                               &font,
                                               Some("toggle animation"),
                                               rrect(frame_rect.x + frame_rect.width/2.,
                                                     frame_rect.y + frame_rect.height + 88.,
                                                     184,
                                                     24),
                                               &mut animating);

                        if animating && frame_count % speed == 0 {
                            preview_subimage += 1;
                            if preview_subimage >= preview_obj.1.img_per_side {
                                preview_subimage = 0;
                            }
                        }
                    }

                    // Object Select button
                    if ds_rounded_button_centered(
                        &mut d,
                        &font,
                        rrect(
                            frame_rect.x + frame_rect.width / 2.0,
                            frame_rect.y + frame_rect.height + 64.0,
                            184,
                            24,
                        ),
                        Some("LOAD AND EDIT OBJECT"),
                        true,
                    )
                    .0
                    {
                        // Select item and gtfo
                        object_mode = true;
                        obj_preview_mode = false;
                        edit_existing_object = true;

                        let fname = String::from("obj/")
                            + items.get(edit_object as usize).unwrap()
                            + "/spr.png";

                        spritesheet = unsafe { Texture2D::from_raw(preview_obj.0.clone()) };
                        obj.image_name = fname;
                        obj.conf = preview_obj.1.clone();

                        side_options = divisors_bar(spritesheet.height())
                            .expect("Unable to get divisors for spritesheet");
                        side_options_str = div_to_cstr(&side_options);
                        subimage_options = divisors_bar(spritesheet.width())
                            .expect("Unable to get divisors for spritesheet");
                        subimage_options_str = div_to_cstr(&subimage_options);

                        side = find_i32(obj.conf.sides, &side_options).unwrap_or(0) as i32;
                        subimage =
                            find_i32(obj.conf.img_per_side, &subimage_options).unwrap_or(0) as i32;
                        cur_subimg = 0;
                        anim_speed = obj.conf.image_speed.unwrap_or(0);
                    }
                }
            }
        /* DRAW GUI AND INTERFACE */
        } else {
            // draw texture appropriately to fit on screen.
            let mut sprsht_rec = rrect(0, 0, spritesheet.width(), spritesheet.height());
            let src_rect = sprsht_rec;
            scale_to(&mut sprsht_rec, (scr_w / 2) as f32, (scr_h / 2) as f32);
            d.draw_texture_pro(
                &spritesheet,
                src_rect,
                sprsht_rec,
                rvec2(0, 0),
                0.0,
                Color::WHITE,
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

            

            // DRAW IMPORTANT CONTROLS
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

            /* ANIMATE SPRITE PREVIEW */
            {
                let spr_w = spritesheet.width() / subimage_options.get(subimage as usize).unwrap();
                let spr_h = spritesheet.height() / side_options.get(side as usize).unwrap();
                if anim_speed > 0 && frame_count % anim_speed == 0 {
                    cur_subimg += 1;
                    if cur_subimg == *subimage_options.get(subimage as usize).unwrap() {
                        cur_subimg = 0;
                    }
                }
                let spr_rect = rrect(spr_w * cur_subimg, spr_h * side, spr_w, spr_h);
                let mut draw_rect = spr_rect.clone();
                scale_to(&mut draw_rect, (scr_w/2) as f32, 290.0);
                draw_rect.x = (scr_w as f32 * 0.75) - (draw_rect.width / 2.0);
                draw_rect.y = scr_h as f32 - draw_rect.height;
                d.draw_texture_pro(&spritesheet, spr_rect, draw_rect, rvec2(0,0), 0.0, Color::WHITE);
                // anim_frame(&mut d, &spritesheet, spr_w, spr_h, side, cur_subimg, pos);
            }
            

            /* DRAW AND EXECUTE SAVE AND EXIT */
            if !bounding_box_mode
                && d.gui_button(
                    rrect(scr_w / 2, 386, scr_w / 2, 64),
                    Some(CString::new("Save and Exit").unwrap().as_c_str()),
                )
            {
                let path = format!("obj/{}", obj.conf.name);
                let new_obj = fs::read_dir(path).is_err();

                let spr_w =
                    spritesheet.width() / subimage_options.get(subimage as usize).unwrap();
                let spr_h = spritesheet.height() / side_options.get(side as usize).unwrap();
                obj.conf.dim = (spr_w, spr_h);
                obj.conf.sides = *side_options.get(side as usize).unwrap();
                obj.conf.img_per_side = *subimage_options.get(subimage as usize).unwrap();
                if anim_speed > 0 {
                    obj.conf.image_speed = Some(anim_speed);
                } else {
                    obj.conf.image_speed = None;
                }
                
                let path = format!("obj/{}", obj.conf.name);
                
                if new_obj {
                    obj.conf.id = get_next_id();
                    let _ = fs::DirBuilder::new().create(&path);
                    let _ = fs::copy(&obj.image_name, path.clone() + "/spr.png");
                }

                let toml = toml::to_string(&obj.conf).unwrap();
                let mut file = fs::File::create(path.clone() + "/obj.toml").unwrap();
                let _ = file.write(toml.as_bytes());
                object_mode = false;
            }
            
            /* DRAW ERROR MESSAGES, IF ANY */
            if let Some((e, f)) = &err {
                let trans_frame = f + 180;
                let max_frame = trans_frame + 60;
                let fade;
                if *f >= trans_frame && *f < max_frame {
                    fade = -(trans_frame - f) as f32 / 60.0;
                } else {
                    fade = 1.0;
                }

                let color = Color::DARKPURPLE.fade(fade);
                if frame_count < max_frame {
                    draw_text_centered(&mut d, &font, e, scr_w / 4 * 3, 450, 24, color);
                } else {
                    err = None;
                }
            }
            
            /* BOUNDING BOX BUTTON */
            if !bounding_box_mode
                && d.gui_button(
                    rrect(scr_w / 2, 322, scr_w / 2, 64),
                    Some(CString::new("Create Bounding Box").unwrap().as_c_str()),
                )
            {
                bounding_box_mode = true;
                let spr_w = spritesheet.width() / subimage_options.get(subimage as usize).unwrap();
                let spr_h = spritesheet.height() / side_options.get(side as usize).unwrap();

                if obj.conf.default_b_box.is_none() {
                    obj.conf.default_b_box = Some((0, 0, 1, 1));
                    obj.conf.dim = (spr_w, spr_h);
                    obj.conf.sides = *side_options.get(side as usize).unwrap();
                    obj.conf.img_per_side = *subimage_options.get(subimage as usize).unwrap();
                }
            }
            
            /* DRAW BOUNDING BOX EDITOR OVER REST OF SCREEN */
            if bounding_box_mode {
                let mode_rect = rrect(
                    scr_w as f32 * 0.25,
                    scr_h as f32 * 0.25,
                    scr_w / 2,
                    scr_h / 2,
                );
                ds_rounded_rectangle(&mut d, mode_rect, 0.5, 4);
                let (x, y, width, height): (&mut i32, &mut i32, &mut i32, &mut i32) = obj
                    .conf
                    .default_b_box
                    .as_mut()
                    .map(|tup| (&mut tup.0, &mut tup.1, &mut tup.2, &mut tup.3))
                    .unwrap();

                let src_rect = rrect(0, 0, obj.conf.dim.0, obj.conf.dim.1);
                let mut spr_rect = rrect(
                    mode_rect.x + mode_rect.width / 2.0,
                    mode_rect.y + mode_rect.height / 2.0,
                    src_rect.width,
                    src_rect.height,
                );

                let target_w = mode_rect.width / 2.0;
                let target_h = mode_rect.height / 2.0;

                if spr_rect.width < (target_w - 60.0) {
                    spr_rect.height *= target_w / spr_rect.width;
                    spr_rect.width = target_w;
                } else if spr_rect.height < (target_h - 60.0) {
                    spr_rect.width *= target_h / spr_rect.height;
                    spr_rect.height = target_h;
                }

                if spr_rect.width > (target_w) {
                    spr_rect.height *= target_w / spr_rect.width;
                    spr_rect.width = target_w;
                } else if spr_rect.height > (target_h) {
                    spr_rect.width *= target_h / spr_rect.height;
                    spr_rect.height = target_h;
                }

                spr_rect.x -= spr_rect.width / 2.0;
                spr_rect.y -= spr_rect.height / 2.0;

                d.draw_texture_pro(
                    &spritesheet,
                    src_rect,
                    spr_rect,
                    rvec2(0, 0),
                    0.0,
                    Color::WHITE,
                );

                ds_draw_slider_centered(
                    &mut d,
                    &font,
                    "Modify BBox X",
                    rvec2(
                        mode_rect.x + mode_rect.width * (2.0 / 8.0),
                        mode_rect.y + mode_rect.height * (1.0 / 3.0),
                    ),
                    mode_rect.width / 4.0,
                    20.0,
                    x,
                    0.0,
                    src_rect.width - *width as f32,
                    true,
                );
                ds_draw_slider_centered(
                    &mut d,
                    &font,
                    "Modify BBox Y",
                    rvec2(
                        mode_rect.x + mode_rect.width * (2.0 / 8.0),
                        mode_rect.y + mode_rect.height * (2.0 / 3.0),
                    ),
                    mode_rect.width / 4.0,
                    20.0,
                    y,
                    0.0,
                    src_rect.height - *height as f32,
                    true,
                );
                ds_draw_slider_centered(
                    &mut d,
                    &font,
                    "Modify BBOx WIDTH",
                    rvec2(
                        mode_rect.x + mode_rect.width * (6.0 / 8.0),
                        mode_rect.y + mode_rect.height * (1.0 / 3.0),
                    ),
                    mode_rect.width / 4.0,
                    20.0,
                    width,
                    0.0,
                    src_rect.width - *x as f32 + 1.0,
                    true,
                );
                ds_draw_slider_centered(
                    &mut d,
                    &font,
                    "Modify BBOx HEIGHT",
                    rvec2(
                        mode_rect.x + mode_rect.width * (6.0 / 8.0),
                        mode_rect.y + mode_rect.height * (2.0 / 3.0),
                    ),
                    mode_rect.width / 4.0,
                    20.0,
                    height,
                    0.0,
                    src_rect.height - *y as f32 + 1.0,
                    true,
                );

                draw_text_centered(
                    &mut d,
                    &font,
                    format!("({}, {}) {}px X {}px", *x, *y, *width, *height).as_str(),
                    (mode_rect.x + mode_rect.width / 2.0) as i32,
                    (mode_rect.y + 45.0) as i32,
                    24,
                    Color::BLACK,
                );

                let (exit_bbox, _) = ds_rounded_button_centered(
                    &mut d,
                    &font,
                    rrect(
                        mode_rect.x + mode_rect.width / 2.0,
                        mode_rect.y + mode_rect.height * 0.875,
                        120,
                        30,
                    ),
                    Some("Exit BBox Editor"),
                    true,
                );

                let mut bbox = rrect(*x, *y, *width, *height);
                let factor = spr_rect.width / src_rect.width;
                bbox.x *= factor;
                bbox.x += spr_rect.x;
                bbox.y *= factor;
                bbox.y += spr_rect.y;
                bbox.width *= factor;
                bbox.height *= factor;
                d.draw_rectangle_lines_ex(bbox, 1, Color::BLACK);

                if exit_bbox {
                    bounding_box_mode = false;
                }
            }
        }
    }
}
