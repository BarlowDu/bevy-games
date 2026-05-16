use bevy::{color::palettes::basic::*, prelude::*};
use rand::RngExt;

#[derive(Component)]
struct Bird;

#[derive(Resource)]
struct BirdPosition {
    x: f32,
    y: f32,
    width: f32,
    height: f32,
}

#[derive(Component)]
struct Pipe {
    location: PipeLocation,
    height: f32,
}

enum PipeLocation {
    TOP,
    BOTTOM,
}

impl Default for BirdPosition {
    fn default() -> Self {
        BirdPosition {
            x: 0.,
            y: 0.,
            width: 20.,
            height: 20.,
        }
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(BirdPosition::default())
        .add_systems(Startup, setup)
        .add_systems(Update, (bird_ctrl,pipe_translate))
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    bird_position: Res<BirdPosition>,
) {
    commands.spawn(Camera2d);

    commands.spawn((
        Mesh2d(meshes.add(Rectangle::new(bird_position.width, bird_position.height))),
        MeshMaterial2d(materials.add(Color::from(RED))),
        //MeshMaterial2d(materials.add(Color::from(RED))),
        Transform::from_xyz(bird_position.x, bird_position.y, 10.),
        Bird,
    ));
    commands.spawn((
        Mesh2d(meshes.add(Rectangle::new(220., 420.0))),
        //MeshMaterial2d(materials.add(Color::srgb_u8(43u8, 44u8, 47u8))),
        MeshMaterial2d(materials.add(Color::srgba(0., 0., 1., 0.5))),
        Transform::from_xyz(0., 0., -5.),
    ));
    generate_pipe(&mut commands, &mut meshes, &mut materials);
}

fn bird_ctrl(
    mut query: Query<(&mut Transform, &Bird)>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    let a: fn(f32, f32) -> f32 = std::ops::Add::<f32>::add;
    let b = a(1., 2.);
    let mut op: Option<fn(f32, f32) -> f32> = Option::None;
    if keyboard_input.just_pressed(KeyCode::ArrowUp) {
        op = Option::Some(std::ops::Add::<f32>::add);
    }
    if keyboard_input.just_pressed(KeyCode::ArrowDown) {
        op = Option::Some(std::ops::Sub::<f32>::sub);
    }
    match op {
        Some(p) => {
            for (mut transform, _bird) in query.iter_mut() {
                let y = p(transform.translation.y, 50.);
                if y <= 210. && y >= -210. {
                    transform.translation.y = y;
                }
            }
        }
        _ => {}
    }
}

fn pipe_translate(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials:ResMut<Assets<ColorMaterial>>,
    mut query: Query<(Entity, &mut Transform, &Pipe)>,
    time: Res<Time>,
) {
    let mut removed = false;
    for (entity, mut transform, _pipe) in query.iter_mut() {
        let x = transform.translation.x - 60.0 * time.delta_secs();
        if x <= -100. {
            removed = true;
            commands.entity(entity).despawn();
        }else{
            transform.translation.x=x;
        }
    }
    if removed {
        generate_pipe(&mut commands, &mut meshes, &mut materials);
    }
}

fn generate_pipe(
    mut commands: &mut Commands,
    mut meshes: &mut ResMut<Assets<Mesh>>,
    mut materials: &mut ResMut<Assets<ColorMaterial>>,
) {
    let mut rng = rand::rng();
    let rate = rng.random_range(0.6..=0.9);
    let height = 210. * rate;
    commands.spawn((
        Mesh2d(meshes.add(Rectangle::new(20., height))),
        MeshMaterial2d(materials.add(Color::from(GREEN))),
        //MeshMaterial2d(materials.add(Color::from(RED))),
        Transform::from_xyz(100., 210.0 - height / 2., 10.),
        Pipe {
            location: PipeLocation::TOP,
            height: height,
        },
    ));

    commands.spawn((
        Mesh2d(meshes.add(Rectangle::new(20., height))),
        MeshMaterial2d(materials.add(Color::from(GREEN))),
        //MeshMaterial2d(materials.add(Color::from(RED))),
        Transform::from_xyz(100., -(210.0 - height / 2.), 10.),
        Pipe {
            location: PipeLocation::BOTTOM,
            height: height,
        },
    ));
}
