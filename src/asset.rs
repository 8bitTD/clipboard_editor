use bevy::prelude::*;

use super::app;
use super::record;
use super::clipboard;
use super::define::common;

pub fn setup_system(
    mut commands: Commands, 
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    ap: Res<app::MyApp>, 
    mut images: ResMut<Assets<Image>>
) {
    commands.spawn(Camera2d);
    commands.spawn((
        Mesh2d(meshes.add(Rectangle::default())),
        MeshMaterial2d(materials.add(Color::from(bevy::color::Srgba::new(0.0, 0.5, 1.0, 0.5)))),
        Transform::default().with_translation(Vec3::new(0.0, 0.0, 10.0)).with_scale(Vec3::new(0.0, 0.0, 0.0)),
        app::GuidePlane,
    ));
    let size = bevy::render::render_resource::Extent3d{
        width: ap.dynamic_image.width(),
        height: ap.dynamic_image.height(),
        depth_or_array_layers: 1
    };
    let td = bevy::render::render_resource::TextureDimension::D2;
    let tf = bevy::render::render_resource::TextureFormat::Rgba8UnormSrgb;
    let rau = bevy::asset::RenderAssetUsages::default();
    let ig = Image::new(size, td, ap.dynamic_image.as_bytes().to_vec(), tf, rau);
    let image_handle: Handle<Image> = images.add(ig);
    commands.spawn((
        Transform::default().with_scale(Vec3::new(1.0, 1.0, 1.0)),
        GlobalTransform::default(),
        app::Root,
    )).with_children(|parent|{
        parent.spawn(
            Sprite::from_image(image_handle)
        );
    });
    
}

pub fn set_background_color(
    mut ap: ResMut<app::MyApp>,
    mut clear_color: ResMut<ClearColor>,
){
    if ap.is_first_frame {
        ap.is_first_frame = false;
        clear_color.0 = bevy::color::Color::srgba_u8(
            common::BACKGROUNDCOLOR[0], 
            common::BACKGROUNDCOLOR[1], 
            common::BACKGROUNDCOLOR[2], 
            common::BACKGROUNDCOLOR[3]
        );
    }
}

pub fn root_scale(//ルートのスケール変更
    mut root: Single<&mut Transform, With<app::Root>>,
    ap: Res<app::MyApp>,
){
    root.scale = Vec3::new(ap.scale, ap.scale, ap.scale);
}

pub fn guide_plane(//黒板配置前のガイド板表示
    ap: Res<app::MyApp>,
    mut plane: Single<&mut Transform, With<app::GuidePlane>>,
    q_windows: Query<&Window, With<bevy::window::PrimaryWindow>>,
){
    if q_windows.single().unwrap().cursor_position().is_none(){return;}
    if ap.mouse_drag_state != app::MouseDragState::Drag{
        plane.scale = Vec3::new(0.0, 0.0, 1.0);
    }else{
        let pos = q_windows.single().unwrap().cursor_position().unwrap();
        let width = q_windows.single().unwrap().width();
        let height = q_windows.single().unwrap().height();
        let end_pos_x = pos.x-(width * 0.5);
        let end_pos_y = pos.y-(height * 0.5);
        let px = (end_pos_x + ap.plane_start_pos.x) * 0.5;
        let py = (-end_pos_y - ap.plane_start_pos.y) * 0.5;
        let sx = (end_pos_x - ap.plane_start_pos.x).abs();
        let sy = (end_pos_y - ap.plane_start_pos.y).abs();
        plane.translation = Vec3::new(px, py, 20.0);
        plane.scale = Vec3::new(sx, sy, 1.0);
    }
}

pub fn mouse_input(//マウスホイールの回転で画像の縮小具合を調整
    accumulated_mouse_scroll: Res<bevy::input::mouse::AccumulatedMouseScroll>,
    mut ap: ResMut<app::MyApp>, 
){
    let delta = accumulated_mouse_scroll.delta;
    if delta.y != 0.0{
        let val = delta.y * 0.05;
        ap.scale -= val;
        if ap.scale > 1.0 {ap.scale = 1.0;}
        if ap.scale < 0.1 {ap.scale = 0.1;}
    }
}

