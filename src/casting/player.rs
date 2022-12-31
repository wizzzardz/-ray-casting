use super::constants::{DISPLACEMENT_STEP, ROTATE_STEP};
use super::types::Vector;

#[derive(PartialEq, Debug)]
pub struct Player {
    pub position: Vector,
    pub direction: Vector,
    pub plane: Vector,
}

impl Player {
    pub fn new(x: f64, y: f64) -> Self {
        Player {
            position: Vector { x, y },
            direction: Vector { x: 1.0, y: 0.0 },
            plane: Vector { x: 0.0, y: 0.66 },
        }
    }

    pub fn walk(&mut self, mut direction: Vector) {
        direction.normalize();
        self.position.x += direction.x * DISPLACEMENT_STEP;
        self.position.y += direction.y * DISPLACEMENT_STEP;
    }

    pub fn turn(&mut self, angle: f64) {
        self.direction.deg_rotate(angle);
        self.plane.deg_rotate(angle);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let player = Player::new(3.0, 6.0);
        assert_eq!(
            Player {
                position: Vector { x: 3.0, y: 6.0 },
                plane: Vector { x: 0.0, y: 0.66 },
                direction: Vector { x: 1.0, y: 0.0 }
            },
            player
        );
    }

    #[test]
    fn test_walk() {
        let mut player = Player::new(3.0, 6.0);
        let direction = Vector::new(1.0, 2.0);
        player.walk(direction);
        dbg!(&player.position);
        assert!((3.447213 - player.position.x).abs() < 10e-6);
        assert!((6.894427 - player.position.y).abs() < 10e-6);
    }

    #[test]
    fn test_turn() {
        let mut player = Player::new(3.0, 6.0);
        player.turn(100.0);
        assert!(player.direction.dot_product(player.plane) < 1e-8);
    }
}
