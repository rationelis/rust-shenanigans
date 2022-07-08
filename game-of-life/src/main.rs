use rand::Rng;
use std::{thread, time};

const WIDTH: usize = 200;
const HEIGHT: usize = 50;

fn main() {
    let mut state = [[0i16; WIDTH]; HEIGHT];
    randomize_grid(&mut state);

    let millies = time::Duration::from_millis(100);

    loop {
        print!("{esc}[2J{esc}[1;1H", esc = 27 as char);

        print_grid(&mut state);

        println!("\n");

        tick(&mut state);
        
        thread::sleep(millies);
    }
}

fn randomize_grid(state: &mut [[i16; WIDTH]; HEIGHT]) {
    let mut rng = rand::thread_rng();
    for element in state.iter_mut().flat_map(|r| r.iter_mut()) {
        *element = rng.gen_range(0..2);
    }
}

fn print_grid(state: &mut [[i16; WIDTH]; HEIGHT]) {
    for row in state.iter_mut() {
        for col in row.iter_mut() {
            print!("{}", col);
        }
        print!("{}", "\n");
    }
}

fn tick(state: &mut [[i16; WIDTH]; HEIGHT]) {
    let mut state_clone = state.clone();

    for i in 0..HEIGHT {
        let i = *&i as i16;

        for j in 0..WIDTH {
            let j = *&j as i16;

            let mut neighbor_count = 0;

            let mut j_left = j.clone();
            j_left -= 1;
            if j_left < 0 {
                j_left = 199;
            }

            let mut j_right = j.clone();
            j_right += 1;
            if j_right >= 199 {
                j_right = 0;
            }

            let mut i_left = i.clone();
            i_left -= 1;
            if i_left < 0 {
                i_left = 49;
            }

            let mut i_right = i.clone();
            i_right += 1;
            if i_right >= 49 {
                i_right = 0;
            }

            // Left column.
            if state[i_left as usize][j_left as usize] == 1 {
                neighbor_count += 1;
            }
            if state[i as usize][j_left as usize] == 1 {
                neighbor_count += 1;
            }
            if state[i_right as usize][j_left as usize] == 1 {
                neighbor_count += 1;
            }

            // Center column.
            if state[i_left as usize][j as usize] == 1 {
                neighbor_count += 1;
            }
            if state[i_right as usize][j as usize] == 1 {
                neighbor_count += 1;
            }

            // Right column.
            if state[i_left as usize][j_right as usize] == 1 {
                neighbor_count += 1;
            }
            if state[i as usize][j_right as usize] == 1 {
                neighbor_count += 1;
            }
            if state[i_right as usize][j_right as usize] == 1 {
                neighbor_count += 1;
            }

            // Any live cell with fewer than two live neighbours dies, as if by underpopulation.
            // Any live cell with two or three live neighbours lives on to the next generation.
            // Any live cell with more than three live neighbours dies, as if by overpopulation.
            // Any dead cell with exactly three live neighbours becomes a live cell, as if by reproduction.
            if state[i as usize][j as usize] == 1 && (neighbor_count == 2 || neighbor_count == 3) {
                state_clone[i as usize][j as usize] = 1;
            } else if state[i as usize][j as usize] == 0 && neighbor_count == 3 {
                state_clone[i as usize][j as usize] = 1;
            } else {
                state_clone[i as usize][j as usize] = 0;
            }
        }
    }

    *state = state_clone;
}
