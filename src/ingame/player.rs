use bevy::{
    animation::RepeatAnimation,
    ecs::event::ManualEventReader,
    input::mouse::MouseMotion,
    prelude::*,
    window::{CursorGrabMode, PrimaryWindow},
};
use bevy_xpbd_3d::math::PI;
use bevy_xpbd_3d::prelude::*;

#[derive(Resource)]
pub struct Animations(Vec<Handle<AnimationClip>>);

#[derive(Resource, Default)]
pub struct InputState {
    reader_motion: ManualEventReader<MouseMotion>,
}

#[derive(Resource)]
pub struct SensitivitySettings {
    pub sensitivity: f32,
}

pub fn edit_mode_toggler(input: ResMut<ButtonInput<KeyCode>>, mut windows: Query<&mut Window>) {
    if input.just_pressed(KeyCode::Escape) {
        let mut window = windows.single_mut();
        match window.cursor.grab_mode {
            CursorGrabMode::Confined => {
                window.cursor.grab_mode = CursorGrabMode::None;
                window.cursor.visible = true;
            }
            _ => {
                window.cursor.grab_mode = CursorGrabMode::Confined;
                window.cursor.visible = false;
            }
        }
    }
}

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    //gun animation load
    commands.insert_resource(Animations(vec![
        asset_server.load("models/p226_anim.glb#Animation0")
    ]));

    //crosshair test
    commands.spawn(NodeBundle {
        style: Style {
            height: Val::Px(2.0),
            width: Val::Px(6.0),
            align_self: AlignSelf::Center,
            justify_self: JustifySelf::Center,
            ..default()
        },
        background_color: Color::WHITE.into(),
        ..default()
    });
    commands.spawn(NodeBundle {
        style: Style {
            height: Val::Px(6.0),
            width: Val::Px(2.0),
            align_self: AlignSelf::Center,
            justify_self: JustifySelf::Center,
            ..default()
        },
        background_color: Color::WHITE.into(),
        ..default()
    });

    //player
    commands
        .spawn((
            Camera3dBundle {
                transform: Transform::from_xyz(0.0, 0.8, 0.0),
                projection: Projection::Perspective(PerspectiveProjection {
                    fov: 90.0 / 180.0 * PI, // ! One PI = 180, first value is the real fov
                    near: 0.01,
                    ..default()
                }),

                ..default()
            },
            FogSettings {
                color: Color::BLACK,
                falloff: FogFalloff::Exponential { density: 1.0 },
                ..default()
            },
        ))
        .with_children(|parent| {
            // child cube
            parent.spawn((
                SceneBundle {
                    scene: asset_server.load("models/p226_anim.glb#Scene0"),
                    transform: Transform::from_xyz(0.15, -0.1, -0.19),
                    ..default()
                },
                Name::new("P226"),
            ));
        })
        .with_children(|parent| {
            parent.spawn((
                Collider::sphere(0.01),
                TransformBundle::from(Transform::from_xyz(0.0, 0.0, -1.0)),
                Name::new("BulletSpawner"),
            ));
        });

    //shooting range model
    commands.spawn((
        SceneBundle {
            scene: asset_server.load("models/shooting-range.glb#Scene0"),
            ..default()
        },
        Name::new("Shooting Range"),
    ));
    // ! Blender models looking at positive Y direction

    //point light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 10_000_000.,
            range: 100.,
            ..default()
        },
        transform: Transform::from_xyz(0.0, 16.0, 0.0),
        ..default()
    });
}

pub fn run_animation(
    animations: Res<Animations>,
    mut gun_query: Query<&mut AnimationPlayer, Added<AnimationPlayer>>,
) {
    for mut gun in &mut gun_query {
        gun.play(animations.0[0].clone_weak()).repeat();
        gun.set_repeat(RepeatAnimation::Count(0));
    }
}

pub fn keyboard_animation_control(
    input: Res<ButtonInput<MouseButton>>,
    mut gun_query: Query<&mut AnimationPlayer>,
    windows: Query<&Window>,
) {
    let window = windows.single();
    for mut gun in &mut gun_query {
        if window.cursor.grab_mode == CursorGrabMode::Confined
            && input.just_pressed(MouseButton::Left)
        {
            gun.set_repeat(RepeatAnimation::Count(1));
            gun.replay();
        }
    }
}

pub fn player_look(
    time: Res<Time>,
    settings: Res<SensitivitySettings>,
    primary_window: Query<&Window, With<PrimaryWindow>>,
    mut state: ResMut<InputState>,
    motion: Res<Events<MouseMotion>>,
    mut query_camera: Query<&mut Transform, With<Camera3d>>,
) {
    if let Ok(window) = primary_window.get_single() {
        for ev in state.reader_motion.read(&motion) {
            let mut camera_transform = query_camera.single_mut();

            let (mut yaw_camera, mut pitch_camera, _) =
                camera_transform.rotation.to_euler(EulerRot::YXZ);

            match window.cursor.grab_mode {
                CursorGrabMode::None => (),
                _ => {
                    // Using smallest of height or width ensures equal vertical and horizontal sensitivity
                    let window_scale = window.height().min(window.width());
                    pitch_camera -= (settings.sensitivity * ev.delta.y * window_scale).to_radians()
                        * time.delta_seconds();
                    yaw_camera -= (settings.sensitivity * ev.delta.x * window_scale).to_radians()
                        * time.delta_seconds();
                }
            }

            pitch_camera = pitch_camera.clamp(-1.54, 1.54);

            // Order is important to prevent unintended roll
            camera_transform.rotation = Quat::from_axis_angle(Vec3::Y, yaw_camera)
                * Quat::from_axis_angle(Vec3::X, pitch_camera);
        }
    } else {
        warn!("Primary window not found for `player_look`!");
    }
}
