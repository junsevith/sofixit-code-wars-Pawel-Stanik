fn main() {
    let spaceship1: [usize; 26] = [6, 4, 2, 0, 3, 2, 0, 3, 1, 4, 5, 3, 2, 7, 5, 3, 0, 1, 2, 1, 3, 4, 6, 8, 1, 3];
    let spaceship2: [usize; 18] = [6, 2, 1, 1, 8, 0, 5, 5, 0, 1, 8, 9, 6, 9, 4, 8, 0, 0];
    let spaceship3: [usize; 12] = [0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1];
    let spaceship4: [usize; 11] = [0, 1, 0, 2, 1, 0, 3, 1, 0, 1, 2];

    assert_eq!(material(&spaceship1), 83);
    assert_eq!(material(&spaceship2), 50);
    assert_eq!(material(&spaceship3), 6);
    assert_eq!(material(&spaceship4), 8);

    println!("{}", material(&spaceship1));
    println!("{}", material(&spaceship2));
    println!("{}", material(&spaceship3));
    println!("{}", material(&spaceship4));
}

fn material(spaceship: &[usize]) -> usize {
    let mut material: usize = 0;
    let mut current_pos: usize = 0;
    let spaceship_size: usize = spaceship.len();

    //get first non 0 element
    while spaceship[current_pos] == 0 {
        current_pos += 1;
    }

    //height of the the valley
    let mut valley_height: usize = spaceship[current_pos];

    //material that can be placed in the valley so far
    let mut current_material: usize = 0;

    current_pos += 1;
    while current_pos < spaceship_size {


        //check if current block is lower than valley_height
        //if yes add appropriate number of blocks to current_material
        //if not start new valley from current position
        let current_height: usize = spaceship[current_pos];
        if current_height < valley_height {
            current_material += valley_height - current_height
        } else {
            //close current valley and start new one
            material += current_material;
            current_material = 0;
            valley_height = current_height;

            //if new valley_height is bigger or same than before we need to check if valley will close before counting material
            let mut closure_curr_position: usize = current_pos + 1;
            let mut biggest_so_far: usize = 0;
            'check: while closure_curr_position < spaceship_size {
                let closure_curr_height: usize = spaceship[closure_curr_position];
                if closure_curr_height >= valley_height {
                    break 'check; // valley will close
                } else if closure_curr_height > biggest_so_far {
                    biggest_so_far = closure_curr_height; //finding biggest block so far
                }
                closure_curr_position += 1;
            }
            if closure_curr_position >= spaceship_size { //valley didn't close
                valley_height = biggest_so_far; //assign valley height to highest value that will close
            }
        }
        current_pos += 1;
    }

    return material;
}
