// move_semantics2.rs
// Make me compile without changing line 13!
// Execute `rustlings hint move_semantics2` for hints :)

fn main() {
    let mut vec0: Vec<i32> = Vec::new();

    // fix 1:
    // let mut vec1 = fill_vec(vec0[..].to_vec());

    // fix 2:
    // let mut vec1 = fill_vec_2(&vec0);

    // fix 3:
    fill_vec_3(&mut vec0);

    // Do not change the following line!
    println!("{} has length {} content `{:?}`", "vec0", vec0.len(), vec0);

    // for fix 3
    let vec1 = &mut vec0;
    vec1.push(88);

    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);
}

fn fill_vec(vec: Vec<i32>) -> Vec<i32> {
    let mut vec = vec;

    vec.push(22);
    vec.push(44);
    vec.push(66);

    vec
}

fn fill_vec_2(vec: &Vec<i32>) -> Vec<i32> {
    let mut vec = vec[..].to_vec();

    vec.push(22);
    vec.push(44);
    vec.push(66);

    vec
}

fn fill_vec_3(vec: &mut Vec<i32>)  {
    vec.push(22);
    vec.push(44);
    vec.push(66);
}
