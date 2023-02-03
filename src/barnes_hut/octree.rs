use super::particle::Particle;
use super::vector::Vector;

const K: f64 = 8.988;

#[derive(Debug, Clone)]
pub struct Cell {
    children: Vec<Cell>,
    particle: Option<Particle>,
    count: f64,
    length: f64,
    base: Vector,
    charge: f64,
    center_of_charge: Vector, //center of charge, center of mass
}

impl Cell {
    pub fn new(length: f64, base: Vector) -> Cell {
        Cell {
            children: Vec::<Cell>::new(),
            particle: None,
            count: 0.0,
            length,
            base,
            charge: 0.0,
            center_of_charge: Vector::new(0.0, 0.0, 0.0),
        }
    }

    pub fn is_inside(&self, pt: &Vector) -> bool {
        if self.base.x <= pt.x
            && self.base.x + self.length > pt.x
            && self.base.y <= pt.y
            && self.base.y + self.length > pt.y
            && self.base.z <= pt.z
            && self.base.z + self.length > pt.z
        {
            return true;
        }
        false
    }

    pub fn create_subcells(&mut self) {
        self.children.push(Cell::new(
            self.length / 2.0,
            Vector::new(self.base.x, self.base.y, self.base.z),
        ));
        self.children.push(Cell::new(
            self.length / 2.0,
            Vector::new(self.base.x, self.base.y, self.base.z + self.length / 2.0),
        ));
        self.children.push(Cell::new(
            self.length / 2.0,
            Vector::new(self.base.x, self.base.y + self.length / 2.0, self.base.z),
        ));
        self.children.push(Cell::new(
            self.length / 2.0,
            Vector::new(self.base.x + self.length / 2.0, self.base.y, self.base.z),
        ));
        self.children.push(Cell::new(
            self.length / 2.0,
            Vector::new(
                self.base.x,
                self.base.y + self.length / 2.0,
                self.base.z + self.length / 2.0,
            ),
        ));
        self.children.push(Cell::new(
            self.length / 2.0,
            Vector::new(
                self.base.x + self.length / 2.0,
                self.base.y,
                self.base.z + self.length / 2.0,
            ),
        ));
        self.children.push(Cell::new(
            self.length / 2.0,
            Vector::new(
                self.base.x + self.length / 2.0,
                self.base.y + self.length / 2.0,
                self.base.z,
            ),
        ));
        self.children.push(Cell::new(
            self.length / 2.0,
            Vector::new(
                self.base.x + self.length / 2.0,
                self.base.y + self.length / 2.0,
                self.base.z + self.length / 2.0,
            ),
        ));
    }

    pub fn insert_particle(&mut self, particle: &Particle) {
        if self.count > 1.0 {
            let subcell = self
                .children
                .iter_mut()
                .find(|c| c.is_inside(&particle.position));
            if let Some(c) = subcell {
                c.insert_particle(particle);
            }
        } else if self.count == 1.0 {
            self.create_subcells();

            if let Some(p) = &self.particle {
                let subcell_existing = self.children.iter_mut().find(|c| c.is_inside(&p.position));
                if let Some(c) = subcell_existing {
                    c.insert_particle(p);
                }
            }
            self.particle = None;

            let subcell = self
                .children
                .iter_mut()
                .find(|c| c.is_inside(&particle.position));
            if let Some(c) = subcell {
                c.insert_particle(particle);
            }
        } else if self.count == 0.0 {
            self.particle = Some(particle.clone());
        }

        self.count += 1.0;
    }

    pub fn tree_construction(&mut self, particles: &[Particle]) {
        for particle in particles.iter() {
            self.insert_particle(particle);
        }
    }

    pub fn calculate_charge_distribution(&mut self) {
        if self.count == 1.0 {
            match &self.particle {
                Some(p) => {
                    self.center_of_charge = p.position.clone();
                    self.charge = p.charge;
                }
                None => (),
            }
        } else if self.count != 0.0 {
            for child in self.children.iter_mut() {
                child.calculate_charge_distribution();
                self.charge += child.charge;
                self.center_of_charge = self
                    .center_of_charge
                    .add(&child.center_of_charge.scalar_multiplication(child.charge));
            }
            self.center_of_charge = self
                .center_of_charge
                .scalar_multiplication(1.0 / self.charge);
        }
    }

    pub fn calculate_coulomb_force(&self, particle: &Particle, accuracy_parameter: f64) -> Vector {
        let r_magnitude = self.center_of_charge.distance(&particle.position);
        let mut force = Vector::new(0.0, 0.0, 0.0);
        if r_magnitude == 0.0 || self.count == 0.0 {
            force
        } else if self.count == 1.0 || self.length / r_magnitude < accuracy_parameter {
            let distance = particle.position.distance_vector(&self.center_of_charge);
            let scalar =
                self.charge * particle.charge * K / (r_magnitude * r_magnitude * r_magnitude);
            force = distance.scalar_multiplication(scalar);
            force
        } else {
            for child in self.children.iter() {
                force = force.add(&child.calculate_coulomb_force(particle, accuracy_parameter));
            }
            force
        }
    }
}
