use nalgebra::Vector3;

#[derive(Clone)]
pub struct Sphere {
    pos: Vector3<f64>,
    radius: f64,
}

impl Sphere {
    pub fn new(position: Vector3<f64>, radius: f64) -> Self {
        Sphere {
            pos: position,
            radius: radius,
        }
    }
    pub fn pos(&self) -> Vector3<f64> {
        self.pos
    }

    pub fn radius(&self) -> f64 {
        self.radius
    }
}
