use super::{constants::FIELD_SPACING, player::Player, types::Vector};

pub struct Ray {
    pub direction: Vector,
}

impl Ray {
    pub fn new(player: &Player) -> Self {
        let mut ray = Vector {
            x: -1.0 * player.plane.x + player.direction.x,
            y: -1.0 * player.plane.y + player.direction.y,
        };
        ray.normalize();
        Ray { direction: ray }
    }

    pub fn update_direction(&mut self, player: &Player, camera_x: f64) {
        let mut ray = Vector {
            x: camera_x * player.plane.x + player.direction.x,
            y: camera_x * player.plane.y + player.direction.y,
        };
        ray.normalize();
        self.direction = ray;
    }

    pub fn get_initial_deltas(&self, player: &Player) -> Vector {
        let map_square = Vector {
            x: player.position.x.floor() + 0.5,
            y: player.position.y.floor() + 0.5,
        };
        let middle = FIELD_SPACING / 2.0;
        let diff_center_x = player.position.x - map_square.x;
        let diff_center_y = player.position.y - map_square.y;
        let (lx, ly) = match self.direction {
            Vector {
                x: x @ 0.0..,
                y: y @ 0.0..,
            } => (middle - diff_center_x, middle - diff_center_y),
            Vector {
                x: x @ f64::NEG_INFINITY..,
                y: y @ 0.0..,
            } => (middle + diff_center_x, middle - diff_center_y),
            // TODO handle all cases
            _ => panic!("not handled"),
        };
        Vector::new(lx / self.direction.x.abs(), ly / self.direction.y.abs())
    }

    pub fn get_deltas(&self) -> Vector {
        Vector::new(
            FIELD_SPACING / (1.0 - self.direction.x.powi(2)),
            FIELD_SPACING / (1.0 - self.direction.y.powi(2)),
        )
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
        ray.update_direction(&player, 1.0);
        assert!(ray.direction.x - 0.834609 < 1e-6);
        assert!(ray.direction.y - 0.550842 < 1e-6);
        ray.update_direction(&player, 0.0);
        assert!(ray.direction.x - 1.0 < 1e-6);
        assert!(ray.direction.y < 1e-6);
    }

    #[test]
    fn test_get_initial_deltas() {
        // TODO test all cases
        //
        // first quarter
        //
        let mut player = Player::new(1.8, 2.7);
        let mut ray = Ray::new(&player);
        // ray direction x-positive y-positive
        player.turn(45.0);
        ray.update_direction(&player, 0.0);
        let deltas = ray.get_initial_deltas(&player);
        assert!(0.2 / (45.0 / 180.0 * std::f64::consts::PI).cos() - deltas.x < 1e-6);
        assert!(0.3 / (45.0 / 180.0 * std::f64::consts::PI).sin() - deltas.y < 1e-6);
        // ray direction x-negative y-positive
        player.turn(90.0);
        ray.update_direction(&player, 0.0);
        let deltas = ray.get_initial_deltas(&player);
        assert!(0.8 / (45.0 / 180.0 * std::f64::consts::PI).cos() - deltas.x < 1e-6);
        assert!(0.3 / (45.0 / 180.0 * std::f64::consts::PI).sin() - deltas.y < 1e-6);

        //
        // third quarter
        //
        let mut player = Player::new(1.4, 2.3);
        player.turn(45.0);
        let mut ray = Ray::new(&player);
        ray.update_direction(&player, 0.0);
        let deltas = ray.get_initial_deltas(&player);
        assert!(0.6 / (45.0 / 180.0 * std::f64::consts::PI).cos() - deltas.x < 1e-6);
        assert!(0.7 / (45.0 / 180.0 * std::f64::consts::PI).sin() - deltas.y < 1e-6);
    }

    #[test]
    fn test_get_deltas() {
        let mut player = Player::new(1.4, 2.3);
        let mut ray = Ray::new(&player);
        player.turn(45.0);
        ray.update_direction(&player, 0.0);
        let deltas = ray.get_deltas();
        assert!(1.0 / (45.0 / 180.0 * std::f64::consts::PI).cos() - deltas.x < 1e-6);
        assert!(1.0 / (45.0 / 180.0 * std::f64::consts::PI).sin() - deltas.y < 1e-6);
    }
}

// TODO test ray
// DDA algo
// Perform fisheye correction
