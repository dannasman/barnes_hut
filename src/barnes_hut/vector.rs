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