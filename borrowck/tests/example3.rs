#[cfg(test)]
mod tests {

    fn gimme_string() -> String {
        "Here you go".to_string()
    }

    fn just_the_tips(text: &String) {
        //cannot consume, it's borrowed, not owned!
        // let _ = text.into_bytes();

        //I can use the value though
        let _ = text.as_bytes();
    }

    fn om_nom_nom(text: String) {
        //can use functions using borrows as well
        let _ = text.as_bytes();        

        let _ = text.into_bytes();
    }

    #[test]
    fn lifetime_borrow() {
        //can be mutable as well
        let text = gimme_string();

        //not doing anything with the value
        just_the_tips(&text);
        just_the_tips(&text);
        just_the_tips(&text);
        just_the_tips(&text);
    }
    
    #[test]
    fn lifetime_consume() {
        let text = gimme_string();

        om_nom_nom(text);
        //the value has been consumed already
        //om_nom_nom(text);
    }
}