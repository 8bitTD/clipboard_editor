use bevy::prelude::*;
use super::record;

#[derive(Resource, Debug)] 
pub struct MyApp {
    pub scale: f32, //画像の親のスケール値
    pub is_first_frame: bool, //最初のフレームに背景色を指定するための変数
    pub dynamic_image: image::DynamicImage, //ツール起動時にクリップボード画像を格納するための変数
    pub plane_start_pos: Pos, //黒い板のスタート位置
    pub plane_end_pos: Pos, //黒い板のエンド位置
    pub mouse_drag_state: MouseDragState, //マウスドラッグの判定
    pub record: record::RecordManager, //Undo, Redo管理
    pub screenshot_state: ScreenshotState, //スクリーンショットの状況管理
    pub is_local_save: bool, //スクリーンショットの画像をローカルに保存するかどうかの変数
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
            is_local_save: false,
        }
    }
}

#[derive(Component, Debug)] 
pub struct Root;

#[derive(Component, Debug)] 
pub struct GuidePlane;
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