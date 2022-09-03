use std::time::Instant;
use std::fs::File;
use std::io::Write;
mod barnes_hut;

fn main() {
    let pi = std::f64::consts::PI;
    let base = barnes_hut::Vector::new(-10.0, -10.0, -10.0);
    let mut cell = barnes_hut::Cell::new(20.0, base);
    let mut particles = Vec::new();
    let mut file = File::create("data.csv").unwrap();
    writeln!(file, "x,y,z,F").unwrap();
    let f = 200.0;
    for i in 0..200  {
        for j in  0..200  {
            particles.push(barnes_hut::Particle::new(2.5+5.0*f64::cos(f64::from(i)*2.0*pi/f)*f64::sin(f64::from(j)*1.0*pi/f), 2.5+5.0*f64::sin(f64::from(i)*2.0*pi/f)*f64::sin(f64::from(j)*1.0*pi/f), 3.0+5.0*f64::cos(f64::from(j)*1.0*pi/f), 1.0));
            particles.push(barnes_hut::Particle::new(-2.5+2.5*f64::cos(f64::from(i)*2.0*pi/f)*f64::sin(f64::from(j)*1.0*pi/f), -2.5+2.5*f64::sin(f64::from(i)*2.0*pi/f)*f64::sin(f64::from(j)*1.0*pi/f), -3.0+2.5*f64::cos(f64::from(j)*1.0*pi/f), 1.0));
        }
    }
    let now = Instant::now();
    cell.tree_construction(&particles);
    cell.calculate_charge_distribution();
    for particle in particles.iter()   {
        let force = cell.calculate_coulomb_force(particle, 1.0);
        writeln!(file, "{},{},{},{}", particle.position.x, particle.position.y, particle.position.z, force.magnitude()).unwrap();
        //println!("Force: {:?}", force);
    }
    let elapsed = now.elapsed();
    println!("Time elapsed: {:?}", elapsed);
    //println!("Root cell count: {:?}", cell);
}
