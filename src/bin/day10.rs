use anyhow::Result;

fn strength(cycle: i32, x: i32, signal: &mut i32) {
    let checkpoints = vec![20, 60, 100, 140, 180, 220];
    if checkpoints.contains(&cycle) {
        *signal += x * cycle;
    }
}

fn draw(cycle: i32, x: i32, crt: &mut String) {
    if (cycle - 1) % 40 >= x - 1 && (cycle - 1) % 40 <= x + 1 {
        *crt += "#";
    } else {
        *crt += ".";
    }
}

fn main() -> Result<()> {
    let mut x = 1;
    let mut cycle = 1;
    let mut signal = 0;
    let mut crt = String::from("");

    include_str!("./day10.txt").lines().for_each(|line| {
        let loops = if line == "noop" { 1 } else { 2 };

        for i in 0..loops {
            draw(cycle, x, &mut crt);
            cycle += 1;
            if i == 1 {
                x += line.split_once(' ').unwrap().1.parse::<i32>().unwrap();
            }
            strength(cycle, x, &mut signal);
        }
    });

    println!("Sigal: {}, X: {}, Cycle: {}", signal, x, cycle);

    for i in 0..6 {
        println!("{}", &crt[i * 40..(i + 1) * 40]);
    }
    Ok(())
}
