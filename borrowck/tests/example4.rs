#[cfg(test)]
mod tests {

    #[derive(Debug)]
    struct Person {
        name: String
    }

    impl Person {
        pub fn new(name: &str) -> Person {
            Person { name: name.to_string() }
        }

        pub fn get_name(&self) -> &str {
            &self.name
        }
    }

    #[test]
    fn lifetime_scope() {
        
        let last_served_user_name;
        {
            let me = Person::new("Daniel Bider");

            //everything's fine here
            let my_name_borrow = me.get_name();
            println!("{:?}", my_name_borrow);
            
            //I can't borrow here, 'last_served_user_name' would outlive 'me'
            //last_served_user_Name = me.get_name();
            
            //I could get a copy though, which makes sense
            last_served_user_name = me.get_name().to_string();
        }

        //I can't use regular string literal, it's type is '&str', I want 'String'
        assert_eq!(last_served_user_name, "Daniel Bider".to_string());
    }
}