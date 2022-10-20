use std::f64::consts::E;

fn attacker_success_probability(q: f64, z: u64) -> f64 {
    let p = 1.0 - q;
    let lambda = z as f64 * (q / p);
    let mut sum = 1.0;

    for k in 0..z {
        let mut poisson = E.powf(-lambda);

        for i in 1..k + 1 {
            poisson = poisson * (lambda / i as f64);
        }
        sum = sum - poisson * (1.0 - (q / p).powf((z as f64) - (k as f64)));
    }

    return sum;
}

fn main() {
    println!("q=0.1");
    for z in 0..11 {
        let q = attacker_success_probability(0.1, z);
        println!("z={} P={}", z, q);
    }
    println!();

    println!("q=0.3");
    for z in (0..11).map(|i| i * 5) {
        let q = attacker_success_probability(0.3, z);
        println!("z={} P={}", z, q);
    }
    println!();

    println!("P < 0.001");
    let v = vec![
        (0.10, 5),
        (0.15, 8),
        (0.20, 11),
        (0.25, 15),
        (0.30, 24),
        (0.40, 89),
        (0.45, 340),
    ];

    for t in v {
        let q = attacker_success_probability(t.0, t.1);
        println!("z={} q={} P={}", t.1, t.0, q);
        assert!(q < 0.001);
    }
}
