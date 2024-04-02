use plotters::prelude::*;

const TIME_STEP: f64 = 0.01; // s
const STEPS: u32 = 100000000;
const ANIMATION_FPS: u32 = 30;
const ANIMATION_LENGTH: u32 = 40; // s

#[derive(Clone, Debug)]
struct Body {
    mass: f64,          // g
    velocity: Velocity, // m/s
    position: Coordinate,
}

#[derive(Clone, Debug)]

struct Coordinate {
    x: f64, // m
    y: f64, // m
}

#[derive(Clone, Debug)]

struct Velocity {
    vx: f64, // m/s
    vy: f64, // m/s
}

#[derive(Debug, Clone)]
struct Step {
    time: f64, // s from 0
    step: u32, // step number
    bodies: [Body; 3],
}

fn main() {
    println!("Simulating three bodies in a 2D vaccum with no external forces.");
    let mut first_body = Body {
        mass: 1.0,
        velocity: Velocity { vx: 0.0, vy: 0.0 },
        position: Coordinate {
            x: 0.3089693008,
            y: 0.4236727692,
        },
    };

    let mut second_body = Body {
        mass: 1.0,
        velocity: Velocity { vx: 0.0, vy: 0.0 },
        position: Coordinate { x: -0.5, y: 0.0 },
    };

    let mut third_body = Body {
        mass: 1.0,
        velocity: Velocity { vx: 0.0, vy: 0.0 },
        position: Coordinate { x: 0.5, y: 0.0 },
    };

    let mut steps: Vec<Step> = vec![];

    for n in 0..STEPS {
        let mut new_step = Step {
            time: n as f64 * TIME_STEP,
            step: n,
            bodies: [first_body, second_body, third_body],
        };

        for i in 0..3 {
            for j in 0..3 {
                if i != j {
                    let dx = new_step.bodies[j].position.x - new_step.bodies[i].position.x;
                    let dy = new_step.bodies[j].position.y - new_step.bodies[i].position.y;
                    let r = (dx * dx + dy * dy).sqrt();
                    let force =
                        6.67430e-11 * new_step.bodies[i].mass * new_step.bodies[j].mass / r / r;
                    let angle = dy.atan2(dx);
                    let fx = force * angle.cos();
                    let fy = force * angle.sin();
                    new_step.bodies[i].velocity.vx += fx / new_step.bodies[i].mass * TIME_STEP;
                    new_step.bodies[i].velocity.vy += fy / new_step.bodies[i].mass * TIME_STEP;
                }
            }
        }

        for i in 0..3 {
            new_step.bodies[i].position.x += new_step.bodies[i].velocity.vx * TIME_STEP;
            new_step.bodies[i].position.y += new_step.bodies[i].velocity.vy * TIME_STEP;
        }

        first_body = new_step.bodies[0].clone();
        second_body = new_step.bodies[1].clone();
        third_body = new_step.bodies[2].clone();

        if n % 1000 == 0 {
            println!("Finished step {}", n);
        }
        if n % (STEPS / (ANIMATION_LENGTH * ANIMATION_FPS)) == 0 {
            steps.push(new_step);
        }
    }
    println!(
        "Finished simulating {} steps. Generating visualization...",
        STEPS
    );
    graph_steps(&steps);
    animate_steps(&steps);
    println!("Done!");
}

fn graph_steps(steps: &[Step]) {
    println!("Generating single PNG file...");
    let area = BitMapBackend::new("three_body.png", (250, 250)).into_drawing_area();
    let mut ctx = ChartBuilder::on(&area)
        .build_cartesian_2d(-100..100, -100..100)
        .unwrap();
    area.fill(&WHITE).unwrap();
    ctx.configure_mesh().draw().unwrap();
    for n in 0..3 {
        let color = match n {
            0 => BLUE,
            1 => RED,
            2 => GREEN,
            _ => BLACK,
        };
        ctx.draw_series(steps[1..].iter().map(|step| {
            Circle::new(
                (
                    (step.bodies[n].position.x * 100.0).round() as i32,
                    (step.bodies[n].position.y * 100.0).round() as i32,
                ),
                1,
                color.filled(),
            )
        }))
        .unwrap();
    }
    for n in 0..3 {
        ctx.draw_series([steps[0].clone()].iter().map(|step| {
            Circle::new(
                (
                    (step.bodies[n].position.x * 100.0).round() as i32,
                    (step.bodies[n].position.y * 100.0).round() as i32,
                ),
                2,
                BLACK.filled(),
            )
        }))
        .unwrap();
    }
    area.present().unwrap();
}

fn animate_steps(steps: &[Step]) {
    println!("Generating animation...");
    let area = BitMapBackend::gif("three_body.gif", (250, 250), 1000 / ANIMATION_FPS)
        .unwrap()
        .into_drawing_area();
    let mut ctx = ChartBuilder::on(&area)
        .build_cartesian_2d(-100..100, -100..100)
        .unwrap();
    for step in steps {
        println!("Rendering frame {}", step.step);
        area.fill(&WHITE).unwrap();
        ctx.configure_mesh().draw().unwrap();

        for n in 0..3 {
            let color = match n {
                0 => BLUE,
                1 => RED,
                2 => GREEN,
                _ => BLACK,
            };
            ctx.draw_series([step.clone()].iter().map(|step| {
                Circle::new(
                    (
                        (step.bodies[n].position.x * 100.0).round() as i32,
                        (step.bodies[n].position.y * 100.0).round() as i32,
                    ),
                    2,
                    color.filled(),
                )
            }))
            .unwrap();
        }
        area.draw(&Text::new(
            format!("T : {}", step.time.round() as u32),
            (5, 5),
            ("sans-serif", 12),
        ))
        .unwrap();
        area.present().unwrap();
    }
}
