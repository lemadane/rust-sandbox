pub fn run() {
    println!("On Primative Arrays\n");
    let a0: [i32; 5] = [65; 5];
    let a1: [i32; 3] = [1, 2, 3];
    let a2 = a1;
    println!("{:?}", (a0, a1, a2));

    println!("On Vectors\n");
    let v1 = vec![11, 12, 13];
    let v2 = &v1;
    let v3 = v2;

    println!("{:?}", (&v1, v2, v3));
}
