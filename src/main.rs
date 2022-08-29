use std::io;
use std::time::Instant;
mod barnes_hut;

fn main() {
    let base = barnes_hut::Vector::new(0.0, 0.0, 0.0);
    let mut cell = barnes_hut::Cell::new(10.0, base);
    let mut particles = Vec::new();
    
    for i in 0..58  {
        for j in  0..58  {
            for k in 0..58  {
                particles.push(barnes_hut::Particle::new(f64::from(i)*10.0/58.0, f64::from(j)*10.0/58.0, f64::from(k)*10.0/58.0, 1.0));
            }
        }
    }
    let now = Instant::now();
    cell.tree_construction(&particles);
    let elapsed = now.elapsed();
    println!("Time elapsed: {:?}", elapsed);
    println!("Root cell count: {:?}", cell.count);
}
