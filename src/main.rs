use plotters::prelude::*;

const TIME_STEP: f64 = 1.0; // s
const STEPS: i32 = 500000;
const ANIMATION_START: i32 = 0;
const ANIMATION_END: i32 = 500000;

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
    step: i32, // step number
    bodies: [Body; 3],
}

fn main() {
    println!("Simulating three bodies in a 2D vaccum with no external forces.");
    let mut first_body = Body {
        mass: 1.0,
        velocity: Velocity { vx: 0.0, vy: 0.0 },
        position: Coordinate { x: 0.309, y: 0.42 },
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

        println!("{:#?}", new_step);
        steps.push(new_step);
    }
    println!("Finished simulating {} steps. Generating GIF...", STEPS);

    // 5 second gif
    let area = BitMapBackend::gif("three_body.gif", (100, 100), 1)
        .unwrap()
        .into_drawing_area();
    let mut ctx = ChartBuilder::on(&area)
        .build_cartesian_2d(-500..500, -500..500)
        .unwrap();

    for step in steps.iter() {
        if step.step < ANIMATION_START {
            continue;
        } else if step.step > ANIMATION_END {
            break;
        }
        let x0 = (step.bodies[0].position.x * 100.0).round() as i32;
        let y0 = (step.bodies[0].position.y * 100.0).round() as i32;
        let x1 = (step.bodies[1].position.x * 100.0).round() as i32;
        let y1 = (step.bodies[1].position.y * 100.0).round() as i32;
        let x2 = (step.bodies[2].position.x * 100.0).round() as i32;
        let y2 = (step.bodies[2].position.y * 100.0).round() as i32;
        println!(
            "Scaled coordinates: ({}, {}), ({}, {}), ({}, {})",
            x0, y0, x1, y1, x2, y2
        );

        area.fill(&WHITE).unwrap();
        ctx.configure_mesh().draw().unwrap();
        ctx.draw_series(vec![Circle::new((x0, y0), 2, BLUE.filled())])
            .unwrap();
        ctx.draw_series(vec![Circle::new((x1, y1), 2, RED.filled())])
            .unwrap();
        ctx.draw_series(vec![Circle::new((x2, y2), 2, GREEN.filled())])
            .unwrap();
        area.present().unwrap();
        if step.step % 100 == 0 {
            println!("Generating frame {}", step.step);
        }
    }
}
