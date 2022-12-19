use super::constants::DISPLACEMENT_STEP;
use super::types::Vector;

struct Player {
    pub position: Vector,
    pub direction: Vector,
}

impl Player {
    pub fn new(x: f64, y: f64) -> Self {
        Player {
            position: Vector { x: x, y: y },
            direction: Vector { x: 0.0, y: 0.0 },
        }
    }

    pub fn displace(&mut self, direction: &mut Vector) {
        direction.normalize();
        self.position.x += direction.x * DISPLACEMENT_STEP;
        self.position.y += direction.y * DISPLACEMENT_STEP;
    }
}
