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

    current_pos += 1; // start counting from the next element

    let mut valley_start: usize = current_pos; //next element after block that starts valley, first element in the valley

    let mut biggest_so_far: usize = 0; //biggest element in the valley

    let mut current_material: usize = 0; //material that can be placed in the valley so far

    loop {
        //when at the end of spaceship check if no valley is open
        if current_pos >= spaceship_size {
            if valley_start >= spaceship_size { //valley was opened at the last block - successfully end program
                break;
            } else { //some valley wasn't closed, try to close it with highest valley_height that will close
                valley_height = biggest_so_far;
                current_pos = valley_start;
                current_material = 0;
            }
        }

        //check if current block is lower than valley_height
        let current_height: usize = spaceship[current_pos];
        if current_height < valley_height { //valley wasn't closed, count material

            current_material += valley_height - current_height; //add appropriate number of blocks to current_material

            //find the biggest block in the valley in case it will not close
            if current_height > biggest_so_far {
                biggest_so_far = current_height;
            }
        } else { //valley was successfully closed
            //add counted material to output and start new valley
            material += current_material;
            current_material = 0;
            valley_height = current_height;
            valley_start = current_pos + 1;
            biggest_so_far = 0;
        }
        current_pos += 1;
    }

    return material;
}
