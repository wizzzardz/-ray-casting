use super::{constants::FIELD_SPACING, player::Player, types::Vector};

pub struct Ray<'a> {
    pub direction: Vector,
    pub player: &'a Player,
}

impl<'a> Ray<'a> {
    pub fn new(player: &'a Player) -> Self {
        let mut ray = Vector {
            x: -1.0 * player.plane.x + player.direction.x,
            y: -1.0 * player.plane.y + player.direction.y,
        };
        ray.normalize();
        Ray {
            direction: ray,
            player,
        }
    }

    pub fn update_direction(&mut self, camera_x: f64) {
        let mut ray = Vector {
            x: camera_x * self.player.plane.x + self.player.direction.x,
            y: camera_x * self.player.plane.y + self.player.direction.y,
        };
        ray.normalize();
        self.direction = ray;
    }

    pub fn get_initial_deltas(&self, player: Player) -> Vector {
        let map_square = Vector {
            x: player.position.x.floor() + 0.5,
            y: player.position.y.floor() + 0.5,
        };
        let lx = FIELD_SPACING / 2.0 - (player.position.x - map_square.x);
        let ly = FIELD_SPACING / 2.0 - (player.position.y - map_square.y);
        let mut v = Vector {
            x: lx / self.direction.x,
            y: ly / self.direction.y,
        };
        v.handle_infinite();
        v
    }

    pub fn get_deltas(&self) -> Vector {
        let mut v = Vector {
            x: FIELD_SPACING / (1.0 - self.direction.x.powi(2)),
            y: FIELD_SPACING / (1.0 - self.direction.y.powi(2)),
        };
        v.handle_infinite();
        v
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_ray() {
        let player = Player::new(1.0, 2.0);
        let ray = Ray::new(&player);
    }
}

// TODO test ray
// DDA algo
// Perform fisheye correction
