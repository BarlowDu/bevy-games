use bevy::{color::palettes::css::*, prelude::*, window::WindowResolution};
use chrono::{DateTime, Local, Timelike};
// Define a component to designate a rotation speed to an entity.
#[derive(Component)]
struct Movable {
    speed: f32,
    distance: f32,
    angle: i32,
    op: fn(f32, f32) -> f32,
}

impl Movable {
    fn new(speed: f32) -> Self {
        Self {
            speed,
            distance: 0.,
            angle: 0,
            op:std::ops::Add::<f32>::add
        }
    }
}



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
    App::new()
        //.add_plugins(DefaultPlugins)
        .add_plugins(
            DefaultPlugins
                // 1. 开启窗口透明
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        position: WindowPosition::Centered(MonitorSelection::Primary),
                        resolution: WindowResolution::new(300, 300),
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
        .add_systems(Update, (rotate_cube,rotate_time,rotate_light,drag_window))
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // 移动球
    commands.spawn((
        Mesh3d(meshes.add(Sphere::new(1.))),
        MeshMaterial3d(materials.add(Color::srgba(0.0, 0.0, 1.0, 1.5))),
        Transform::from_translation(Vec3::new(5., 5., 1.)),
        Movable::new(0.005),
    ));
    // 中心点
    commands.spawn((
        Mesh3d(meshes.add(Sphere::new(0.5))),
        MeshMaterial3d(materials.add(Color::from(GOLD))),
        Transform::from_translation(Vec3::ZERO),
    ));

    // 相机
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(15.0, 15.0, 35.0).looking_at(Vec3::ZERO, Vec3::Y),
        
    ));

    // 光源
    commands.spawn((
        SpotLight  {
            intensity:30_000_000.,
            shadows_enabled: true,
            range:300.,
            inner_angle:10.,
            ..default()
        },
        Transform::from_xyz(15.0, 0.0, 35.0).looking_at(Vec3::new(10.,0.,0.), Vec3::Y),
    ));



    
    // 表盘
    commands.spawn((
        Mesh3d(meshes.add(Circle::new(12.0).mesh().resolution(256))),
        MeshMaterial3d(materials.add(Color::srgba(1.0, 1.0, 1.0, 0.0))),
        Transform::from_translation(Vec3::new(0., 0., 0.)),
    ));

    // 刻度
    for i in (0..360).step_by(30) {
        let angle = i as f32 * std::f32::consts::PI / 180.;
        let x = 10. * angle.cos();
        let y = 10. * angle.sin();
        commands.spawn((
            Mesh3d(meshes.add(Sphere::new(0.8))),
            MeshMaterial3d(materials.add(Color::srgb_u8(234, 108, 57))),
            Transform::from_translation(Vec3::new(x, y, 1.)),
        ));
    }
    // 秒针
    commands.spawn((
        Mesh3d(meshes.add(Cylinder::new(0.05, 9.).mesh().resolution(180))),
        MeshMaterial3d(materials.add(Color::from(GOLD))),
        Transform {
            translation: Vec3::new(4.5, 0., 0.),
            rotation: Quat::from_rotation_z(0.0 - std::f32::consts::FRAC_PI_2),
            ..Transform::IDENTITY
        },
        TimeHand::new(TimeUnit::Second),
    ));
    // 分针
    commands.spawn((
        Mesh3d(meshes.add(Cylinder::new(0.1, 7.).mesh().resolution(180))),
        MeshMaterial3d(materials.add(Color::from(GOLD))),
        Transform {
            translation: Vec3::new(3.5, 0., 0.),
            rotation: Quat::from_rotation_z(0.0 - std::f32::consts::FRAC_PI_2),
            ..Transform::IDENTITY
        },
        TimeHand::new(TimeUnit::Minute),
    ));
    // 时针
    commands.spawn((
        Mesh3d(meshes.add(Cylinder::new(0.15, 5.).mesh().resolution(180))),
        MeshMaterial3d(materials.add(Color::from(GOLD))),
        Transform {
            translation: Vec3::new(2.5, 0., 0.),
            rotation: Quat::from_rotation_z(0.0 - std::f32::consts::FRAC_PI_2),
            ..Transform::IDENTITY
        },
        TimeHand::new(TimeUnit::Hour),
    ));
}

// This system will rotate any entity in the scene with a Rotatable component around its y-axis.
fn rotate_cube(mut cubes: Query<(&mut Transform, &mut Movable)>) {
    for (mut transform, mut cube) in &mut cubes {
        
        if cube.distance > 11. {
            cube.op=std::ops::Sub::<f32>::sub;
        }if cube.distance < 0.01 {
            cube.op=std::ops::Add::<f32>::add;
        }
        let distance = (cube.op)(cube.distance , cube.speed);
        let angle = cube.angle + 1;
        let x = distance * (angle as f32).to_radians().cos();
        let y = distance * (angle as f32).to_radians().sin();
        transform.translation.x = x;
        transform.translation.y = y;
        cube.distance = distance;
        cube.angle = angle % 360;
    }
}

fn rotate_time(mut query: Query<(&mut Transform, &mut TimeHand), With<Mesh3d>>) {
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

fn rotate_light(mut query: Query<&mut Transform, With<SpotLight>>) {
    let time = Local::now();
    for mut transform in query.iter_mut() {
        let angle =
            (TimeHand::convert_ms(time.second() as i32)+90) as f32;

        let x = 15. * (angle.to_radians().sin());
        let y = 15. * (angle.to_radians().cos());
        //transform.translation = Vec3::new(x, y, 50.);
        transform.translation.x = x;
        transform.translation.y = y;
        let _=transform.looking_at(Vec3::new(x/1.5, y/1.5,0.),Vec3::Y);
        //println!("light angle: {}, x: {}, y: {}", angle, x, y);
    }
}

// 窗口拖动系统
fn drag_window(
    mouse_input: Res<ButtonInput<MouseButton>>,
    mut windows: Query<&mut Window>,
) {
    // 正确判断：鼠标左键刚刚按下
    if mouse_input.just_pressed(MouseButton::Left) {
        if let Ok(mut win) = windows.single_mut(){
            win.start_drag_move();
        }
    }
}
