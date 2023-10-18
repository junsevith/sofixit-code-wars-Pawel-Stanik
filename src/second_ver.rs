use std::cmp::min;

fn material(spaceship: &[usize]) -> usize {
    let spaceship_size: usize = spaceship.len();

    let mut forward = spaceship.to_vec();
    let mut backward = spaceship.to_vec();

    let mut pos = 0;
    let mut highest_so_far = 0;

    //generate spaceship shadowed from front
    //shadowed space is space that is preceded by block
    while pos < spaceship_size - 1{
        let pos_val = spaceship[pos];
        if pos_val > highest_so_far {
            highest_so_far = pos_val;
        } else {
            forward[pos] = highest_so_far;
        }
        pos += 1;
    }

    //generate spaceship shadowed from back
    //shadowed space is space that is followed by block
    highest_so_far = 0;
    while pos > 0 {
        let pos_val = spaceship[pos];
        if pos_val > highest_so_far {
            highest_so_far = pos_val;
        } else {
            backward[pos] = highest_so_far;
        }
        pos -= 1;
    }

    //check how much space can be filled
    let mut material = 0;
    while pos < spaceship_size {
        //space can be filled only if it is shadowed both from front and back
        let potential_material = min(forward[pos],backward[pos]);
        //subtract space covered by blocks
        material += potential_material - spaceship[pos];
        pos += 1
    }

    return material;
}