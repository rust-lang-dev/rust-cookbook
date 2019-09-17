pub fn test() {
    let normal_noodle = Noodle::new().build();
    let multiple_eggs_noodle = Noodle::new().egg_count(2).build();
    let beef_noodle = Noodle::new().beef(true).build();
    let invalid_noodle = Noodle::new().turnip(true).sweet(true).build();
    if let Ok(multiple_eggs_noodle) = multiple_eggs_noodle {
        multiple_eggs_noodle.print();
    }
    if let Ok(normal_noodle) = normal_noodle {
        normal_noodle.print();
    }
    if let Ok(beef_noodle) = beef_noodle {
        beef_noodle.print();
    }
    if let Err(invalid_noodle) = invalid_noodle {
        println!("Falied to cook noodle, error: {}", invalid_noodle);
    }
}
struct Noodle {
    egg_count: u8,
    beef: bool,
    turnip: bool,
    sweet: bool,
}

impl Noodle {
    fn new() -> Self {
        Noodle {
            egg_count: 1,
            beef: false,
            turnip: false,
            sweet: false,
        }
    }

    fn egg_count(mut self, count: u8) -> Self {
        self.egg_count = count;
        self
    }
    fn beef(mut self, beef: bool) -> Self {
        self.beef = beef;
        self
    }
    fn turnip(mut self, turnip: bool) -> Self {
        self.turnip = turnip;
        self
    }
    fn sweet(mut self, sweet: bool) -> Self {
        self.sweet = sweet;
        self
    }
    fn build(self) -> Result<Noodle, String> {
        // add some checks here
        if self.sweet && self.turnip {
            Err("Cannot cook turnip&swet".to_string())
        } else {
            Ok(self)
        }
    }

    fn print(&self) {
        let egg = if self.egg_count <= 1 { "egg" } else { "eggs" };
        let print_bool = |x| if x { "" } else { "no " };
        println!(
            "This is your noodle: {} {}, {}beef, {}turnip, {}sweet",
            self.egg_count,
            egg,
            print_bool(self.beef),
            print_bool(self.turnip),
            print_bool(self.sweet)
        );
    }
}
