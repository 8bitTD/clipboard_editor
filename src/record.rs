use bevy::prelude::*;
use super::app;

#[derive(Debug, Clone)] 
pub struct Plane{
    pub pos_x: f32,
    pub pos_y: f32,
    pub scale_x: f32,
    pub scale_y: f32
}
impl Plane{
    pub fn new(px: f32, py: f32, sx: f32, sy: f32) -> Plane{
        Plane { pos_x: px, pos_y: py, scale_x: sx, scale_y: sy }
    } 
}

#[derive(Debug, Clone)]
pub struct Record{
    pub planes: Vec<Plane>,
}
impl Record{
    pub fn new(planes: Vec<Plane>) -> Record{
        Record { planes: planes }
    }
}

#[derive(Debug, Clone)]
pub struct RecordManager{
    pub is_undo: bool,//アンドゥー判定
    pub is_redo: bool,//リドゥー判定
    pub is_add_record: bool,//レコードの追加判定
    pub record_index: i32,//現在のレコードのindex
    pub records: Vec<Record>,//レコードのヒストリー
}
impl Default for RecordManager  {
    fn default() -> RecordManager {
        RecordManager {
            is_undo: false,
            is_redo: false,
            is_add_record: false,
            record_index: -1,
            records: Vec::new(),
        }
    }
} 


pub fn update_record(
    mut commands: Commands,
    mut ap: ResMut<app::MyApp>,
    meshes: Query<Entity, With<app::BlackPlane>>,
    mut mesh: ResMut<Assets<Mesh>>,
    mouse_button_input: Res<ButtonInput<MouseButton>>,
    root: Single<Entity, (With<app::Root>, Without<Sprite>)>,
    mut materials: ResMut<Assets<ColorMaterial>>,
){
    let mut is_change = false;
    if ap.record.is_add_record && !mouse_button_input.pressed(MouseButton::Left) {
        is_change = true;
        ap.record.is_add_record = false;
    }
    if ap.record.is_undo{
        if ap.record.record_index > -1 {
            ap.record.record_index -= 1;
            is_change = true;
        }else{
            ap.record.record_index = -1;
        }
        ap.record.is_undo = false;
    }
    if ap.record.is_redo{
        if ap.record.record_index == -1{
            ap.record.record_index = 0;
        }else if ap.record.record_index  < ap.record.records.len() as i32 -1{
            ap.record.record_index += 1;
            is_change = true;
        }
        ap.record.is_redo = false;
    }
    if !is_change {return;}
    for e in meshes.iter(){
        commands.entity(e).despawn();
    }
    if ap.record.record_index == -1 {return;}
    let rcd = ap.record.records[ap.record.record_index as usize].clone();
    for p in rcd.planes.iter(){
        let plane = commands.spawn((
            Mesh2d(mesh.add(Rectangle::default())),
            MeshMaterial2d(materials.add(Color::from(bevy::color::palettes::basic::BLACK))),
            Transform::default().with_translation(Vec3::new(p.pos_x, p.pos_y, 10.0)).with_scale(Vec3::new(p.scale_x, p.scale_y, 1.0)),
            app::BlackPlane,
        )).id();
        commands.entity(*root).add_child(plane); 
    }
}