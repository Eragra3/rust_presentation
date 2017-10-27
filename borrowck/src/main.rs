
fn gimme_string_with_suffix<T: ToString>(n: T) -> String {
    let mut s = "Here you go".to_string();
    s.push_str(&n.to_string());
    s
}

fn gimme_vector() -> Vec<String> {
    (1..10).map(|i| gimme_string_with_suffix(i)).collect()
}

fn main() {
    
    let x = gimme_vector();

    //variable explicitly cloned here. Even values inside it
    let y = x.clone();

    println!("{:?}", y);
    println!("{:?}", x);
}