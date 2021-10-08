use bevy::prelude::*;

mod game_object;
mod physics;

use game_object::*;
use physics::*;

struct Materials {
    default_material: Handle<ColorMaterial>,
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.insert_resource(Materials {
        // default_material: materials.add(Color::rgb(1.0, 1.0, 0.0).into()),
        default_material: materials.add(asset_server.load("branding/icon.png").into()),
    });
    commands
        .spawn()
        .insert(PrintTimer(Timer::from_seconds(1.0, true)));
}

fn spawn_entities(mut commands: Commands, materials: Res<Materials>) {
    spawn_entity(
        &mut commands,
        &materials,
        Position(Transform::from_translation(Vec3::new(-200., 0., 0.))),
    );
    spawn_entity(
        &mut commands,
        &materials,
        Position(Transform::from_translation(Vec3::new(200., 0., 0.))),
    );
}

fn spawn_entity(commands: &mut Commands, materials: &Res<Materials>, pos: Position) {
    let transform = pos.0.clone();
    commands
        .spawn()
        .insert(GameObject)
        .insert(PhysicsObject::default())
        .insert(pos)
        .insert_bundle(SpriteBundle {
            material: materials.default_material.clone(),
            sprite: Sprite::new(Vec2::new(100., 100.)),
            transform: transform,
            ..Default::default()
        });
}

pub struct PrintTimer(Timer);

fn tick(time: Res<Time>, sprites: Query<&Sprite>, mut query: Query<&mut PrintTimer>) {
    for mut timer in query.iter_mut() {
        timer.0.tick(time.delta());

        if timer.0.just_finished() {
            println!("Sprites: {}", sprites.iter().count(),);
        }
    }
}

fn main() {
    App::build()
        .add_startup_system(setup.system())
        .add_startup_stage(
            "spawn_entities",
            SystemStage::single(spawn_entities.system()),
        )
        .add_plugins(DefaultPlugins)
        .add_plugin(PhysicsPlugin)
        .add_system(tick.system().label("Tick"))
        .run();
}
