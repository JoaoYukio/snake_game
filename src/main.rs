fn main() {
    let res = print_welcome(&"Player 1");
    println!("{}", res);
    create_variables();

    let sum = add(10, 20);
    println!("Sum: {}", sum);

    // Cria um vector de inteiros
    let mut v: Vec<i32> = Vec::new();
    v.push(10);
    v.push(12);
    v.push(14);

    println!("Vector: {:?}", v);
}

fn print_welcome(name: &str) -> &str {
    println!("Welcome to the Snake Game, {}!", &name);
    return "Welcome to the Snake Game!";
}

fn create_variables() {
    let tuple = (20, "Ola", 10);
    println!("Tuple: {:?}", tuple);

    let x = [1, 2, 3];

    println!("Array: {}", x[2]);

    let y = [2; 10];

    println!("Array: {:?}", y);
}

fn add(x: u32, y: u32) -> u32 {
    let sum = x + y;
    return sum;
}
