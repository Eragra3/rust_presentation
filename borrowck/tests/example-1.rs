#[cfg(test)]
mod tests {

    fn gimme_string() -> String {
        "Here you go".to_string()
    }

    #[derive(Debug)]
    struct Person {
        name: String
    }

    impl Person {
        pub fn posess(victim: Person) -> Person {
            Person{name: victim.name}
        }

        pub fn eat_and_extract_string(self) -> String {
            self.name
        }
    }

    #[test]
    fn ownership_simple() {
        
        //value, has ownershp
        let owner_of_2 = 2;

        //borrow, doesn't own the value
        let _ = &owner_of_2;
    }

    #[test]
    fn mutability_simple() {
        let i = 1;

        //i is immutable
        // i += 1;

        let mut n = 1;
        n = 2;
        n += 3;
    }

    #[test]
    fn mutability_borrows() {
        let mut n = 0;

        let add10 = |x: &mut usize| *x += 10;
        add10(&mut n);

        assert_eq!(10, n);
    }

    #[test]
    fn ownership_voyage() {
        
        let text = gimme_string();

        let first_person = Person{name: text};

        //can't do this, text is moved to first_person
        //let second_person = Person{name: text};

        //consume first person and turn into posessed without copying anything
        let posessed_person = Person::posess(first_person);

        //extract name again, without copying
        let _ = posessed_person.eat_and_extract_string();

        //none of those is existing now
        // println!("{:?}", text);
        // println!("{:?}", first_person);
        // println!("{:?}", posessed_person);
    }
}