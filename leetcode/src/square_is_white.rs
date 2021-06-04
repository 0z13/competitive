
// lmao please find a better way to write code like this.
// probably a good idea to start propagating errors.

pub fn square_is_white(coordinates: String) -> bool {
    // - 96
    let mut vert = 'a';
    let mut hori = 1;
    let mut iter = coordinates.chars();
    vert = iter.next().unwrap();
    hori = iter.next().unwrap().to_string().parse::<i32>().unwrap();
    let zz = (vert as i32) - 96;
    (hori % 2 == 0  && zz % 2 != 0) || (hori % 2 != 0  && zz % 2 == 0) 
}