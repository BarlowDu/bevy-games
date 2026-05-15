use bevy::{
    asset::RenderAssetUsages, color::palettes::css::*, mesh::Indices, prelude::*,
    render::render_resource::PrimitiveTopology,
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        // 3. 设置白底
        .insert_resource(ClearColor(Color::WHITE))
        .add_systems(Startup, setup)
        //.add_systems(Update, shoot_cell)
        //.add_systems(Update, rotate)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2d);

    /*commands.spawn((
        Mesh2d(meshes.add(RegularPolygon::new(100.0, 17))),
        MeshMaterial2d(materials.add(Color::from(RED))),
        Transform::from_translation(Vec3::ZERO),
    ));*/
    let width =100.0;
    let mut star = Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::RENDER_WORLD,
    );
    let mut v_pos = vec![
        [0.0-200.0, 0.0, 0.0],
        [(width*3.285714)/1.414-200.0, (width*3.285714)/1.414,0.0],
        [(width/1.414)+(width*4.285714/1.414)-200.0, (-width/1.414)+(width*4.285714/1.414), 0.0],
        [width/1.414-200.0, -width/1.414, 0.0],
    ];

    println!("{:?}", v_pos);
    star.insert_attribute(Mesh::ATTRIBUTE_POSITION, v_pos);
    let mut indices = vec![0, 1, 2, 0, 2, 3];
    //indices.extend_from_slice(&[0, 2, 3]);
    star.insert_indices(Indices::U32(indices));
    commands.spawn((
        // We use a marker component to identify the custom colored meshes
        //ColoredMesh2d,
        // The `Handle<Mesh>` needs to be wrapped in a `Mesh2d` for 2D rendering
        Mesh2d(meshes.add(star)),
        MeshMaterial2d(materials.add(Color::srgba(
            0.0862745098039216,
            0.7254901960784314,
            0.992156862745098,
            0.9,
        ))),
        Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
    ));

    star = Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::RENDER_WORLD,
    );
    v_pos = vec![
        [width*1.5/1.414-200.0, -width*1.5/1.414, 0.0],
        [(width*1.5/1.414)+(width*1.785714/1.414)-200.0, -(width*1.5/1.414)+(width*1.785714/1.414), 0.0],
        [(width*2.5/1.414)+(width*2.785714/1.414)-200.0, (-width*2.5/1.414)+(width*2.785714/1.414), 0.0],
        [width*2.5/1.414-200.0, -width*2.5/1.414, 0.0],
    ];
    star.insert_attribute(Mesh::ATTRIBUTE_POSITION, v_pos);
    indices = vec![0, 1, 2, 0, 2, 3];
    //indices.extend_from_slice(&[0, 2, 3]);
    star.insert_indices(Indices::U32(indices));
    commands.spawn((
        // We use a marker component to identify the custom colored meshes
        //ColoredMesh2d,
        // The `Handle<Mesh>` needs to be wrapped in a `Mesh2d` for 2D rendering
        Mesh2d(meshes.add(star)),
        MeshMaterial2d(materials.add(Color::srgba(
            0.0862745098039216,
            0.7254901960784314,
            0.992156862745098,
            0.9,
        ))),
        Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
    ));

    star = Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::RENDER_WORLD,
    );
    v_pos = vec![
        [width*1.5/1.414-200.0, -width*1.5/1.414, 0.0],
        [(width*1.5/1.414)+(width*1.785714/1.414)-200.0, (-width*1.5/1.414)-(width*1.785714/1.414), 0.0],
        [(width*1.5/1.414)+(width/1.414)+(width*2.785714/1.414)-200.0, (-width*1.5/1.414)+(width/1.414)-(width*2.785714/1.414), 0.0],
        [(width*1.5/1.414)+(width/1.414)-200.0, (-width*1.5/1.414)+(width/1.414), 0.0],
    ];
    star.insert_attribute(Mesh::ATTRIBUTE_POSITION, v_pos);
    indices = vec![0, 1, 2, 0, 2, 3];
    //indices.extend_from_slice(&[0, 2, 3]);
    star.insert_indices(Indices::U32(indices));
    commands.spawn((
        // We use a marker component to identify the custom colored meshes
        //ColoredMesh2d,
        // The `Handle<Mesh>` needs to be wrapped in a `Mesh2d` for 2D rendering
        Mesh2d(meshes.add(star)),
        MeshMaterial2d(materials.add(Color::srgb(
            0.0352941176470588,
            0.3529411764705882,
            0.6156862745098039,
        ))),
        Transform::from_translation(Vec3::new(0.0, 0.0, -1.0)),
    ));
}
