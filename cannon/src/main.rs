use tuples::{normalize, point, vector, Tuple};

struct Environment {
    gravity: Tuple,
    wind: Tuple,
}

struct Projectile {
    position: Tuple,
    velocity: Tuple,
}

fn tick(environment: &Environment, projectile: &mut Projectile) {
    projectile.position = projectile.position + projectile.velocity;
    projectile.velocity = projectile.velocity + environment.gravity + environment.wind;
}

fn print_projectile_position(projectile: &Projectile) {
    println!(
        "Projectile position is ({}, {}, {})",
        projectile.position.x, projectile.position.y, projectile.position.z
    );
}

fn print_projectile_velocity(projectile: &Projectile) {
    println!(
        "Projectile velocity is ({}, {}, {})",
        projectile.velocity.x, projectile.velocity.y, projectile.velocity.z
    );
}

fn main() {
    let mut ticks = 0;
    let environment = Environment {
        gravity: vector(0.0, -0.1, 0.0),
        wind: vector(-0.01, 0.0, 0.0),
    };
    let mut projectile = Projectile {
        position: point(0.0, 1.0, 0.0),
        velocity: normalize(vector(1.0, 1.0, 0.0)),
    };

    loop {
        tick(&environment, &mut projectile);
        ticks += 1;
        print_projectile_position(&projectile);
        print_projectile_velocity(&projectile);
        if projectile.position.y <= 0.0 {
            println!("Crashed at ticks equal to {}", ticks);
            break;
        }
    }
}
