fn main() {
    let mut data = vec![1,2,3];

    let last_data = data[data.len() - 1];

    data.push(4);

    println!("{:?}", last_data);
}
