#[macro_use]
extern crate serde_json;
extern crate serde;


#[cfg(test)]
mod tests {
    
    trait CanQuack {
        fn quack(&self);
    }   
    
    impl CanQuack for i32 {
        fn quack(&self) {
            println!("Quack!");
        }
    }

    use std::ops;
    use std;

    trait CanBeDoubled where Self: ops::Add<Self> + std::marker::Sized + std::marker::Copy {
        fn double(self) -> <Self as std::ops::Add>::Output {
            self + self
        }
    }

    impl CanBeDoubled for usize { }
    impl CanBeDoubled for u32 { }
    impl CanBeDoubled for i32 { }
    impl CanBeDoubled for f32 { }

    #[test]
    fn trait_default_implementation() {
        assert_eq!(64, 32.double());
        assert_eq!(64.0, (32.0).double());
    }

}
