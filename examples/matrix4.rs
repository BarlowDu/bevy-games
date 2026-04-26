use bevy::{
    prelude::*,
    text::{FontSmoothing},
};

use rand::{self, RngExt};

const ACCELERATION:f32=49.0;

fn main() {

    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                transparent: true, // ✅ 窗口透明

                ..default()
            }),
            ..default()
        }))
        .insert_resource(ClearColor(Color::srgba(0., 0., 0., 0.7)))
        .add_systems(Startup, setup)
        .add_systems(Update, animate_fall)
        .run();
}

#[derive(Component)]
struct Fall {
    speed: f32,
}

fn setup(mut commands: Commands,asset_server: Res<AssetServer>) {
    let font = asset_server.load("fonts/msyh.ttc");
    let text_font = TextFont {
        font: font.clone(),
        font_size: 10.0,
        ..default()
    };
    // Add a light source so we can see clearly.
    commands.spawn((
        DirectionalLight::default(),
        Transform::from_xyz(0.0, 0.0, 3.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    commands.spawn(Camera2d);
    let mut rng = rand::rng();
    for i in 0..=300 {
        let parent=commands.spawn((
             Sprite::from_color(Color::NONE, Vec2::new(10.,500.)),
             Transform::from_translation(Vec3::new(-rng.random_range(0.9..1.5)*1000. + (i * 25) as f32, 500., 0.)),
             Fall {
                speed: get_init_speed()*2.0,
            },
            
            //children![a]
        )).id();
        //let mut pentity=commands.entity(parent);
        let ch=get_fall_char(i);
        for i in 0..=50{
             let child=commands.spawn((
                Text2d::new(&ch),
                text_font.clone().with_font_smoothing(FontSmoothing::AntiAliased),
                TextColor::from(Color::srgba(0.,1.,0.,1.0-(i as f32*0.02))),
                Transform::from_translation(Vec3::new(0.,(-250+(10*i)) as f32,10.)),
             )).id();
             commands.entity(parent).add_child(child);
        }
         /*for item in a{
            let child=commands.spawn(item).id();
            commands.entity(parent).add_child(child);
         }*/
    }
}

fn animate_fall(mut query: Query<(&mut Transform, &mut Fall), With<Sprite>>,time:Res<Time>) {
    // Consider changing font-size instead of scaling the transform. Scaling a Text2D will scale the
    // rendered quad, resulting in a pixellated look.
    for (mut transform, mut f) in &mut query {
        if transform.translation.y < -500. {
            transform.translation.y = 500.;
            f.speed=get_init_speed();
        }
        f.speed=f.speed+ACCELERATION*time.delta_secs();
        transform.translation.y -= f.speed;
    }
}

fn get_fall_char(index:i32)->String{
    let charset=vec!["甲","乙","丙","丁","戊","己","庚","辛","壬","癸","子","丑","寅","卯","辰","巳","午","未","申","酉","戌","亥"];
    let i:usize=(index.abs() as usize)%charset.len();
    return charset[i].to_string();
}

fn get_init_speed()->f32{
     let mut rng = rand::rng();
     return rng.random_range(20.0..40.0);
}
