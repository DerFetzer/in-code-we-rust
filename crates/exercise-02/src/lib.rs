// Todo: Make all bins compile
// cargo run --bin <bin-name>

use std::marker::PhantomData;

pub trait CakeState {}

#[derive(Debug)]
pub struct Raw {}
#[derive(Debug)]
pub struct Baked {}

impl CakeState for Raw {}
impl CakeState for Baked {}

#[derive(Debug)]
pub struct Cake<S: CakeState> {
    ingredients: Vec<String>,
    _phantom_data: PhantomData<S>,
}

impl<S: CakeState> Cake<S> {
    pub fn get_ingredients(&self) -> &Vec<String> {
        &self.ingredients
    }
}

impl Cake<Raw> {
    pub fn new() -> Self {
        Self {
            ingredients: vec![],
            _phantom_data: PhantomData,
        }
    }

    pub fn add_ingredient(mut self, ingredient: String) -> Self {
        self.ingredients.push(ingredient);
        self
    }

    pub fn bake(self) -> Option<Cake<Baked>> {
        if self.ingredients.is_empty() {
            None
        } else {
            Some(Cake {
                ingredients: self.ingredients,
                _phantom_data: PhantomData,
            })
        }
    }
}

impl Default for Cake<Raw> {
    fn default() -> Self {
        Self::new()
    }
}

impl Cake<Baked> {
    pub fn decorate(mut self, decoration: String) -> Self {
        self.ingredients.push(decoration);
        self
    }

    pub fn eat(self) {
        println!("Yummy!");
    }
}
