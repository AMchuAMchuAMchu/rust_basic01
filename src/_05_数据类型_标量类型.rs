fn main() {
    let tup01: (i32, u64, f64) = (02, 63, 81.9);

    println!("01 >> {}", tup01.0);

    let (x, y, z) = tup01;

    println!("02 >> {},{},{}", x, y, z);

    let arr01: [i128; 3] = [63194, 22194, 81194];

    println!("03 >> {}", arr01[2]);
}







