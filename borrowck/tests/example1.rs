#[cfg(test)]
mod tests {

    struct FooMutable<'a> {
       pub name: &'a mut str 
    }

    fn gimme_string() -> String {
        "Here you go".to_string()
    }
        
    fn gimme_string_with_suffix<T: ToString>(n: T) -> String {
        let mut s = "Here you go".to_string();
        s.push_str(&n.to_string());
        s
    }

    fn gimme_vector() -> Vec<String> {
        (1..10).map(|i| gimme_string_with_suffix(i)).collect()
    }

    #[test]
    fn cloning_simple() {
        
        let x = gimme_string();

        //value explicitly cloned here
        let y = x.clone();

        println!("{:?}", y);
        println!("{:?}", x);
    }

    #[test]
    fn cloning_complex() {
        
        let x = gimme_vector();

        //value explicitly cloned here. Even values inside it
        let y = x.clone();

        println!("{:?}", y);
        println!("{:?}", x);
        //those are EXACTLY same vectors
        assert_eq!(x, y);
    }

    #[test]
    fn cloning_mutable() {
        
        let mut x = gimme_string();

        //value explicitly cloned here
        let mut y = x.clone();

        let _foo_mut_1 = FooMutable { name: &mut y };
        let _foo_mut_2 = FooMutable { name: &mut x };
        
        // this does not work, immutable borrow exists in _foo_mut1
        // let _foo_mut_3 = FooMutable { name: &mut y };
    }
    
    #[test]
    fn cloning_mutable_borrow() {
        let mut text = gimme_string();

        let x = &mut text;
        //borrows don't have clone, but '.' operator will try to deref '&mut String' 
        //and find clone on 'String'
        //y is of type String, not &mut String
        let mut y = x.clone();

        let _foo_mut_1 = FooMutable { name: &mut y };
        let _foo_mut_2 = FooMutable { name: x };
    }
}