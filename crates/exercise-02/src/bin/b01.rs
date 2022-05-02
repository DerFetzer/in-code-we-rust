use exercise_02::Cake;

pub fn main() {
    let cake = Cake::new()
        .add_ingredient("Flour")
        .add_ingredient("Eggs")
        .add_ingredient("Sugar");

    println!("This will be a yummy cake: {cake:?}!");

    cake.eat();
}
