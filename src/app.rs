use bevy::prelude::*;
use super::record;

#[derive(Resource, Debug)] 
pub struct MyApp{
    pub scale: f32,
    pub is_first_frame: bool,
    pub dynamic_image: image::DynamicImage,
    pub plane_start_pos: Pos,
    pub plane_end_pos: Pos,
    pub mouse_drag_state: MouseDragState,
    pub record: record::RecordManager,
    pub screenshot_state: ScreenshotState,
}
impl Default for MyApp{
    fn default() -> Self{
        MyApp{
            scale: 1.0,
            is_first_frame: true,
            dynamic_image: image::DynamicImage::default(),
            plane_start_pos: Pos::new(0.0, 0.0),
            plane_end_pos: Pos::new(0.0, 0.0),
            mouse_drag_state: MouseDragState::Idle,
            record: record::RecordManager::default(),
            screenshot_state: ScreenshotState::Idle,
        }
    }
}

#[derive(Component, Debug)] 
pub struct Root;

#[derive(Component, Debug)] 
pub struct EditPlane;
#[derive(Component, Debug)] 
pub struct BlackPlane;

#[derive(PartialEq, Debug)] 
pub enum MouseDragState{
    Idle,
    Drag,
}

#[derive(PartialEq, Debug)] 
pub enum ScreenshotState{
    Idle,
    ScreenShot,
    Cutout,
    ReplaceClipboard,
}


#[derive(Debug, Default)] 
pub struct Pos{
    pub x: f32,
    pub y: f32,
}
impl Pos{
    pub fn new(x: f32, y: f32) -> Pos{
        Pos{
            x: x,
            y: y,
        }
    }
}