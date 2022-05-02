use exercise_02::Cake;

pub fn main() {
    let cake = Cake::new();

    let flour = "Flour".to_string();
    let eggs = "Eggs".to_string();
    let sugar = "Sugar".to_string();

    let baked_cake = cake
        .add_ingredient(flour)
        .add_ingredient(eggs)
        .add_ingredient(sugar)
        .bake();

    let decorated_cake = baked_cake
        .decorate("Cream".to_string())
        .decorate("Sprinkles".to_string());

    decorated_cake.eat();

    println!("Wow, what a cake: {decorated_cake:?}!")
}
