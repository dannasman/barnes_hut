use std::env;
use std::fs::File;
use std::io::{prelude::*, BufReader, Write};
use std::time::Instant;
mod barnes_hut;

use barnes_hut::{Cell, Particle, Vector};

fn main() -> Result<(), &'static str> {
    //read file name of atom structure
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        return Err("Expected only 1 extra argument (file name)");
    }

    let base = Vector::new(-10.0, -10.0, -10.0);
    let mut cell = Cell::new(20.0, base);
    let mut particles = Vec::new();

    //---EXAMPLE USE---
    //file given contains positions in the form of x y z (values divided with space)
    let ni_halfsphere = File::open(&args[1]).unwrap();
    let reader = BufReader::new(ni_halfsphere);
    let lines: Vec<String> = reader
        .lines()
        .map(|l| l.expect("Could not parse line"))
        .collect();

    for l in lines {
        let position = l.split(' ').collect::<Vec<&str>>();
        let x = position[0].parse::<f64>().unwrap();
        let y = position[1].parse::<f64>().unwrap();
        let z = position[2].parse::<f64>().unwrap();

        particles.push(Particle::new(x, y, z, 2.0));
    }

    let now = Instant::now();
    cell.tree_construction(&particles);
    cell.calculate_charge_distribution();

    let mut file = File::create("data.xyz").unwrap();
    writeln!(file, "{}", particles.len()).unwrap();
    writeln!(file, "Ni").unwrap();
    for particle in particles.iter() {
        let force = cell.calculate_coulomb_force(particle, 1.0);
        writeln!(
            file,
            "{} {} {} {}",
            particle.position.x,
            particle.position.y,
            particle.position.z,
            force.magnitude()
        )
        .unwrap();
        //println!("Force: {:?}", force);
    }
    let elapsed = now.elapsed();
    println!("Time elapsed: {:?}", elapsed);
    //println!("Root cell count: {:?}", cell);

    Ok(())
}
