#[cfg(test)]
mod tests {
    
    struct Context<'s>(&'s str);

    struct Parser<'c, 's: 'c> {
        context: &'c Context<'s>,
    }

    impl<'c, 's> Parser<'c, 's> {
        pub fn parse(&self) -> Result<(), &'s str> {
            Err(self.context.0)
        }
    }

    fn parse_context(context: Context) -> Result<(), &str> {
        Parser { context: &context }.parse()
    }

    #[test]
    fn lifetime_bounds() {

        let parsed_value;

        {
            parsed_value = parse_context(Context("Some text"));
        }

        match parsed_value {
            Ok(_) => (),
            Err(msg) => println!("{:?}", msg),
        }
    }
}