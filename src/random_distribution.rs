use nannou::{
    color::{BLACK, GRAY, WHITE},
    event::Update,
    rand::random_f32,
    App, Draw, Frame,
};
use nature_of_code::Exercise;
use rand::{thread_rng, Rng};

const EXERCISE: Exercise = Exercise::new(540, 240, 2);

pub fn run() {
    nannou::app(model).update(update).run();
}

fn model(app: &App) -> Distribution {
    EXERCISE.init_with_view(app, view);

    Distribution {
        total: 20,
        random_counts: (0..20).map(|_| 0).collect::<Vec<u32>>(),
    }
}

fn update(_app: &App, model: &mut Distribution, _update: Update) {
    model.step();
}

fn view(app: &App, model: &Distribution, frame: Frame) {
    let draw = EXERCISE.draw(app);

    model.show(&draw);

    draw.to_frame(app, &frame).unwrap()
}

struct Distribution {
    total: u32,
    random_counts: Vec<u32>,
}

impl Distribution {
    pub fn show(&self, draw: &Draw) {
        draw.background().color(WHITE);

        let width = EXERCISE.width() as f32 / self.total as f32;

        for (x, ele) in self.random_counts.iter().enumerate() {
            draw.rect()
                .x(x as f32 * width + width / 2.)
                .y(EXERCISE.height() as f32 - *ele as f32 / 2.)
                .width(width - 1.)
                .height(*ele as f32)
                .stroke(BLACK)
                .stroke_weight(1.)
                .color(GRAY);
        }
    }

    pub fn step(&mut self) {
        accept_reject_distribution(self.total, &mut self.random_counts);
    }
}

#[allow(dead_code)]
fn uniform_distribution(total: u32, random_counts: &mut Vec<u32>) {
    let index = f32::floor(random_f32() * total as f32) as usize;
    random_counts[index] += 1;
}

#[allow(dead_code)]
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
