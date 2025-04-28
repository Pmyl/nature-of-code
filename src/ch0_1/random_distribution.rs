use nannou::App;
use nannou::{
    color::{BLACK, GRAY, WHITE},
    rand::random_f32,
    Draw,
};
use nature_of_code::{ExerciseData, ExerciseRunner, ExerciseState};
use rand::{thread_rng, Rng};

pub fn run() {
    ExerciseRunner::run::<Distribution>(ExerciseData::new(540, 240, 2));
}

struct Distribution {
    total: u32,
    random_counts: Vec<u32>,
}

impl ExerciseState for Distribution {
    fn new(_: &ExerciseData, _: &App) -> Self {
        Distribution {
            total: 20,
            random_counts: (0..20).map(|_| 0).collect::<Vec<u32>>(),
        }
    }

    fn show(&self, draw: &Draw, exercise: &ExerciseData) {
        draw.background().color(WHITE);

        let width = exercise.width() as f32 / self.total as f32;

        for (x, ele) in self.random_counts.iter().enumerate() {
            draw.rect()
                .x(x as f32 * width + width / 2.)
                .y(exercise.height() as f32 - *ele as f32 / 2.)
                .width(width - 1.)
                .height(*ele as f32)
                .stroke(BLACK)
                .stroke_weight(1.)
                .color(GRAY);
        }
    }

    fn step(&mut self, _: &ExerciseData) {
        accept_reject_distribution(self.total, &mut self.random_counts);
    }
}

#[allow(unused)]
fn uniform_distribution(total: u32, random_counts: &mut Vec<u32>) {
    let index = f32::floor(random_f32() * total as f32) as usize;
    random_counts[index] += 1;
}

fn accept_reject_distribution(total: u32, random_counts: &mut Vec<u32>) {
    loop {
        let random = thread_rng().gen_range(0..total);
        let qualifying_random = thread_rng().gen_range(0..total);

        if qualifying_random < random {
            random_counts[random as usize] += 1;
            break;
        }
    }
}
