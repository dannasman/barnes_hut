const K: f64 = 8.988;

#[derive(Debug, Clone)]
pub struct Vector    {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vector  {
    pub fn magnitude(&self) -> f64  {
        return self.x*self.x+self.y*self.y+self.z+self.z;
    }

    //squared distance
    pub fn distance(&self, pt: &Vector) -> f64 {
        return (self.x-pt.x)*(self.x-pt.x)+(self.y-pt.y)*(self.y-pt.y)+(self.z-pt.z)*(self.z-pt.z);
    }

    //vector distance
    pub fn vector_distance(&self, pt: &Vector) -> Vector {
        return Vector {
            x: self.x-pt.x,
            y: self.y-pt.y,
            z: self.z-pt.z
        };
    }

    pub fn scalar_multiplication(&self, scalar: f64) -> Vector {
        return Vector   {
            x: scalar*self.x,
            y: scalar*self.y,
            z: scalar*self.z
        }
    }

    pub fn add(&self, pt: &Vector) -> Vector    {
        return Vector   {
            x: self.x+pt.x,
            y: self.y+pt.y,
            z: self.z+pt.z
        }
    }

    pub fn new(x: f64, y: f64, z: f64) -> Vector    {
        return Vector {x, y, z};
    }
}

#[derive(Debug, Clone)]
pub struct Particle {
    pub position: Vector,
    charge: f64,
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


#[derive(Debug, Clone)]
pub struct Cell  {
    children: Vec<Cell>,
    particle: Option<Particle>,
    count: f64,
    length: f64,
    base: Vector,
    charge: f64,
    center_of_charge: Vector
}

impl Cell   {
    pub fn is_inside(&self, pt: &Vector) -> bool   {
        if self.base.x <= pt.x && self.base.x+self.length > pt.x 
            && self.base.y <= pt.y && self.base.y+self.length > pt.y
                && self.base.z <= pt.z && self.base.z+self.length > pt.z {
                    return true
        }
        return false
    }

    pub fn new(length:f64, base: Vector) -> Cell    {
        return Cell {
            children: Vec::<Cell>::new(),
            particle: None,
            count: 0.0,
            length: length,
            base: base,
            charge: 0.0,
            center_of_charge: Vector::new(0.0, 0.0, 0.0)
        }
    }

    pub fn create_subcells(&mut self)    {
        self.children.push(Cell::new(self.length/2.0, Vector::new(self.base.x, self.base.y, self.base.z)));
        self.children.push(Cell::new(self.length/2.0, Vector::new(self.base.x, self.base.y, self.base.z+self.length/2.0)));
        self.children.push(Cell::new(self.length/2.0, Vector::new(self.base.x, self.base.y+self.length/2.0, self.base.z)));
        self.children.push(Cell::new(self.length/2.0, Vector::new(self.base.x+self.length/2.0, self.base.y, self.base.z)));
        self.children.push(Cell::new(self.length/2.0, Vector::new(self.base.x, self.base.y+self.length/2.0, self.base.z+self.length/2.0)));
        self.children.push(Cell::new(self.length/2.0, Vector::new(self.base.x+self.length/2.0, self.base.y, self.base.z+self.length/2.0)));
        self.children.push(Cell::new(self.length/2.0, Vector::new(self.base.x+self.length/2.0, self.base.y+self.length/2.0, self.base.z)));
        self.children.push(Cell::new(self.length/2.0, Vector::new(self.base.x+self.length/2.0, self.base.y+self.length/2.0, self.base.z+self.length/2.0)));
    }


    pub fn insert_particle(&mut self, particle: &Particle)  {
        if self.count > 1.0 {
            let subcell = self.children.iter_mut().find(|c| c.is_inside(&particle.position));
            match subcell {
                Some(c) => {
                    c.insert_particle(particle);
                },
                None => (),
            }
        }

        else if self.count == 1.0    {    
            self.create_subcells();

            if let Some(p) = &self.particle  {
                let subcell_existing = self.children.iter_mut().find(|c| c.is_inside(&p.position));
                match subcell_existing   {
                    Some(c) => {
                        c.insert_particle(&p);
                    },
                    None => (),
                }
            }
            self.particle = None;

            let subcell = self.children.iter_mut().find(|c| c.is_inside(&particle.position));
            match subcell   {
                Some(c) => {
                    c.insert_particle(particle);
                },
                None => (),
            }
        }
        else if self.count == 0.0    {
            self.particle = Some(particle.clone());
        }
    
        self.count += 1.0;
    }

    pub fn tree_construction(&mut self, particles: &Vec<Particle>)  {
        for particle in particles.iter()    {
            self.insert_particle(particle);
        }
    }

    pub fn calculate_charge_distribution(&mut self) {
        if self.count == 0.0  {
            return;
        }
        else if self.count == 1.0  {
            match &self.particle {
                Some(p) => {
                    self.center_of_charge = p.position.clone();
                    self.charge = p.charge;
                },
                None => (),
            }
        }
        else    {
            for child in self.children.iter_mut()    {
                child.calculate_charge_distribution();
                self.charge += child.charge;
                self.center_of_charge = self.center_of_charge.add(&child.center_of_charge.scalar_multiplication(child.charge));
            }
            self.center_of_charge = self.center_of_charge.scalar_multiplication(1.0/self.charge);
        }
    }

    pub fn calculate_coulomb_force(&self, particle: &Particle, accuracy_parameter: f64) -> Vector  {

        let r_magnitude = self.center_of_charge.distance(&particle.position);
        let mut force = Vector::new(0.0, 0.0, 0.0);
        if r_magnitude == 0.0 || self.count == 0.0 {
            return force;
        }
        else if self.count == 1.0 || self.length / r_magnitude < accuracy_parameter {
            let distance = particle.position.vector_distance(&self.center_of_charge);
            let scalar = self.charge*particle.charge*K/r_magnitude;
            force = distance.scalar_multiplication(scalar);
            return force;
        }
        else    {
            for child in self.children.iter()   {
                force = force.add(&child.calculate_coulomb_force(particle, accuracy_parameter));
            }
            return force;
        }
    }
}