pub fn take_screenshot(
    mut commands: Commands, 
    mut ap: ResMut<app::MyApp>, 
    mut clear_color: ResMut<ClearColor>
){
    if ap.screenshot_state == app::ScreenshotState::Idle{return;}
    if ap.screenshot_state == app::ScreenshotState::ScreenShot{//ウィンドウ内のスクリーンショットを撮影
        commands.spawn(bevy::render::view::screenshot::Screenshot::primary_window())
            .observe(bevy::render::view::screenshot::save_to_disk(std::path::Path::new(common::SCREENSHOT).to_path_buf()));
        ap.screenshot_state = app::ScreenshotState::Cutout;
    }else if ap.screenshot_state == app::ScreenshotState::Cutout{//必要な範囲の画像を切り抜く
        if !std::path::Path::new(common::SCREENSHOT).is_file(){ return; }
        else{
            let img = image::ImageReader::open(common::SCREENSHOT).unwrap().decode().unwrap();
            let size_x = (ap.dynamic_image.width() as f32 * ap.scale - (common::SCREENSHOTOFFSET*2.0)) as u32;
            let size_y = (ap.dynamic_image.height() as f32 * ap.scale - (common::SCREENSHOTOFFSET*2.0)) as u32;
            let start_x = ((img.width() as f32 * 0.5) - (size_x as f32 * 0.5) + common::SCREENSHOTOFFSET) as u32;
            let start_y = ((img.height() as f32 * 0.5) - (size_y as f32 * 0.5) + common::SCREENSHOTOFFSET) as u32;
            let cropped = img.crop_imm(start_x, start_y, size_x, size_y);
            cropped.save(common::SCREENSHOTCUTOUT).expect("画像の保存に失敗しました");
            ap.screenshot_state = app::ScreenshotState::ReplaceClipboard;
        }
    }else if ap.screenshot_state == app::ScreenshotState::ReplaceClipboard{//切り抜いた画像をクリップボードに置き換え
        if std::path::Path::new(common::SCREENSHOTCUTOUT).is_file(){ 
            if ap.is_local_save{ //ローカルに保存する場合
                let mut tmp_screenshot_path: String = dirs::home_dir().unwrap().as_os_str().to_str().unwrap().to_string();
                tmp_screenshot_path.push_str("\\Pictures\\Screenshots\\");
                let mut screenshot_path = tmp_screenshot_path.replace("\\","/");
                if !std::path::Path::new(&screenshot_path).is_dir(){Some(std::fs::create_dir_all(&screenshot_path));}
                let datetime = chrono::Utc::now().with_timezone(&chrono::FixedOffset::east_opt(9 * 3600).unwrap()).naive_local();
                let ymdhms = datetime.format(common::SCREENSHOTLOCALFILE).to_string();
                screenshot_path.push_str(&ymdhms);
                let _copy_res = std::fs::copy(common::SCREENSHOTCUTOUT, &screenshot_path);
                //if copy_res.is_ok(){
                //    std::process::Command::new("explorer.exe")
                //        .arg(format!("{}{}", "/select,", &screenshot_path.replace("/","\\"))).status().unwrap();
                //}
            }else{ //
                let img = image::ImageReader::open(common::SCREENSHOTCUTOUT).unwrap().decode().unwrap();
                let data = clipboard::gen_from_img(&img);
                let _res = clipboard_win::set_clipboard(clipboard_win::formats::Bitmap, data);
            }
            
            ap.screenshot_state = app::ScreenshotState::Idle;
            if std::path::Path::new(common::SCREENSHOT).is_file(){ let _ = std::fs::remove_file(common::SCREENSHOT); }
            if std::path::Path::new(common::SCREENSHOTCUTOUT).is_file(){ let _ = std::fs::remove_file(common::SCREENSHOTCUTOUT); }
            clear_color.0 = bevy::color::Color::srgba_u8(
                common::BACKGROUNDCOLOR[0], 
                common::BACKGROUNDCOLOR[1], 
                common::BACKGROUNDCOLOR[2], 
                common::BACKGROUNDCOLOR[3]
            );
        }
    }
}

