use super::{constants::FIELD_SPACING, player::Player, types::Vector};

pub struct Ray {
    pub direction: Vector,
}

impl Ray {
    pub fn get_ray_direction(&self, player: Player, camera_x: f64) -> Vector {
        let mut ray = Vector {
            x: camera_x * player.plane.x + player.direction.x,
            y: camera_x * player.plane.y + player.direction.y,
        };
        ray.normalize();
        ray
    }

    pub fn get_side_distance(&self, player: Player, map_square: Vector) -> Vector {
        let lx = FIELD_SPACING / 2.0 - (player.position.x - map_square.x);
        let ly = FIELD_SPACING / 2.0 - (player.position.y - map_square.y);
        Vector {
            x: lx / self.direction.x,
            y: ly / self.direction.y,
        }
    }

    pub fn get_deltas(&self) -> Vector {
        Vector {
            x: FIELD_SPACING / (1.0 - self.direction.x.powi(2)),
            y: FIELD_SPACING / (1.0 - self.direction.y.powi(2)),
        }
    }
}
