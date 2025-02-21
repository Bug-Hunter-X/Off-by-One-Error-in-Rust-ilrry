fn main() {
    let mut v = vec![1, 2, 3];
    let len = v.len();
    for i in 0..len {
        v[i] = v[i] * 2; 
    }
    println!(" {:?}",v);
}