pub fn keyboard_shortcut(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut ap: ResMut<app::MyApp>, 
    mut app_exit: MessageWriter<bevy::app::AppExit>,
    mut clear_color: ResMut<ClearColor>
){
    if keyboard_input.any_pressed([KeyCode::ControlLeft, KeyCode::ControlRight]) && 
        !keyboard_input.any_pressed([KeyCode::ShiftLeft, KeyCode::ShiftRight]) &&
        keyboard_input.just_pressed(KeyCode::KeyZ){//undo
            ap.record.is_undo = true;
    }
    if keyboard_input.any_pressed([KeyCode::ControlLeft, KeyCode::ControlRight]) && 
        !keyboard_input.any_pressed([KeyCode::ShiftLeft, KeyCode::ShiftRight]) &&
        keyboard_input.just_pressed(KeyCode::KeyY){//redo
            ap.record.is_redo = true;
    }
    if keyboard_input.just_pressed(KeyCode::Escape){//Escでアプリ終了
        app_exit.write(bevy::app::AppExit::Success);
    }
    if keyboard_input.just_pressed(KeyCode::Enter) && ap.screenshot_state == app::ScreenshotState::Idle{//Enterでoutput.png出力
        if std::path::Path::new(common::SCREENSHOT).is_file(){ let _ = std::fs::remove_file(common::SCREENSHOT); }
        if std::path::Path::new(common::SCREENSHOTCUTOUT).is_file(){ let _ = std::fs::remove_file(common::SCREENSHOTCUTOUT); }
        let clipboard = arboard::Clipboard::new();
        let Ok(mut ctx) = clipboard else {return};
        let _ = ctx.clear();
        ap.screenshot_state = app::ScreenshotState::ScreenShot;
        ap.is_local_save = false;
        clear_color.0 = bevy::color::Color::srgba_u8(0,0,0,0);
    }
    if keyboard_input.just_pressed(KeyCode::NumpadAdd) && ap.screenshot_state == app::ScreenshotState::Idle{
        if std::path::Path::new(common::SCREENSHOT).is_file(){ let _ = std::fs::remove_file(common::SCREENSHOT); }
        if std::path::Path::new(common::SCREENSHOTCUTOUT).is_file(){ let _ = std::fs::remove_file(common::SCREENSHOTCUTOUT); }
        let clipboard = arboard::Clipboard::new();
        let Ok(mut ctx) = clipboard else {return};
        let _ = ctx.clear();
        ap.screenshot_state = app::ScreenshotState::ScreenShot;
        ap.is_local_save = true;
        clear_color.0 = bevy::color::Color::srgba_u8(0,0,0,0);
    }
}

pub fn create_black_plane(
    mut ap: ResMut<app::MyApp>,
    mouse_button_input: Res<ButtonInput<MouseButton>>,
    q_windows: Query<&Window, With<bevy::window::PrimaryWindow>>,
){
    if q_windows.single().unwrap().cursor_position().is_none(){return;}
    if mouse_button_input.just_pressed(MouseButton::Left) && ap.mouse_drag_state == app::MouseDragState::Idle {
        let pos = q_windows.single().unwrap().cursor_position().unwrap();
        if pos.y > 24.0{
            ap.mouse_drag_state = app::MouseDragState::Drag;
            let width = q_windows.single().unwrap().width();
            let height = q_windows.single().unwrap().height();
            ap.plane_start_pos.x = pos.x-(width * 0.5);
            ap.plane_start_pos.y = pos.y-(height * 0.5);
        }
    }
    if mouse_button_input.just_released(MouseButton::Left) && ap.mouse_drag_state == app::MouseDragState::Drag {
        ap.mouse_drag_state = app::MouseDragState::Idle;
        let ratio = 1.0 / ap.scale;
        let pos = q_windows.single().unwrap().cursor_position().unwrap();
        let width = q_windows.single().unwrap().width();
        let height = q_windows.single().unwrap().height();
        ap.plane_end_pos.x = pos.x-(width * 0.5);
        ap.plane_end_pos.y = pos.y-(height * 0.5);
        ap.plane_start_pos.x *= ratio;
        ap.plane_start_pos.y *= ratio;
        ap.plane_end_pos.x *= ratio;
        ap.plane_end_pos.y *= ratio;
        let px = (ap.plane_end_pos.x + ap.plane_start_pos.x) * 0.5;
        let py = (-ap.plane_end_pos.y - ap.plane_start_pos.y) * 0.5;
        let sx = (ap.plane_end_pos.x - ap.plane_start_pos.x).abs();
        let sy = (ap.plane_end_pos.y - ap.plane_start_pos.y).abs();
        let plane = record::Plane::new(px, py, sx, sy);
        let mut record = match ap.record.record_index{
            -1 => record::Record::new(vec![]),
            _ => ap.record.records[ap.record.record_index as usize].clone()
        };
        if ap.record.records.len() > 0 && ap.record.record_index as usize != ap.record.records.len() -1{
            let sa = ap.record.records.len() -1 - ap.record.record_index as usize;
            for _ in 0..sa{
                ap.record.records.pop();
            }
        }
        ap.record.record_index += 1;
        record.planes.push(plane);
        ap.record.records.push(record);
        ap.record.is_add_record = true;
    }
}