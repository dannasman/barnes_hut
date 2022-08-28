const K: f64 = 8.988;

#[derive(Debug)]
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

pub struct Cell  {
    pub children: Vec<Cell>,
    pub weighted_position: Vector,
    pub charge: f64,
    pub count: f64,
    pub length: f64,
    pub base: Vector,
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
            weighted_position: Vector {x: 0.0, y: 0.0, z: 0.0},
            charge: 0.0,
            count: 0.0,
            length: length,
            base: base
        }
    }
}

pub fn tree_construction(particles: &Vec<Particle>, cell: &mut Cell) {
    let mut c = 0.0;
    let mut total_charge = 0.0;
    let mut sum_vector = Vector   {
        x: 0.0,
        y: 0.0,
        z: 0.0
    };

    for particle in particles.iter()    {
        if cell.is_inside(&particle.position) {
            c += 1.0;
            total_charge += particle.charge;
            sum_vector = sum_vector.add(&particle.position.scalar_multiplication(particle.charge));
        }
        if c > 1.0  {
            break;
        }
    }
    if c == 0.0 {
        return;
    }

    if c == 1.0 {
        cell.charge = total_charge;
        cell.weighted_position = sum_vector;
        cell.count = c;
        return;
    }

    let mut cell000 = Cell::new(cell.length/2.0, Vector::new(cell.base.x, cell.base.y, cell.base.z));
    let mut cell001 = Cell::new(cell.length/2.0, Vector::new(cell.base.x, cell.base.y, cell.base.z+cell.length/2.0));
    let mut cell010 = Cell::new(cell.length/2.0, Vector::new(cell.base.x, cell.base.y+cell.length/2.0, cell.base.z));
    let mut cell100 = Cell::new(cell.length/2.0, Vector::new(cell.base.x+cell.length/2.0, cell.base.y, cell.base.z));
    let mut cell011 = Cell::new(cell.length/2.0, Vector::new(cell.base.x, cell.base.y+cell.length/2.0, cell.base.z+cell.length/2.0));
    let mut cell101 = Cell::new(cell.length/2.0, Vector::new(cell.base.x+cell.length/2.0, cell.base.y, cell.base.z+cell.length/2.0));
    let mut cell110 = Cell::new(cell.length/2.0, Vector::new(cell.base.x+cell.length/2.0, cell.base.y+cell.length/2.0, cell.base.z));
    let mut cell111 = Cell::new(cell.length/2.0, Vector::new(cell.base.x+cell.length/2.0, cell.base.y+cell.length/2.0, cell.base.z+cell.length/2.0));

    tree_construction(particles, &mut cell000);
    tree_construction(particles, &mut cell001);
    tree_construction(particles, &mut cell010);
    tree_construction(particles, &mut cell100);
    tree_construction(particles, &mut cell011);
    tree_construction(particles, &mut cell101);
    tree_construction(particles, &mut cell110);
    tree_construction(particles, &mut cell111);

    cell.children.push(cell000);
    cell.children.push(cell001);
    cell.children.push(cell010);
    cell.children.push(cell100);
    cell.children.push(cell011);
    cell.children.push(cell101);
    cell.children.push(cell110);
    cell.children.push(cell111);

    cell.charge = cell.children.iter().map(|c| c.charge).sum();
    cell.weighted_position = cell.children.iter().fold(Vector::new(0.0, 0.0, 0.0), |sv, c| sv.add(&c.weighted_position));
    cell.count = cell.children.iter().map(|c| c.count).sum();
}
