use super::body::Body;

pub struct Friction(pub f32);

impl Friction {
    pub fn apply_to(self, mover: &mut Body) {
        let friction = (mover.velocity * -1.).normalize() * self.0;

        mover.apply_force(friction);
    }
}
