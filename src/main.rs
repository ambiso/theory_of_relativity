use bevy::prelude::*;

mod game_object;
mod physics;

use game_object::*;
use physics::*;

struct Materials {
    default_material: Handle<ColorMaterial>,
}

fn setup(mut commands: Commands,
    asset_server: Res<AssetServer>,
     mut materials: ResMut<Assets<ColorMaterial>>) {
    commands.spawn().insert(OrthographicCameraBundle::new_2d());
    commands.insert_resource(Materials {
        // default_material: materials.add(Color::rgb(1.0, 1.0, 0.0).into()),
        default_material: materials.add(asset_server.load("branding/icon.png").into())
    });
}

fn spawn_entities(mut commands: Commands, materials: Res<Materials>) {
    spawn_entity(&mut commands, &materials, Pos(-1.));
    // spawn_entity(&mut commands, &materials, Pos(1.));
}

fn spawn_entity(commands: &mut Commands, materials: &Res<Materials>, pos: Pos) {

    commands
        .spawn()
        .insert(GameObject)
        .insert(PhysicsObject)
        .insert(pos)
        .insert(SpriteBundle {
            material: materials.default_material.clone(),
            // sprite: Sprite::new(Vec2::new(100., 100.)),
            ..Default::default()
        });
}

fn main() {
    App::build()
        .add_startup_system(setup.system())
        .add_startup_stage("spawn_entities", SystemStage::single(spawn_entities.system()))
        .add_plugins(DefaultPlugins)
        .add_plugin(PhysicsPlugin)
        .run();
}
