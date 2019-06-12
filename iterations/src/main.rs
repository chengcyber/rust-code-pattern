fn main() {
    // iter_demo();
    // iter_rev_demo();
    // enumerate_demo();
    // old_school_while();
    // index_of_demo();
}

fn iter_demo() {
    let v1 = vec![1, 2, 3];

    for val in v1.iter() {
        println!("iter_demo: {}", val);
    }
}

fn iter_rev_demo() {
    let v1 = vec![1, 2, 3];

    for val in v1.iter().rev() {
        println!("iter_rev_demo: {}", val);
    }
}

/**
 * when i want know index
 */
fn enumerate_demo() {
    let v = vec![0; 3];
    for (index, value) in v.iter().enumerate() {
        println!("enumerate_demo: {}, {}", index, value);
    }
}

/**
 * controlling i is intentional sometime
 * but, for(;;) is not support in rust
 * so, using while here to mimic this pattern
 */
fn old_school_while() {
    let v = vec![1, 2, 3];
    let mut i = 0;
    while i < v.len() {
        println!("old_school_while: {}", v[i]);
        // there is no i++ in rust as well :)
        i += 1;
    }
}

fn index_of_demo() {
    let v = vec![1, 2, 3, 4, 5, 6];
    /**
     * return a Option(index), add rev() if lastIndexOf
     */
    let index = v.iter().position(|&value| value == 3);
    println!("index is {:?}", index);
}
