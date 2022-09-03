use super::vector::Vector;

#[derive(Debug, Clone)]
pub struct Particle {
    pub position: Vector,
    pub charge: f64,
}

impl Particle   {
    /*pub fn coulomb_force(&self, p: Particle) -> Vector  {
        let r_magnitude = self.position.distance(&p.position);
        let d = self.position.vector_distance(&p.position);
        let scalar = self.charge*p.charge*K/r_magnitude;
        return d.scalar_multiplication(scalar);
    }*/

    pub fn new(x: f64, y: f64, z: f64, q: f64) -> Particle  {
        return Particle {
            position: Vector::new(x, y, z),
            charge: q
        }
    }
}