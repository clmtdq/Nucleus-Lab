use macroquad::prelude::*;
use ::rand::Rng;
use macroquad::math::Vec2;

struct Particle {
    ptype: String,
    pos: Vec2,
    vel: Vec2,
    acc: Vec2,
    charge: f32,
    mass: f32,
}

fn repulsion(particles: &mut Vec<Particle>) {
    let k = 500.0;
    let min_dist = 20.0;

    for i in 0..particles.len() {
        for j in 0..particles.len() {
            if i == j { continue; }

            let pos_a = particles[i].pos;
            let pos_b = particles[j].pos;
            let delta = pos_b - pos_a;
            let distance = delta.length().max(min_dist);
            let magnitude_strength = k * (particles[i].charge * particles[j].charge) / (distance * distance);
            let vectorial_strength = (delta/distance) * - magnitude_strength;

            let particle_a = &mut particles[i];
            particle_a.acc += vectorial_strength / particle_a.mass;
        }
    }
}

#[macroquad::main("Atom Game")]
async fn main() {
    let mut particles: Vec<Particle> = Vec::new();
    let radius = 10.0; 

    for i in 0..=5 {
        let mut rng = ::rand::thread_rng();

        let xrand: f32 = rng.gen_range(0.0..=screen_width());
        let yrand: f32 = rng.gen_range(0.0..=screen_height());

        let x: f32 = xrand;
        let y: f32 = yrand;
        let nx = rng.gen_range(-3.0..=3.0);
        let ny = rng.gen_range(-3.0..=3.0);

        if i % 2 == 0 {
            particles.push(Particle {
                ptype: "Proton".to_string(),
                pos: Vec2::new(x, y),
                vel: Vec2::new(nx, ny),
                acc: Vec2::ZERO,
                charge: 1.0,
                mass: 1.0,
            });
        } else if i == 5 {
            particles.push(Particle {
                ptype: "Neutron".to_string(),
                pos: Vec2::new(x, y),
                vel: Vec2::new(nx, ny),
                acc: Vec2::ZERO,
                charge: 0.0,
                mass: 1.0,
            });
        } else {
            particles.push(Particle {
                ptype: "Electron".to_string(),
                pos: Vec2::new(x, y),
                vel: Vec2::new(nx, ny),
                acc: Vec2::ZERO,
                charge: -1.0,
                mass: 1.0,
            });
        }
    }

    loop {
        clear_background(BLACK);

        repulsion(&mut particles);

        for particle in particles.iter_mut() {
            particle.vel += particle.acc;
            particle.pos += particle.vel;
            particle.acc = Vec2::ZERO;

            if particle.pos.x - radius < 0.0 || particle.pos.x > screen_width() {
                particle.vel.x *= -1.0;
            } else if particle.pos.y - radius < 0.0 || particle.pos.y > screen_height() {
                particle.vel.y *= -1.0;
            }
        }

        for particle in &particles {
            let color = if particle.charge > 0.0 { RED } else if particle.charge < 0.0 { BLUE } else { GREEN };
            draw_circle(particle.pos.x, particle.pos.y, radius, color);
        }

        next_frame().await
    }
}