use bevy::prelude::*;

#[derive(Component)]
pub struct Soldier();

pub fn new(x: f32, y: f32) -> (Transform, Soldier){
    (
        Transform::from_xyz(x, y, 0.0),
        Soldier()
    )
}

fn move_soldier_system(mut soldiers: Query<&mut Transform, With<Soldier>>){

}