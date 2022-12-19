#[derive(PartialEq, Debug)]
pub struct Vector {
    pub x: f64,
    pub y: f64,
}

impl Vector {
    pub fn new(x: f64, y: f64) -> Self {
        Vector { x: x, y: y }
    }

    pub fn rad_rotate(&mut self, alpha: f64) {
        let x = self.x;
        self.x = self.x * alpha.cos() - self.y * alpha.sin();
        self.y = x * alpha.sin() + self.y * alpha.cos();
    }

    pub fn deg_rotate(&mut self, alpha: f64) {
        self.rad_rotate(alpha * std::f64::consts::PI / 180.0);
    }

    pub fn normalize(&mut self) {
        let length = self.x.powi(2) + self.y.powi(2);
        let norm = length.sqrt();
        self.x = self.x / norm;
        self.y = self.y / norm;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f64::consts::{FRAC_1_PI, FRAC_2_PI, FRAC_PI_6};

    #[test]
    fn test_normalize() {
        let mut vector = Vector::new(1.0, 2.0);
        vector.normalize();
        assert!(vector.x.abs() - 0.447213 < 1e-6);
        assert!(vector.y.abs() - 0.894427 < 1e-6);
    }

    #[test]
    fn test_rad_rotate() {
        let mut vector = Vector::new(1.0, 2.0);
        // PI/6
        vector.rad_rotate(FRAC_PI_6);
        assert!(vector.x + 0.133974 < 1e-6);
        assert!(vector.y - 2.232050 < 1e-6);
        // 2PI/6
        vector.rad_rotate(FRAC_PI_6);
        dbg!(&vector);
        assert!(vector.x + 1.232050 < 1e-6);
        assert!(vector.y - 1.866025 < 1e-6);
        // 5PI/6
        vector.rad_rotate(3.0 * FRAC_PI_6);
        assert!(vector.x + 1.866025 < 1e-6);
        assert!(vector.y + 1.232050 < 1e-6);
        // 7PI/6
        vector.rad_rotate(2.0 * FRAC_PI_6);
        assert!(vector.x - 0.133974 < 1e-6);
        assert!(vector.y + 2.232050 < 1e-6);
    }

    #[test]
    fn test_deg_rotate() {
        let mut vector = Vector::new(1.0, 2.0);
        // 30
        vector.deg_rotate(30.0);
        assert!(vector.x + 0.133974 < 1e-6);
        assert!(vector.y - 2.232050 < 1e-6);
        // 60
        vector.deg_rotate(30.0);
        dbg!(&vector);
        assert!(vector.x + 1.232050 < 1e-6);
        assert!(vector.y - 1.866025 < 1e-6);
        // 150
        vector.deg_rotate(90.0);
        assert!(vector.x + 1.866025 < 1e-6);
        assert!(vector.y + 1.232050 < 1e-6);
        // 210
        vector.deg_rotate(60.0);
        assert!(vector.x - 0.133974 < 1e-6);
        assert!(vector.y + 2.232050 < 1e-6);
    }
}
