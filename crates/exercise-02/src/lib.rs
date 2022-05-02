// Todo: Make all bins compile
// cargo run --bin <bin-name>

pub struct Cake {
    ready: bool,
    ingredients: Vec<String>,
}

impl Cake {
    pub fn new() -> Self {
        Cake {
            ready: false,
            ingredients: vec![],
        }
    }

    pub fn is_ready(&self) -> bool {
        self.ready
    }

    pub fn add_ingredient(&mut self, ingredient: String) {
        if self.is_ready() {
            panic!("Cannot add ingredient to baked cake")
        }
        self.ingredients.push(ingredient)
    }

    pub fn get_ingredients(&self) -> &Vec<String> {
        &self.ingredients
    }

    pub fn bake(&mut self) {
        if self.is_ready() {
            panic!("Cannot bake cake twice")
        }
        self.ready = true
    }

    pub fn eat(self) {
        if !self.ready {
            panic!("Cannot eat cake that is not ready")
        }
    }
}
