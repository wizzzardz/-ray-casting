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

    pub fn get_initial_deltas(&self) -> Vector {
        let map_square = Vector {
            x: self.player.position.x.floor() + 0.5,
            y: self.player.position.y.floor() + 0.5,
        };
        let lx = FIELD_SPACING / 2.0 - (self.player.position.x - map_square.x);
        let ly = FIELD_SPACING / 2.0 - (self.player.position.y - map_square.y);
        dbg!(lx);
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
        assert!(ray.direction.x - 0.834609 < 1e-6);
        assert!(ray.direction.y + 0.550842 < 1e-6);
    }

    #[test]
    fn test_update_ray_direction() {
        let player = Player::new(1.0, 2.0);
        let mut ray = Ray::new(&player);
        ray.update_direction(1.0);
        assert!(ray.direction.x - 0.834609 < 1e-6);
        assert!(ray.direction.y - 0.550842 < 1e-6);
        ray.update_direction(0.0);
        assert!(ray.direction.x - 1.0 < 1e-6);
        assert!(ray.direction.y < 1e-6);
    }

    #[test]
    fn test_get_initial_deltas() {
        let mut player = Player::new(1.4, 2.3);
        player.turn(45.0);
        let mut ray = Ray::new(&player);
        ray.update_direction(0.0);
        let deltas = ray.get_initial_deltas();
        assert!(0.6 / (45.0 / 180.0 * std::f64::consts::PI).cos() - deltas.x < 1e-6);
        assert!(0.7 / (45.0 / 180.0 * std::f64::consts::PI).sin() - deltas.y < 1e-6);
    }

    #[test]
    fn test_get_deltas() {
        let mut player = Player::new(1.4, 2.3);
        player.turn(45.0);
        let mut ray = Ray::new(&player);
        ray.update_direction(0.0);
        let deltas = ray.get_deltas();
        assert!(1.0 / (45.0 / 180.0 * std::f64::consts::PI).cos() - deltas.x < 1e-6);
        assert!(1.0 / (45.0 / 180.0 * std::f64::consts::PI).sin() - deltas.y < 1e-6);
    }
}

// TODO test ray
// DDA algo
// Perform fisheye correction
