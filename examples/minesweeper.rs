use std::collections::HashSet;

use bevy::prelude::*;
use rand::RngExt;

fn main() {
    println!("270:{},90:{}", 270.0f64.to_radians(), 90.0f64.to_radians());
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(CellVec::default())
        .add_systems(Startup, setup)
        //.add_systems(Update, shoot_cell)
        //.add_systems(Update, rotate)
        .run();
}

fn setup(
    mut commands: Commands,
    mut cellvec: ResMut<CellVec>,
    asset_server: Res<AssetServer>,

    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    commands.spawn(Camera2d);
    let texture = asset_server.load("cells.png");
    let layout = TextureAtlasLayout::from_grid(UVec2::splat(25), 13, 1, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);
    let mut bomb_set = HashSet::new();
    let mut rng = rand::rng();
    while bomb_set.len() < 12usize {
        let i = rng.random_range(0..121);
        bomb_set.insert(i);
        //bomb_set.insert(rng::random)
    }

    for y in 0..11 {
        for x in 0..11 {
            let point = Point::new(x, y);
            let is_bomb = if bomb_set.contains(&(x * y)) {
                true
            } else {
                false
            };
            let cell = Cell::new(is_bomb, point);

            let entity = commands
                .spawn((
                    Sprite::from_atlas_image(
                        texture.clone(),
                        TextureAtlas {
                            layout: texture_atlas_layout.clone(),
                            index: cell.status.get_image(),
                        },
                    ),
                    Transform::from_translation(Vec3::new((x as f32) * 25., (y as f32) * 25., 0.)),
                    cell,
                    Pickable::default(),
                ))
                .observe(cell_click_on::<Pointer<Click>>())
                .id();

            let index = y * 11 + x;
            println!("x:{},y:{},index:{}", x, y, index);
            cellvec.0[index as usize] = entity.to_bits();
        }
    }
    println!("{:?}", cellvec.0);
}

fn cell_click_on<E: EntityEvent + Clone + Reflect>() -> impl Fn(On<E>, Query<(&mut Sprite, &Cell)>)
{
    // An observer closure that captures `new_material`. We do this to avoid needing to write four
    // versions of this observer, each triggered by a different event and with a different hardcoded
    // material. Instead, the event type is a generic, and the material is passed in.
    move |event: On<'_, '_, E>, mut query| {
        if let Ok((mut sprite, &cell)) = query.get_mut(event.event_target()) {
            if let Some(texture_atlas) = &mut sprite.texture_atlas {
                texture_atlas.index = if cell.is_bomb {
                    CellStatus::Bomb.get_image()
                } else {
                    4usize
                };
            }
        };
    }
}

#[derive(Resource)]
struct CellVec([u64; 121usize]);

impl Default for CellVec {
    fn default() -> Self {
        Self([0u64; 121usize])
    }
}

#[derive(Component, Clone, Copy)]
struct Cell {
    is_bomb: bool,
    status: CellStatus,
    point: Point,
}

impl Cell {
    fn new(is_bomb: bool, point: Point) -> Self {
        Cell {
            is_bomb,
            status: CellStatus::New,
            point,
        }
    }
}
#[derive(Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Point { x, y }
    }
}

#[derive(Clone, Copy)]
enum CellStatus {
    New,
    Flag,
    Bomb,
    Bombed,
    Turned(usize),
}
impl CellStatus {
    fn get_image(self: Self) -> usize {
        match self {
            CellStatus::New => {
                return 9usize;
            }
            CellStatus::Flag => {
                return 10usize;
            }
            CellStatus::Bomb => {
                return 11usize;
            }
            CellStatus::Bombed => {
                return 12usize;
            }
            CellStatus::Turned(i) => return i,
        }
    }
}
