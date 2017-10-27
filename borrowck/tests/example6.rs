#[cfg(test)]
mod tests {

    #[derive(Debug)]
    enum Name3 {
        Name1{ dupa: string },
        Name2,
    }

    fn foo<'a, 'b: 'a>(&Name{ ref dupa }: &'a Name) -> () {
        unimplemented!();
    }

    fn name<T>(predicate: impl Fn(T) -> bool){
        unimplemented!();
    }

    #[test]
    fn lifetime_bounds() {

    }
}