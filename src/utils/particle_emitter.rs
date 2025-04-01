use nannou::geom::Point2;

use super::particle::Particle;

pub struct ParticleEmitter {
    pub origin: Point2,
    pub particles: Vec<Particle>,
    pub forces: Vec<Point2>,
}

impl ParticleEmitter {
    pub fn new(origin: Point2) -> Self {
        Self {
            origin,
            particles: vec![],
            forces: vec![],
        }
    }

    pub fn add_particle(&mut self) -> &mut Particle {
        self.particles.push(Particle::new(self.origin, 100.0));
        self.particles.last_mut().unwrap()
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
