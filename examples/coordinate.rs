use bevy::{color::palettes::{css::*, tailwind::RED_950}, prelude::*};
use std::f32::consts::{FRAC_PI_2, FRAC_PI_4, PI, TAU};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::from(WHITE)))
        .add_systems(Startup, setup)
        .add_systems(Update, draw_function)
        .run();
}

const RATE: f32 = 15.0;
const INTERVAL: f32 = 24.0;

struct FUNCTION {
    func: Box<dyn Fn(f32) -> f32>,
    color: Srgba,
}

impl FUNCTION {
    fn new(func: impl Fn(f32) -> f32 + 'static, color: Srgba) -> Self {
        FUNCTION {
            func: Box::new(func),
            color,
        }
    }
    
}
fn draw_function(mut gizmos: Gizmos,) {
    let domain = Interval::EVERYWHERE;
    let resolution=100;    
    let convert=|n:f32|( INTERVAL * RATE/(resolution as f32))*n;
    let function_vec=vec![
        FUNCTION::new(|t| t, RED),
        FUNCTION::new(|t| t.powf(2.0), YELLOW),
        FUNCTION::new(|t| t.powf(3.0), GREEN),

        
        FUNCTION::new(|t| t.sin(), BLUE),        
        FUNCTION::new(|t| t.cos(), PURPLE),     
        FUNCTION::new(|t| t.cos()+t.sin(), BLACK),
    ];

    for item in function_vec {
        let curve = FunctionCurve::new(domain, |t| Vec2::new(t, (item.func)(t/RATE)*RATE));
        let times_and_colors = (-resolution..=resolution)
            .map(|n| convert(n as f32))
            .map(|t| (t, item.color));
        gizmos.curve_gradient_2d(curve, times_and_colors);
    }


}
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2d);

    let half_length = INTERVAL * RATE + RATE * 0.8;
    commands.spawn((
        Mesh2d(meshes.add(Segment2d::new(
            Vec2::new(-half_length, 0.0),
            Vec2::new(half_length, 0.0),
        ))),
        MeshMaterial2d(materials.add(Color::from(BLACK))),
        Transform::from_translation(Vec3::ZERO),
    ));
    commands.spawn((
        Mesh2d(meshes.add(Segment2d::new(
            Vec2::new(0.0, -half_length),
            Vec2::new(0.0, half_length),
        ))),
        MeshMaterial2d(materials.add(Color::from(BLACK))),
        Transform::from_translation(Vec3::ZERO),
    ));

    let point_height = INTERVAL * 0.3;
    for i in -(INTERVAL as i32)..=(INTERVAL as i32) {
        commands.spawn((
            Mesh2d(meshes.add(Segment2d::new(
                Vec2::new((i as f32) * RATE, point_height),
                Vec2::new((i as f32) * RATE, 0.0),
            ))),
            MeshMaterial2d(materials.add(Color::from(BLACK))),
            Transform::from_translation(Vec3::ZERO),
        ));

        commands.spawn((
            Mesh2d(meshes.add(Segment2d::new(
                Vec2::new(point_height, (i as f32) * RATE),
                Vec2::new(0.0, (i as f32) * RATE),
            ))),
            MeshMaterial2d(materials.add(Color::from(BLACK))),
            Transform::from_translation(Vec3::ZERO),
        ));
    }
    let arrow_sin = point_height * (FRAC_PI_4.sin());

    commands.spawn((
        Mesh2d(meshes.add(Segment2d::new(
            Vec2::new(half_length, 0.0),
            Vec2::new(half_length - arrow_sin, arrow_sin),
        ))),
        MeshMaterial2d(materials.add(Color::from(BLACK))),
        Transform::from_translation(Vec3::ZERO),
    ));

    commands.spawn((
        Mesh2d(meshes.add(Segment2d::new(
            Vec2::new(half_length, 0.0),
            Vec2::new(half_length - arrow_sin, -arrow_sin),
        ))),
        MeshMaterial2d(materials.add(Color::from(BLACK))),
        Transform::from_translation(Vec3::ZERO),
    ));

    commands.spawn((
        Mesh2d(meshes.add(Segment2d::new(
            Vec2::new(0.0, half_length),
            Vec2::new(arrow_sin, half_length - arrow_sin),
        ))),
        MeshMaterial2d(materials.add(Color::from(BLACK))),
        Transform::from_translation(Vec3::ZERO),
    ));

    commands.spawn((
        Mesh2d(meshes.add(Segment2d::new(
            Vec2::new(0.0, half_length),
            Vec2::new(-arrow_sin, half_length - arrow_sin),
        ))),
        MeshMaterial2d(materials.add(Color::from(BLACK))),
        Transform::from_translation(Vec3::ZERO),
    ));
}
