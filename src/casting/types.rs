use super::constants::INFINITY;

#[derive(PartialEq, Debug)]
pub struct Vector {
    pub x: f64,
    pub y: f64,
}

impl Vector {
    pub fn new(x: f64, y: f64) -> Self {
        let mut v = Vector { x, y };
        v.handle_infinite();
        v
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
        self.x /= norm;
        self.y /= norm;
    }

    pub fn cross_product(&self, vector: Self) -> f64 {
        self.x * vector.y - self.y * vector.x
    }

    pub fn dot_product(&self, vector: Self) -> f64 {
        self.x * vector.x + self.y * vector.y
    }

    pub fn handle_infinite(&mut self) {
        self.x = if self.x.is_infinite() {
            INFINITY
        } else {
            self.x
        };
        self.y = if self.y.is_infinite() {
            INFINITY
        } else {
            self.y
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f64::consts::FRAC_PI_6;

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

    #[test]
    fn test_cross_product() {
        let v_1 = Vector::new(1.0, 3.0);
        let v_2 = Vector::new(4.0, 3.0);
        assert_eq!(-9.0, v_1.cross_product(v_2));
    }

    #[test]
    fn test_cross_product_paralell() {
        let v_1 = Vector::new(1.0, 4.0);
        let v_2 = Vector::new(0.25, 1.0);
        assert_eq!(0.0, v_1.cross_product(v_2));
    }

    #[test]
    fn test_dot_product() {
        let v_1 = Vector::new(1.0, 2.0);
        let v_2 = Vector::new(1.0, 5.0);
        assert_eq!(11.0, v_1.dot_product(v_2));
    }

    #[test]
    fn test_dot_product_perpendicular() {
        let v_1 = Vector::new(0.0, 4.0);
        let v_2 = Vector::new(3.0, 0.0);
        assert_eq!(0.0, v_1.dot_product(v_2));
    }

    #[test]
    fn test_handle_infinite() {
        let mut v = Vector { x: 10.0, y: 4.0 };
        v.x = 4.0 / 0.0;
        v.y = 4.0 / 0.0;
        v.handle_infinite();
        assert_eq!(
            Vector {
                x: INFINITY,
                y: INFINITY
            },
            v
        );
    }
}
