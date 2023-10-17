fn main() {
    let spaceship1: [usize; 26] = [6, 4, 2, 0, 3, 2, 0, 3, 1, 4, 5, 3, 2, 7, 5, 3, 0, 1, 2, 1, 3, 4, 6, 8, 1, 3];
    let spaceship2: [usize; 18] = [6, 2, 1, 1, 8, 0, 5, 5, 0, 1, 8, 9, 6, 9, 4, 8, 0, 0];
    let spaceship3: [usize; 12] = [0,1,0,2,1,0,1,3,2,1,2,1];
    let spaceship4: [usize; 11] = [0,1,0,2,1,0,3,1,0,1,2];

    assert_eq!(material(&spaceship1), 83);
    assert_eq!(material(&spaceship2), 50);
    assert_eq!(material(&spaceship3), 6);
    assert_eq!(material(&spaceship4), 8);

    println!("{}", material(&spaceship1));
    println!("{}", material(&spaceship2));
    println!("{}", material(&spaceship3));
    println!("{}", material(&spaceship4));
}

fn material(spaceship: &[usize]) -> usize{
    let mut material : usize= 0;
    let mut current_pos: usize = 0;
    let spaceship_size: usize = spaceship.len();

    //get first non 0 element
    while spaceship[current_pos] == 0 {
        current_pos += 1;
    }

    //block that begins the valley
    let mut valley_start: usize = current_pos;

    //height of the the valley
    let mut valley_height: usize = spaceship[current_pos];

    //material that can be placed in the valley so far
    let mut current_material: usize = 0;

    loop {
        current_pos += 1;

        //println!("{} {}", valley_height, current_pos);
        //check if valley has ended successfully, if not start once again from valley_start but with 1 lower height
        if current_pos >= spaceship_size {
            if valley_height == spaceship[current_pos-1] {
                break
            } else {
                current_pos = valley_start +1;
                valley_height -= 1;
                current_material = 0;
            }
        }

        //check if current block is lower than valley_height
            //if yes add appropriate number of blocks to current_material
            //if not start new valley from current position
        if spaceship[current_pos] < valley_height {
            current_material += valley_height - spaceship[current_pos]
        } else {
            material += current_material;
            current_material = 0;
            valley_start = current_pos;
            valley_height = spaceship[current_pos];
        }


    }

    return material;
}
