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

    'count: loop {
        current_pos += 1;

        //check if current block is lower than valley_height
        //if yes add appropriate number of blocks to current_material
        //if not start new valley from current position
        if spaceship[current_pos] < valley_height {
            current_material += valley_height - spaceship[current_pos]

        } else {
            //close current valley and start new one
            material += current_material;
            current_material = 0;
            valley_height = spaceship[current_pos];

            //if new valley_height is bigger or same than before we need to check if valley will close before counting material
            let mut closure_check_pos = current_pos + 1;
            'check: loop {
                if closure_check_pos >= spaceship_size { //valley didn't close
                    if valley_height == 0 { //valley can't close, end program
                        break 'count;
                    } else {
                        valley_height -= 1; //check if lower valley_height will close
                    }
                    closure_check_pos = current_pos + 1;

                } else if spaceship[closure_check_pos] >= valley_height { //valley will close for current valley_height, start counting
                    break 'check;
                }
                closure_check_pos += 1;
            }
        }
    }

    return material;
}
