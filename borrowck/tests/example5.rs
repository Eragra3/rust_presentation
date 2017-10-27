#[cfg(test)]
mod tests {
    
    #[derive(Debug)]
    struct Car<'a> {
        brand: &'a str,
        model: &'a str
    }

    #[derive(Debug)]
    struct Warehouse<'a> {
        supported_brands: Vec<&'a str>,
        cars: Vec<&'a Car<'a>>
    }

    impl<'a> Car<'a> {
        pub fn new(brand: &'a str, model: &'a str) -> Car<'a> {
            Car { 
                brand: brand,
                model: model
            }
        }

        pub fn set_model(&mut self, model: &'a str) {
            self.model = model;
        }
    }

    impl<'a> Warehouse<'a> {
        pub fn new() -> Warehouse<'a> {
            Warehouse { 
                cars: Vec::new(),
                supported_brands: Vec::new()
            }
        }

        pub fn add_brand(&mut self, brand: &'a str) {
            self.supported_brands.push(brand);
        }

        pub fn add_car(&mut self, car: &'a Car) {
            self.cars.push(car);
        }
    }

    #[test]
    fn warehouse() {
        let opel = "Opel";
        let volkswagen = "Volkswagen";

        let kadett = Car::new(opel, "Kadett");

        let mut golf = Car::new(volkswagen, "Golf");
        let passat = Car::new(volkswagen, "Passat");

        let mut warehouse = Warehouse::new();
        warehouse.add_brand(opel);
        warehouse.add_brand(volkswagen);
        
        warehouse.add_car(&kadett);

        warehouse.add_car(&golf);
        warehouse.add_car(&passat);

        {
            let meriva = Car::new(opel, "Meriva");
            // can't do that
            // warehouse.add_car(&meriva);

            // can't do that either
            // golf.set_model("Golf 3");
        }
    }
}