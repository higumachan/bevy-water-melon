use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;
use bevy_rapier2d::prelude::*;
use bevy_water_melon::fruits::Fruit;
use bevy_water_melon::fruits::Fruit::Grape;
use rapier2d::parry::query::contact::contact_support_map_support_map;
use std::collections::{HashMap, HashSet};

#[derive(Component)]
struct Player {}

#[derive(Component)]
struct Promote(Fruit);

#[derive(Resource)]
struct SpawnTimer {
    next_fruit_timer: Option<Timer>,
}

#[derive(Resource)]
struct CurrentFruit {
    fruit: Option<(Fruit, Entity)>,
}

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(
            0xF9 as f32 / 255.0,
            0xF9 as f32 / 255.0,
            0xFF as f32 / 255.0,
        )))
        .add_plugins((
            DefaultPlugins,
            RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(1000.0),
            RapierDebugRenderPlugin::default(),
        ))
        .insert_resource(SpawnTimer {
            next_fruit_timer: Some(Timer::from_seconds(1.0, TimerMode::Once)),
        })
        .insert_resource(CurrentFruit { fruit: None })
        .add_systems(Startup, (setup_graphics, setup_physics))
        .add_systems(Update, (player_movement, spawn_fruit))
        .add_systems(Update, display_events)
        .run();
}

fn spawn_fruit(
    time: Res<Time>,
    mut next_spawn_timer: ResMut<SpawnTimer>,
    mut current_fruit: ResMut<CurrentFruit>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut commands: Commands,
) {
    if let Some(timer) = next_spawn_timer.next_fruit_timer.as_mut() {
        if timer.tick(time.delta()).just_finished() {
            println!("Spawn fruit");
            let fruit = Fruit::Grape;
            let fruit_entity = commands
                .spawn((
                    MaterialMesh2dBundle {
                        mesh: meshes.add(shape::Circle::new(fruit.radius()).into()).into(),
                        material: materials.add(ColorMaterial::from(fruit.color())),
                        transform: Transform::from_xyz(0.0, 300.0, 0.0),
                        ..default()
                    },
                    Player {},
                ))
                .id();
            current_fruit.fruit = Some((fruit, fruit_entity));
        }
    }
}

fn player_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut player_info: Query<(&Player, &mut Transform)>,
    mut current_fruit: ResMut<CurrentFruit>,
    mut next_spawn_timer: ResMut<SpawnTimer>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    for (player, mut transform) in &mut player_info {
        let left = keyboard_input.any_pressed([KeyCode::A, KeyCode::Left]);
        let right = keyboard_input.any_pressed([KeyCode::D, KeyCode::Right]);

        let mut vel_x = 0.0;

        if left {
            vel_x -= 1.0;
        } else if right {
            vel_x += 1.0;
        }

        transform.translation.x += vel_x;

        let drop = keyboard_input.any_pressed([KeyCode::S, KeyCode::Space]);

        if drop {
            println!("Drop");
            let Some(&(fruit, entity)) = current_fruit.fruit.as_ref() else {
                return;
            };
            let entity = commands
                .entity(entity)
                .remove::<Player>()
                .remove::<GravityScale>()
                .insert(GravityScale(10.0))
                .id();
            commands.entity(entity).despawn();
            commands.spawn((
                MaterialMesh2dBundle {
                    mesh: meshes.add(shape::Circle::new(fruit.radius()).into()).into(),
                    material: materials.add(ColorMaterial::from(fruit.color())),
                    transform: transform.clone(),
                    ..default()
                },
                RigidBody::Dynamic,
                fruit.collider(),
                ActiveEvents::COLLISION_EVENTS,
                ContactForceEventThreshold(10.0),
                Velocity::zero(),
                GravityScale(10.0),
                AdditionalMassProperties::Mass(10.0),
                Promote(fruit),
            ));
            current_fruit.fruit = None;
            next_spawn_timer.next_fruit_timer = Some(Timer::from_seconds(1.0, TimerMode::Once));
        }
    }
}

fn setup_graphics(mut commands: Commands) {
    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(0.0, 20.0, 0.0),
        ..default()
    });
}

pub fn setup_physics(mut commands: Commands) {
    /*
     * Ground
     */
    let ground_size = 300.0;
    let ground_thin = 10.0;
    let ground_width = 10.0;

    commands.spawn((
        TransformBundle::from(Transform::from_xyz(0.0, -ground_size, 0.0)),
        RigidBody::KinematicPositionBased,
        Collider::cuboid(ground_size, ground_thin),
    ));
    commands.spawn((
        TransformBundle::from(Transform::from_xyz(-ground_size, ground_size / 2.0, 0.0)),
        RigidBody::KinematicPositionBased,
        Collider::cuboid(ground_width, ground_size * 2.0),
    ));
    commands.spawn((
        TransformBundle::from(Transform::from_xyz(
            ground_size - ground_thin / 2.0,
            ground_size / 2.0,
            0.0,
        )),
        RigidBody::KinematicPositionBased,
        Collider::cuboid(ground_width, ground_size * 2.0),
    ));
}

fn display_events(
    mut collision_events: EventReader<CollisionEvent>,
    mut contact_force_events: EventReader<ContactForceEvent>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    fruits: Query<(&GlobalTransform, &Promote)>,
) {
    for collision_event in collision_events.iter() {
        println!("Received collision event: {collision_event:?}");
        match collision_event {
            &CollisionEvent::Started(a, b, _) => {
                println!("Started");
                let updated = match (fruits.get(a), fruits.get(b)) {
                    (Ok((t1, f1)), Ok((t2, f2))) => {
                        if let Some(next) = f1.0.promote(&f2.0) {
                            // let next_translation = (t1.translation() + t2.translation()) / 2.0;
                            let next_translation = t1.translation();
                            dbg!(next_translation, t1, t2);
                            commands.entity(a).despawn();
                            commands.entity(b).despawn();
                            let entity = commands
                                .spawn((
                                    MaterialMesh2dBundle {
                                        mesh: meshes
                                            .add(shape::Circle::new(next.radius()).into())
                                            .into(),
                                        material: materials.add(ColorMaterial::from(next.color())),
                                        transform: Transform::from_translation(next_translation),
                                        ..default()
                                    },
                                    RigidBody::Dynamic,
                                    next.collider(),
                                    ActiveEvents::COLLISION_EVENTS,
                                    Ccd::enabled(),
                                    ContactForceEventThreshold(10.0),
                                    Velocity::zero(),
                                    AdditionalMassProperties::Mass(10.0),
                                    Promote(next),
                                ))
                                .id();
                            Some((entity, next))
                        } else {
                            None
                        }
                    }
                    _ => None,
                };
            }
            _ => {}
        }
    }

    for contact_force_event in contact_force_events.iter() {
        println!("Received contact force event: {contact_force_event:?}");
    }
}
