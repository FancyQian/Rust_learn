fn main() {
    let mut v: Vec<u8> = Vec::new();
    let v1: Vec<u8> = vec!(1,2,3);
    v.push(1);
    v.push(2);
    println!("{}, {}", v1[0], v[1]);
}
