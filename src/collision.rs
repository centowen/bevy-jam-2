use bevy::prelude::*;
use itertools::Itertools;

pub struct Collision {
    pub entity: Entity,
    pub offset: Vec3,
}

#[derive(Component)]
pub struct Collisions {
    pub collisions: Vec<Collision>,
}

impl Collisions {
    pub fn new() -> Collisions {
        Collisions {
            collisions: Vec::new(),
        }
    }
}

const GRID_POS: Vec2 = Vec2 {
    x: -1280.0 / 2.0,
    y: -768.0 / 2.0,
};
const GRID_WIDTH: f32 = 1280.0;
const GRID_HEIGHT: f32 = 768.0;
const GRID_COLS: usize = 10;
const GRID_ROWS: usize = 10;

#[derive(Debug)]
struct Rect {
    pos: Vec2,
    width: f32,
    height: f32,
}

impl Rect {
    fn contains(&self, pos: Vec2) -> bool {
        pos.x > self.pos.x
            && pos.y > self.pos.y
            && pos.x < self.pos.x + self.width
            && pos.y < self.pos.y + self.height
    }
}

#[derive(Debug)]
struct Grid {
    storage: Vec<Vec<Entity>>,
    dimension: Rect,
    rows: usize,
    cols: usize,
}

impl Grid {
    pub fn new(iter: &mut dyn Iterator<Item = (Entity, &Transform, &Sprite, &Collisions)>) -> Grid {
        let mut grid = Grid {
            storage: vec![Vec::new(); GRID_COLS * GRID_ROWS],
            dimension: Rect {
                pos: GRID_POS,
                width: GRID_WIDTH,
                height: GRID_HEIGHT,
            },
            rows: GRID_ROWS,
            cols: GRID_COLS,
        };
        for (entity, transform, _, _collisions) in
            iter.filter(|t| grid.dimension.contains(t.1.translation.truncate()))
        {
            let offset = grid.cell_offset(transform.translation.truncate());
            grid.storage[offset].push(entity);
        }
        grid
    }

    fn row(&self, pos: Vec2) -> usize {
        let cell_height = self.dimension.height / self.cols as f32;
        f32::floor((pos.y - self.dimension.pos.y) / cell_height) as usize
    }

    fn col(&self, pos: Vec2) -> usize {
        let cell_width = self.dimension.width / self.rows as f32;
        f32::floor((pos.x - self.dimension.pos.x) / cell_width) as usize
    }

    fn cell_offset_rc(&self, row: usize, col: usize) -> usize {
        row * self.cols + col
    }

    fn cell_offset(&self, pos: Vec2) -> usize {
        assert!(self.dimension.contains(pos));
        self.row(pos) * self.cols + self.col(pos)
    }

    fn contains(&self, row: usize, col: usize) -> bool {
        row < self.rows && col < self.cols
    }

    fn get_entities(&self, row: usize, col: usize) -> &[Entity] {
        if !self.contains(row, col) {
            return &[];
        }
        &self.storage[self.cell_offset_rc(row, col)][..]
    }
}

pub fn collide_stuff(mut q_objects: Query<(Entity, &Transform, &Sprite, &mut Collisions)>) {
    for (_, _, _, mut collisions) in q_objects.iter_mut() {
        collisions.collisions.clear();
    }
    let grid = Grid::new(&mut q_objects.iter());
    let mut tmp_collisions = Vec::new();
    for (entity, transform, image, _) in q_objects.iter() {
        // query grid
        let row = grid.row(transform.translation.truncate());
        let col = grid.col(transform.translation.truncate());
        let entities = ((if row == 0 { 1 } else { row } - 1)..(row + 2))
            .cartesian_product((if col == 0 { 1 } else { col } - 1)..(col + 2))
            .flat_map(|(r, c)| grid.get_entities(r, c));
        for e_entity in entities {
            if entity == *e_entity {
                continue;
            }
            let (_, e_transform, e_image, _) = q_objects.get(*e_entity).unwrap();
            if let Some(_) = bevy::sprite::collide_aabb::collide(
                transform.translation,
                image.custom_size.unwrap(),
                e_transform.translation,
                e_image.custom_size.unwrap(),
            ) {
                tmp_collisions.push((entity, *e_entity));
            }
        }
    }
    for collision in tmp_collisions {
        let other_transform = q_objects.get(collision.1).unwrap().1.clone();
        let (_, transform, _, mut collisions) = q_objects.get_mut(collision.0).unwrap();
        collisions.collisions.push(Collision {
            entity: collision.1,
            offset: other_transform.translation - transform.translation,
        });
    }
}
