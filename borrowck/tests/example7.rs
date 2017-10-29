#[macro_use]
extern crate serde_json;
extern crate serde;

#[cfg(test)]
mod tests {

    // MACRO RULES !

    fn create_json() {
        let _ = json!({
                        "name": "John Doe",
                        "age": 43,
                        "phones": [
                        "+44 1234567",
                        "+44 2345678"
                        ]
                    });
    }

    #[test]
    fn macro_serde_json() {
        create_json();
    }

    #[test]
    fn macro_print() {
        println!("My number is {:?}", 1);
        //not enough arguments
        // println!("My number is {:?}");
        //unused argument
        // println!("My number is {:?}", 1, 2);
    }
}
