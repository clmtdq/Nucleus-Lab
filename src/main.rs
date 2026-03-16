use macroquad::prelude::*;
use ::rand::Rng;
use macroquad::math::Vec2;

#[derive(Clone, Copy, Debug, PartialEq)]
enum ParticleType {
    Proton,
    Neutron,
    Electron,
}

#[derive(Clone, Copy, Debug)]
struct Particle {
    ptype: ParticleType,
    pos: Vec2,
    vel: Vec2,
    acc: Vec2,
    charge: f32,
    mass: f32,
    is_captured: bool,
    offset: Vec2,
}

#[derive(Clone, Debug)]
struct Atom {
    //name: String,
    pos: Vec2,
    vel: Vec2,
    acc: Vec2,
    charge: f32,
    mass: f32,
    comp: Vec<Particle>,
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

fn hit(particles: &mut Vec<Particle>, atoms: &mut Vec<Atom>) {
    let radius_sum = 20.0;
    let mut to_remove = Vec::new();

    for i in 0..particles.len() {
        for j in 0..particles.len() {
            if i == j { continue; }

            let delta = particles[j].pos - particles[i].pos;
            let distance = delta.length();

            if distance < radius_sum {
                if (particles[i].ptype == ParticleType::Proton && particles[j].ptype == ParticleType::Neutron) || 
                   (particles[i].ptype == ParticleType::Neutron && particles[j].ptype == ParticleType::Proton) {
                    let atom_pos = (particles[i].mass * particles[i].pos + particles[j].mass * particles[j].pos) / (particles[i].mass + particles[j].mass);
                    particles[i].offset = particles[i].pos - atom_pos;
                    particles[i].is_captured = true;
                    particles[j].offset = particles[j].pos - atom_pos;
                    particles[j].is_captured = true;

                    atoms.push(Atom {
                        //name: "*".to_string(),
                        pos: atom_pos,
                        vel: (particles[i].mass * particles[i].vel + particles[j].mass * particles[j].vel) / (particles[i].mass + particles[j].mass),
                        acc: Vec2::ZERO,
                        charge: particles[i].charge + particles[j].charge,
                        mass: particles[i].mass + particles[j].mass,
                        comp: vec![particles[i].clone(), particles[j].clone()],
                    });
                    to_remove.push(i);
                    to_remove.push(j);
                    break;
                } else {}
            }
            // if (particles[i].pos.x >= particles[j].pos.x - min_dist && particles[i].pos.x <= particles[j].pos.x + min_dist) && 
            //     (particles[i].pos.y >= particles[j].pos.y - min_dist && particles[i].pos.y <= particles[j].pos.y + min_dist) {

            // }
        }
    }
    to_remove.sort_unstable();
    to_remove.dedup();
    for &idx in to_remove.iter().rev() {
        if idx < particles.len() {
            particles.remove(idx);
        }
    }
}

#[macroquad::main("Atom Game")]
async fn main() {
    let mut particles: Vec<Particle> = Vec::new();
    let mut atoms: Vec<Atom> = Vec::new();
    let radius = 10.0; 
    let shell_color = Color::new(0.5, 0.5, 0.8, 0.1);

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
                ptype: ParticleType::Proton,
                pos: Vec2::new(x, y),
                vel: Vec2::new(nx, ny),
                acc: Vec2::ZERO,
                charge: 1.0,
                mass: 1.0,
                is_captured: false,
                offset: Vec2::ZERO,
            });
        } else if i == 5 {
            particles.push(Particle {
                ptype: ParticleType::Neutron,
                pos: Vec2::new(x, y),
                vel: Vec2::new(nx, ny),
                acc: Vec2::ZERO,
                charge: 0.0,
                mass: 1.2,
                is_captured: false,
                offset: Vec2::ZERO,
            });
        } else {
            particles.push(Particle {
                ptype: ParticleType::Electron,
                pos: Vec2::new(x, y),
                vel: Vec2::new(nx, ny),
                acc: Vec2::ZERO,
                charge: -1.0,
                mass: 1.0,
                is_captured: false,
                offset: Vec2::ZERO,
            });
        }
    }

    loop {
        clear_background(BLACK);

        repulsion(&mut particles);

        hit(&mut particles, &mut atoms);

        for atom in atoms.iter_mut() {
            if atom.pos.x - radius < 0.0 || atom.pos.x > screen_width() {
                atom.vel.x *= -1.0;
            } else if atom.pos.y - radius < 0.0 || atom.pos.y > screen_height() {
                atom.vel.y *= -1.0;
            }
            atom.pos += atom.vel;
        }

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

        for atom in atoms.iter_mut() {
            draw_circle(atom.pos.x, atom.pos.y, 20.0, shell_color);
            for p in atom.comp.iter_mut() {
                p.pos = atom.pos + p.offset;
                p.vel = atom.vel;
                let color = if p.charge > 0.0 { RED } else { GREEN };
                draw_circle(p.pos.x, p.pos.y, 8.0, color);
            }
        }

        next_frame().await
    }
}