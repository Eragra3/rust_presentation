#[cfg(test)]
mod tests {

    struct Foo<'a> {
       pub name: &'a str 
    }

    struct FooMutable<'a> {
       pub name: &'a mut str 
    }

    fn gimme_string() -> String {
        "Here you go".to_string()
    }

    #[test]
    fn borrow_immutable() {
        //can be mutable as well
        let text = gimme_string();

        //try borrowing here mutably
        let _foo1 = Foo { name: &text };
        let _foo2 = Foo { name: &text };
    }
    
    #[test]
    fn borrow_mutable() {
        // try removing mut here
        let mut number = gimme_string();

        let _foo_mut_1 = FooMutable { name: &mut number };
        
        // this does not work, immutable borrow exists in _foo_mut1
        // let _foo_mut_2 = FooMutable { name: &mut number };
    }
}