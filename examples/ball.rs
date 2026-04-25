use bevy::{color::palettes::basic::*,prelude::*};
use rand::RngExt;


#[derive(Component)]
struct MoveCube{
    angle:i32,
    speed:f32,
}

impl MoveCube {
    fn new(angle: i32, speed: f32) -> Self {
        Self { angle, speed }
    }

    pub fn next(&self,x:f32,y:f32) -> (f32,f32) {
        let rad = (self.angle as f32).to_radians();
        let nx = x + self.speed * rad.cos();
        let ny = y + self.speed * rad.sin();
        (nx, ny)
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, auto_move)
        .add_systems(Update, remove_cube)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2d);

    commands.spawn((
        Mesh2d(meshes.add(Circle::new(50.))),
        MeshMaterial2d(materials.add(Color::from(PURPLE))),
        Transform::default(),
        MoveCube::new(0, 1.0)
    ));
    commands.spawn((
        Mesh2d(meshes.add(Rectangle::new(1100., 1100.0).to_ring(50.0))),
        MeshMaterial2d(materials.add(Color::srgb_u8(43u8, 44u8, 47u8))),
        //MeshMaterial2d(materials.add(Color::from(RED))),
        Transform::from_xyz(0., 0., 10.),
    ));
    commands.spawn((
        Mesh2d(meshes.add(Rectangle::new(1000., 1000.0).to_ring(5.0))),
        //MeshMaterial2d(materials.add(Color::srgb_u8(43u8, 44u8, 47u8))),
        MeshMaterial2d(materials.add(Color::from(RED))),
        Transform::from_xyz(0., 0., 15.),
    ));
}

fn auto_move(
    mut query: Query<(&mut Transform, &MoveCube)>,
) {
    for (mut transform, move_cube) in query.iter_mut() {
        let (nx, ny) = move_cube.next(transform.translation.x, transform.translation.y);
        transform.translation.x = nx;
        transform.translation.y = ny;
    }
}

fn remove_cube(
    mut commands: Commands,
    query: Query<(Entity, &Transform,&MoveCube), With<MoveCube>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let mut removed=0;
    let mut total=0;
    for (entity, transform,move_cube) in query.iter() {
        if transform.translation.x > 500.0 
        || transform.translation.y > 500.0
        ||transform.translation.x < -500.0 
        || transform.translation.y < -500.0 {
            commands.entity(entity).despawn();
            removed+=1;
        }
        total+=1;
    }
    if removed<=0{
        return;
    }
    let mut rng = rand::rng();
    let n=if total-removed>5{
        0
    }else{
        rng.random_range(1..=3)
    };
    for _ in 0..n {
        let angle = rng.random_range(0..360);
        let speed = rng.random_range(1.0..10.0);
        commands.spawn((
            Mesh2d(meshes.add(Circle::new(rng.random_range(10.0..50.0)))),
            MeshMaterial2d(materials.add(Color::srgb(
                rng.random_range(0.0..1.0),
                rng.random_range(0.0..1.0),
                rng.random_range(0.0..1.0),

            ))),
            Transform::from_xyz(rng.random_range(-200.0..200.0), rng.random_range(-200.0..200.0), 0.),
            MoveCube::new(angle, speed)
        ));
    }
    
}
