use bevy::{color::palettes::basic::*, prelude::*};
use rand::RngExt;

#[derive(Component)]
struct TaiJi;
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, rotate_taiji)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2d);

    let root = commands
        .spawn((
            Mesh2d(meshes.add(Circle::default())),
            //MeshMaterial2d(materials.add(Color::None)),
            TaiJi,
        ))
        .id();

    let r = 128f32;
    let mut rchildren = Vec::<Entity>::new();

    rchildren.push(
        commands
            .spawn((
                Mesh2d(meshes.add(CircularSegment::new(r, 90f32.to_radians()))),
                MeshMaterial2d(materials.add(Color::from(WHITE))),
                Transform::from_translation(Vec3::ZERO),
                //TaiJi,
            ))
            .id(),
    );

    rchildren.push(
        commands
            .spawn((
                Mesh2d(meshes.add(CircularSegment::new(r, 90f32.to_radians()))),
                MeshMaterial2d(materials.add(Color::from(BLACK))),
                Transform {
                    translation: Vec3::ZERO,
                    rotation: Quat::from_rotation_z(180f32.to_radians()),
                    ..default()
                },
                //TaiJi,
            ))
            .id(),
    );

    rchildren.push(
        commands
            .spawn((
                Mesh2d(meshes.add(CircularSegment::new(r / 2., 90f32.to_radians()))),
                MeshMaterial2d(materials.add(Color::from(BLACK))),
                Transform {
                    translation: Vec3::new(-r / 2., 0., 0.),
                    ..default()
                },
                //TaiJi,
            ))
            .id(),
    );

    rchildren.push(
        commands
            .spawn((
                Mesh2d(meshes.add(CircularSegment::new(r / 2., 90f32.to_radians()))),
                MeshMaterial2d(materials.add(Color::from(WHITE))),
                Transform {
                    translation: Vec3::new(r / 2., 0., 0.),
                    rotation: Quat::from_rotation_z(180f32.to_radians()),
                    ..default()
                },
                //TaiJi,
            ))
            .id(),
    );

    rchildren.push(
        commands
            .spawn((
                Mesh2d(meshes.add(Circle::new(r / 8.))),
                MeshMaterial2d(materials.add(Color::from(WHITE))),
                Transform {
                    translation: Vec3::new(-r / 2., 0., 0.),
                    ..default()
                },
                //TaiJi,
            ))
            .id(),
    );

    rchildren.push(
        commands
            .spawn((
                Mesh2d(meshes.add(Circle::new(r / 8.))),
                MeshMaterial2d(materials.add(Color::from(BLACK))),
                Transform {
                    translation: Vec3::new(r / 2., 0., 0.),
                    ..default()
                },
                //TaiJi,
            ))
            .id(),
    );

    for entity in rchildren {
        commands.entity(root).add_child(entity);
    }

    let mut draw_gua = |gua: u8, index: i32| {
        let y = 0.;
        let angle = (-(index * 45) as f32).to_radians();
        for i in 0..=2 {
            let x = 150. + (i as f32) * 30.;
            if 2i32.pow(i) & gua as i32 > 0 {
                // ========== 绕点旋转核心代码 ==========
                let mut pos = Vec3::new(x, y, 0.);

                commands.spawn((
                    Mesh2d(meshes.add(Rectangle::new(10., 90.))),
                    MeshMaterial2d(materials.add(Color::from(WHITE))),
                    //get_final_transfrom(Vec3::new(x, y, 0.), angle)
                    get_final_transfrom(Vec3::new(x, 0., 0.), angle),
                ));
            } else {
                commands.spawn((
                    Mesh2d(meshes.add(Rectangle::new(10., 40.))),
                    MeshMaterial2d(materials.add(Color::from(WHITE))),
                    //get_final_transfrom(Vec3::new(x, 25., 0.), angle)
                    get_final_transfrom(Vec3::new(x, 25., 0.), angle),
                ));

                commands.spawn((
                    Mesh2d(meshes.add(Rectangle::new(10., 40.))),
                    MeshMaterial2d(materials.add(Color::from(WHITE))),
                    //get_final_transfrom(Vec3::new(x, -25., 0.), angle)
                    get_final_transfrom(Vec3::new(x, -25., 0.), angle),
                ));
            }
        }
    };
    let guas = vec![6u8, 7u8, 2u8, 1u8, 4u8, 3u8, 5u8, 0u8];
    for i in 0..8 {
        draw_gua(guas[i as usize], i);
    }
    //draw_gua(6u8, 0);;
}

fn get_final_transfrom(transition: Vec3, angle: f32) -> Transform {
    let pivot = Vec3::ZERO;
    // 1. 得到从中心点到物体的向量
    let mut point_vector = transition - pivot;
    // 2. 旋转这个向量
    let point_vector = transition - pivot; // 中心点→物体的向量
    let rotation = Quat::from_rotation_z(angle); // 旋转四元数
    let new_point_vector = rotation * point_vector; // 旋转向量
    let new_world_pos = pivot + new_point_vector; // 新位置

    // ==========================
    // 3. 物体自身角度（基准方向 [50,50]）
    // ==========================
    let final_rotation = Quat::from_rotation_z(angle);
    return Transform {
        translation: new_world_pos,
        rotation: final_rotation,
        ..default()
    };
}

fn rotate_taiji(query: Query<(&mut Transform, &TaiJi), With<Mesh2d>>, time: Res<Time>) {
    for (mut transform, _) in query {
        transform.rotate_z(0.5 * time.delta_secs());
    }
}
