use bevy::{color::palettes::css::*, prelude::*};
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
const INTERVAL: f32 = 26.0;

fn draw_function(mut gizmos: Gizmos, time: Res<Time>) {
    let domain = Interval::EVERYWHERE;
    let curve = FunctionCurve::new(domain, |t| Vec2::new(t, (t / RATE).powf(2.0) * RATE));

    let times_and_colors = (-100..=100)
        .map(|n| (n as f32) / INTERVAL * RATE)
        .map(|t| (t, RED));
    gizmos.curve_gradient_2d(curve, times_and_colors);

    let curve = FunctionCurve::new(domain, |t| Vec2::new(t, (t / RATE).powf(3.0) * RATE));

    let times_and_colors = (-100..=100)
        .map(|n| (n as f32) / INTERVAL * RATE)
        .map(|t| (t, YELLOW));
    gizmos.curve_gradient_2d(curve, times_and_colors);

    let curve = FunctionCurve::new(domain, |t| Vec2::new(t, (t / RATE).sin() * RATE));

    let times_and_colors = (-100..=100)
        .map(|n| (n as f32) / INTERVAL * RATE)
        .map(|t| (t, BLUE));
    gizmos.curve_gradient_2d(curve, times_and_colors);

        let curve = FunctionCurve::new(domain, |t| Vec2::new(t, (t / RATE).cos() * RATE));

    let times_and_colors = (-100..=100)
        .map(|n| (n as f32) / INTERVAL * RATE)
        .map(|t| (t, PURPLE));
    gizmos.curve_gradient_2d(curve, times_and_colors);
    
        let curve = FunctionCurve::new(domain, |t| Vec2::new(t, (t / RATE).powf(t / RATE) * RATE));

    let times_and_colors = (-100..=100)
        .map(|n| (n as f32) / INTERVAL * RATE)
        .map(|t| (t, GREEN));
    gizmos.curve_gradient_2d(curve, times_and_colors);
}
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2d);

    let half_length = INTERVAL * RATE + RATE / 2.0;
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

    for i in -(INTERVAL as i32)..=(INTERVAL as i32) {
        commands.spawn((
            Mesh2d(meshes.add(Segment2d::new(
                Vec2::new((i as f32) * RATE, 10.0),
                Vec2::new((i as f32) * RATE, 0.0),
            ))),
            MeshMaterial2d(materials.add(Color::from(BLACK))),
            Transform::from_translation(Vec3::ZERO),
        ));

        commands.spawn((
            Mesh2d(meshes.add(Segment2d::new(
                Vec2::new(10.0, (i as f32) * RATE),
                Vec2::new(0.0, (i as f32) * RATE),
            ))),
            MeshMaterial2d(materials.add(Color::from(BLACK))),
            Transform::from_translation(Vec3::ZERO),
        ));
    }
    let arrow_length = INTERVAL*1.5;
    let arrow_sin = arrow_length * (FRAC_PI_4.sin());

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
