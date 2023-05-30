fn main() {
    let res = print_welcome(&"Player 1");
    println!("{}", res);
}

fn print_welcome(name: &str) -> &str{
    println!("Welcome to the Snake Game, {}!", &name);
    return "Welcome to the Snake Game!";
}
