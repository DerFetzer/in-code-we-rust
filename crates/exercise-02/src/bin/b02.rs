use exercise_02::Cake;

pub fn main() {
    let mut cake = Cake::new();

    let flour = "Flour".to_string();
    let eggs = "Eggs".to_string();
    let sugar = "Sugar".to_string();

    cake = cake
        .add_ingredient(flour)
        .add_ingredient(eggs)
        .add_ingredient(sugar);

    println!("Ingredients in the cake: {flour} {eggs} {sugar}");
    println!("This will be a yummy cake: {cake:?}!");
}
