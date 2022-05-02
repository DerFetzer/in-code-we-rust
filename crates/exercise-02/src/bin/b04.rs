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

    println!("Wow, what a cake: {decorated_cake:?}!");

    let first_ingredient = decorated_cake.get_ingredients().first().unwrap();

    decorated_cake.eat();

    println!("The first ingredient was: {first_ingredient}");
}
