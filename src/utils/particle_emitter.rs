use nannou::geom::Point2;

use super::particle::Particle;

pub struct ParticleEmitter {
    pub origin: Point2,
    pub particles: Vec<Particle>,
    pub forces: Vec<Point2>,
    pub particles_left: Option<usize>,
}

impl ParticleEmitter {
    pub fn new(origin: Point2) -> Self {
        Self {
            origin,
            particles: vec![],
            forces: vec![],
            particles_left: None
        }
    }

    pub fn is_dead(&self) -> bool {
        self.particles_left.unwrap_or(1) == 0 && self.particles.len() == 0
    }

    pub fn add_particle(&mut self) -> Option<&mut Particle> {
        if let Some(left) = self.particles_left.as_mut() {
            if *left > 0 {
                *left = *left - 1;
                self.particles.push(Particle::new(self.origin, 100.0));
                return Some(self.particles.last_mut().unwrap());
            }

            return None;
        }

        self.particles.push(Particle::new(self.origin, 100.0));
        Some(self.particles.last_mut().unwrap())
    }

    pub fn update_with_forces(&mut self) {
        for i in (0..self.particles.len()).rev() {
            let particle = self.particles.get_mut(i).unwrap();
            particle.update();

            for force in self.forces.iter().copied() {
                particle.apply_force(force);
            }

            if particle.is_dead() {
                self.particles.remove(i);
            }
        }
    }
}
