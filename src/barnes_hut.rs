const K: f64 = 8.988;

#[derive(Debug, Clone)]
pub struct Vector    {
    x: f64,
    y: f64,
    z: f64,
}

impl Vector  {
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
    position: Vector,
    charge: f64,
}

impl Particle   {
    pub fn coulomb_force(&self, p: Particle) -> Vector  {
        let r_magnitude = self.position.distance(&p.position);
        let d = self.position.vector_distance(&p.position);
        let scalar = self.charge*p.charge*K/r_magnitude;
        return d.scalar_multiplication(scalar);
    }

    pub fn new(x: f64, y: f64, z: f64, q: f64) -> Particle  {
        return Particle {
            position: Vector::new(x, y, z),
            charge: q
        }
    }
}


#[derive(Debug, Clone)]
pub struct Cell  {
    pub children: Vec<Cell>,
    pub particle: Option<Particle>,
    pub count: f64,
    pub length: f64,
    pub base: Vector,
    pub charge: f64,
    pub center_of_mass: Vector
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
            center_of_mass: Vector::new(0.0, 0.0, 0.0)
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
            let mut subcell = self.children.iter_mut().find(|c| c.is_inside(&particle.position));
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

            let mut subcell = self.children.iter_mut().find(|c| c.is_inside(&particle.position));
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
}
