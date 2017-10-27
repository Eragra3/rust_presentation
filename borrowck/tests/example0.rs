#[cfg(test)]
mod tests {

    fn gimme_string() -> String {
        "Here you go".to_string()
    }
    
    fn gimme_usize() -> usize {
        69
    }

    #[test]
    fn moving_simple() {
        
        let x = gimme_string();

        //variable (ownership) moved
        let y = x;

        println!("{:?}", y);

        //variable x no longer owns the value, see compiler error
        // println!("{:?}", x);
    }
    
    #[test]
    fn copying() {
        
        let x = gimme_usize();

        //variable implicitly COPIED here, because usize implements 'Copy' trait
        let y = x;

        println!("{:?}", y);
        //move ?        
        //nope, there is 'copy' trait on str structure
        println!("{:?}", x);
    }
}