use super::particle::Particle;
use crate::utils::particle_force::ParticleForce;
use nannou::geom::Point2;

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
            particles_left: None,
        }
    }

    pub fn is_dead(&self) -> bool {
        self.particles_left.unwrap_or(1) == 0 && self.particles.len() == 0
    }

    pub fn add_particle(&mut self) -> Option<&mut Particle> {
        if self.consume_particle() {
            self.particles.push(Particle::new(self.origin, 100.0));
            Some(self.particles.last_mut().unwrap())
        } else {
            None
        }
    }

    pub fn apply_force(&mut self, force: &impl ParticleForce) {
        for i in (0..self.particles.len()).rev() {
            let particle = self.particles.get_mut(i).unwrap();

            particle.apply_force(force);
        }
    }

    pub fn update(&mut self) {
        for i in (0..self.particles.len()).rev() {
            let particle = self.particles.get_mut(i).unwrap();
            particle.update();

            if particle.is_dead() {
                self.particles.remove(i);
            }
        }
    }
    pub fn update_with_forces(&mut self) {
        for i in (0..self.particles.len()).rev() {
            let particle = self.particles.get_mut(i).unwrap();
            particle.update();

            for force in self.forces.iter().copied() {
                particle.apply_force(&force);
            }

            if particle.is_dead() {
                self.particles.remove(i);
            }
        }
    }

    fn consume_particle(&mut self) -> bool {
        match self.particles_left.as_mut() {
            Some(left) if *left > 0 => {
                *left = *left - 1;
                true
            }
            Some(_) => false,
            None => true,
        }
    }
}
