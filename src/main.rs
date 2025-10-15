//#![windows_subsystem = "windows"]
use bevy::prelude::*;
mod app;
mod asset;
mod clipboard;
mod define;
mod record;

fn main() {
    let clipboard = arboard::Clipboard::new();
    let Ok(mut ctx) = clipboard else {return};
    let Ok(img) = ctx.get_image() else {return};
    let Some(cb_img) = image::RgbaImage::from_vec(img.width as u32, img.height as u32, img.bytes.to_vec()) else {return};
    let di = image::DynamicImage::from(cb_img);
    set_exec(di);
}

fn set_exec(di: image::DynamicImage){
    let mut ap = app::MyApp::default();
    ap.dynamic_image = di;
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin{
            primary_window: Some(Window {
                mode: bevy::window::WindowMode::BorderlessFullscreen(bevy::window::MonitorSelection::Current),
                position: WindowPosition::Automatic,
                present_mode: bevy::window::PresentMode::AutoNoVsync, 
                prevent_default_event_handling: false,
                fit_canvas_to_parent: true,
                decorations: false,
                transparent: true,
                ..default()
            }),
            exit_condition: bevy::window::ExitCondition::OnAllClosed,
            close_when_requested: true,
            ..default()
        },
        ))
        .insert_resource(ap)
        .add_systems(Startup, asset::setup_system)
        .add_systems(Update,(
            asset::set_background_color,
            asset::root_scale,
            asset::guide_plane,
            asset::create_black_plane,
            asset::keyboard_shortcut,
            asset::mouse_input,
            asset::take_screenshot,
            record::update_record,
        ))
        .run();
}