fn main() {
    let mut v = vec![1, 2, 3];
    // Safe way to modify the first element
    v[0] = 10;
    println!("The first element is: {}", v[0]);

    //Another safe approach
    let mut v2 = vec![1,2,3];
    *v2.get_mut(0).unwrap() = 10;
    println!("The first element is: {}", v2[0]);
} 