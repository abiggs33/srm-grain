use srmgrain::*;

fn main() {
    let mut grain = GrainSlice::new(100, 100);

    let (x, y) = (50, 50);
    let radius = 5.0;
    for i in 0..grain.height {
        for j in 0..grain.width {
            let (x, y) = (x as f32, y as f32);
            let (i, j) = (i as f32, j as f32);
            if ((x - i) * (x - i) + (y - j) * (y - j)).sqrt() > radius {
                continue;
            }
            grain.cells[i as usize][j as usize].status = Status::Reached;
        }
    }

    grain.med_field();

    // prints it with ascii into terinal -- just to see if it's working at all
    for y in 0..grain.height {
        for x in 0..grain.width {
            if grain.cells[y][x].time < 30.0 {
                print!(".");
            }
            else {
                print!("#");
            }
        }
        println!();
    }
}
