use bevy::{color::palettes::css::*, 
    prelude::*, 
    window::{WindowResolution}};
use chrono::{DateTime, Local, Timelike};


#[derive(Component)]
struct TimeHand {
    angle: i32,
    unit: TimeUnit,
}

enum TimeUnit {
    Hour,
    Minute,
    Second,
}

impl TimeHand {
    fn new(unit: TimeUnit) -> Self {
        Self { angle: 0, unit }
    }

    fn update_angle(&mut self, new_angle: i32) -> f32 {
        let angle = if new_angle > self.angle {
            new_angle - self.angle
        } else {
            360 + new_angle - self.angle
        };
        self.angle = new_angle;
        (angle as f64).to_radians() as f32
    }

    fn update_time(&mut self, time: DateTime<Local>) -> f32 {
        let new_angle = match self.unit {
            TimeUnit::Hour => TimeHand::convert_h(time.hour() as i32),
            TimeUnit::Minute => TimeHand::convert_ms(time.minute() as i32),
            TimeUnit::Second => TimeHand::convert_ms(time.second() as i32),
        };
        self.update_angle(new_angle)
    }

    fn convert_ms(ms: i32) -> i32 {
        let res = (ms * 6) - 90;
        if res < 0 { res + 360 } else { res }
    }
    fn convert_h(h: i32) -> i32 {
        let res = ((h % 12) * 30) - 90;
        if res < 0 { res + 360 } else { res }
    }
}

fn main() {
    println!("270:{},90:{}", 270.0f64.to_radians(), 90.0f64.to_radians());
    App::new()
        .add_plugins(
            DefaultPlugins
                // 1. 开启窗口透明
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        position: WindowPosition::Centered(MonitorSelection::Primary),
                        resolution: WindowResolution::new(450, 450),
                        transparent: true, // ✅ 窗口透明
                        resizable: false,
                        decorations: false,
                        
                        ..default()
                    }),
                    ..default()
                }),
        )
        // 3. 设置透明清除色
        .insert_resource(ClearColor(Color::NONE))
        .add_systems(Startup, setup)
        .add_systems(Update, rotate)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2d);
    // 相机
    for i in (0..360).step_by(30) {
        let angle = i as f32 * std::f32::consts::PI / 180.;
        let x = 200. * angle.cos();
        let y = 200. * angle.sin();
        commands.spawn((
            Mesh2d(meshes.add(Circle::new(10.0))),
            MeshMaterial2d(materials.add(Color::from(PURPLE))),
            Transform::from_xyz(x, y, 0.),
        ));
    }
    commands.spawn((
        Mesh2d(meshes.add(Circle::new(215.0).mesh().resolution(256))),
        //MeshMaterial2d(materials.add(Color::srgba(0., 0., 1., 0.))), 
        MeshMaterial2d(materials.add(Color::srgba(0.0,0.0,0.6,0.5))), // ✅ 圆盘透明
        Transform::from_xyz(0., 0., -10.),
    ));

    commands.spawn((
        Mesh2d(meshes.add(Ellipse::new(80.0, 1.0).mesh().resolution(180))),
        MeshMaterial2d(materials.add(Color::from(WHITE))),
        Transform::from_xyz(80., 0., 0.),
        TimeHand::new(TimeUnit::Second),
    ));

    commands.spawn((
        Mesh2d(meshes.add(Ellipse::new(60.0, 2.0).mesh().resolution(180))),
        MeshMaterial2d(materials.add(Color::from(WHITE))),
        Transform::from_xyz(60., 0., 0.),
        TimeHand::new(TimeUnit::Minute),
    ));

    commands.spawn((
        Mesh2d(meshes.add(Ellipse::new(40.0, 3.0).mesh().resolution(180))),
        MeshMaterial2d(materials.add(Color::from(WHITE))),
        Transform::from_xyz(40., 0., 0.),
        TimeHand::new(TimeUnit::Hour),
    ));
}

fn rotate(mut query: Query<(&mut Transform, &mut TimeHand), With<Mesh2d>>) {
    let time = Local::now();
    for (mut transform, mut time_hand) in query.iter_mut() {
        let angle = time_hand.update_time(time);

        let pivot = Vec3::new(0.0, 0.0, 0.0);
        let mut pos = transform.translation - pivot;
        let rot = Quat::from_rotation_z(0.0 - angle);
        pos = rot * pos;
        transform.translation = pivot + pos;
        transform.rotation = rot * transform.rotation;
    }
}


