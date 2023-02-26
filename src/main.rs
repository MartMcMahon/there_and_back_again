use std::f32::consts::PI;

use bevy::render::render_resource::PrimitiveTopology;
use bevy::{
    input::mouse::MouseWheel,
    prelude::*,
    reflect::{erased_serde::__private::serde::__private::de, TypeUuid},
    render::{
        mesh,
        render_resource::{AsBindGroup, ShaderRef},
    },
    sprite::MaterialMesh2dBundle,
    time::FixedTimestep,
    transform,
};
use bevy_flycam::{FlyCam, MovementSettings, NoCameraPlayerPlugin};
use bevy_obj::*;

mod debug_text;
use debug_text::{DebugText, DebugValue};

const SPEED_SCALE: f32 = 0.1;
// 6.6743 × 10-11
const GRAV_CONST: f32 = 6.7;
const CAMERA_DIST: f32 = 20.0;

#[derive(Component)]
struct MovesAround;
#[derive(Component, Default)]
struct Player {
    forward: Vec3,
    rot: f32,
    thrust: f32,
}
#[derive(Component, Default)]
struct Body {
    mass: f32,
    vel: Vec3,
}
#[derive(Component, Default)]
struct Planet {
    mass: f32,
}

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::hex("071f3c").unwrap()))
        .insert_resource(DebugValue::default())
        .add_plugins(DefaultPlugins)
        .add_plugin(NoCameraPlayerPlugin)
        .add_plugin(DebugText)
        .insert_resource(MovementSettings {
            ..Default::default()
        })
        .add_startup_system(setup)
        .add_system(keyboard_input)
        .add_system(scroll_events)
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(1.0 / 60.0))
                .with_system(calulate_grav)
                .with_system(move_bodies)
                .with_system(accelerate_player),
        )
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    // camera
    let camera = Camera3dBundle {
        transform: Transform::from_xyz(-2.0, 0.0, 0.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()
    };
    commands.spawn(camera);
    // .insert(FlyCam);

    // player
    commands
        .spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube {
                size: 0.6,
                ..default()
            })),
            material: materials.add(StandardMaterial {
                base_color: Color::Rgba {
                    red: 0.0,
                    green: 1.0,
                    blue: 0.66,
                    alpha: 1.0,
                },
                alpha_mode: AlphaMode::Blend,
                ..default()
            }),
            transform: Transform::from_xyz(CAMERA_DIST, 5.0, 0.0),
            ..default()
        })
        .insert(Player {
            forward: Vec3::Y,
            rot: 0.0,
            thrust: 0.0,
        })
        .insert(Body {
            mass: 0.1,
            vel: Vec3::ZERO,
        });

    // things
    commands
        .spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Icosphere::default())),
            material: materials.add(StandardMaterial {
                base_color: Color::RED,
                alpha_mode: AlphaMode::Blend,
                ..default()
            }),
            transform: Transform::from_xyz(CAMERA_DIST, -2.0, 3.0),
            ..default()
        })
        .insert(Planet { mass: 100.0 });
    // commands
    //     .spawn(PbrBundle {
    //         mesh: meshes.add(Mesh::from(shape::Cube::default())),
    //         material: materials.add(StandardMaterial {
    //             base_color: Color::RED,
    //             alpha_mode: AlphaMode::Blend,
    //             ..default()
    //         }),
    //         transform: Transform::from_xyz(CAMERA_DIST, 2.0, 0.0),
    //         ..default()
    //     })
    //     .insert(Body {
    //         mass: 1.0,
    //         vel: Vec3 {
    //             x: 0.0,
    //             y: 0.5,
    //             z: 0.5,
    //         },
    //     });

    // light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });
}

fn scroll_events(
    mut scroll_evr: EventReader<MouseWheel>,
    mut query: Query<(&Camera, &mut Transform)>,
) {
    let (_camera, mut transform) = query.single_mut();
    use bevy::input::mouse::MouseScrollUnit;

    for ev in scroll_evr.iter() {
        match ev.unit {
            MouseScrollUnit::Line => {
                transform.translation.x += ev.y;
            }
            MouseScrollUnit::Pixel => {}
        }
    }
}

fn calulate_grav(
    mut debug_value: ResMut<DebugValue>,
    mut body_query: Query<(&mut Body, &mut Transform), With<Body>>,
    mut planet_query: Query<(&Planet, &Transform), Without<Body>>,
) {
    let (planet, planet_transform) = planet_query.get_single().unwrap();
    for (mut body, body_transform) in body_query.iter_mut() {
        let distance = body_transform
            .translation
            .distance(planet_transform.translation);
        if distance > 1.0 {
            let f = (0.5 * body.mass)
                / body_transform
                    .translation
                    .distance_squared(planet_transform.translation);
            let g = (planet_transform.translation - body_transform.translation).normalize() * f;

            body.vel += g;
            debug_value.0 = body_transform.translation;
        } else {
            body.vel = Vec3::ZERO;
        }
    }
}

fn accelerate_player(mut query: Query<(&mut Body, &mut Transform, &mut Player)>) {
    let (mut body, mut transform, mut player) = query.single_mut();
    // player.forward = Vec3::from
    transform.rotation = Quat::from_rotation_x(player.rot);
    player.forward = transform.rotation.mul_vec3(Vec3::Y);
    body.vel += player.thrust * player.forward;
    // transform.translation += player.thrust * player.forward;
}

fn move_bodies(mut query: Query<(&Body, &mut Transform)>) {
    for (body, mut transform) in query.iter_mut() {
        transform.translation.z += body.vel.z * SPEED_SCALE;
        transform.translation.y += body.vel.y * SPEED_SCALE;
    }
}

fn keyboard_input(keys: Res<Input<KeyCode>>, mut query: Query<&mut Player, With<Body>>) {
    let mut player = query.single_mut();
    if keys.just_pressed(KeyCode::Space) {
        // Space was pressed
    }
    if keys.just_released(KeyCode::W) || keys.just_released(KeyCode::S) {
        player.thrust = 0.0;
    }
    if keys.just_released(KeyCode::A) || keys.just_released(KeyCode::D) {
        // body.rot = 0.0;
    }
    if keys.pressed(KeyCode::W) {
        // body. += -0.01;
        player.thrust += 0.01;
    }
    if keys.pressed(KeyCode::A) {
        player.rot += -0.1;
    }
    if keys.pressed(KeyCode::S) {
        player.thrust += -0.01;
    }
    if keys.pressed(KeyCode::D) {
        player.rot += 0.1;
    }

    // we can check multiple at once with `.any_*`
    if keys.any_pressed([KeyCode::LShift, KeyCode::RShift]) {
        // Either the left or right shift are being held down
    }
    if keys.any_just_pressed([KeyCode::Delete, KeyCode::Back]) {
        // Either delete or backspace was just pressed
    }
}
