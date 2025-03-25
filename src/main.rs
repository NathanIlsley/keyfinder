use std::io::{self, Write};
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    println!(" ####   #    #  #####   #####   #####  #####  #    #   ####        ####    ####   #    #  #####");
    println!("#       #    #  #      #       #         #    ##   #  #           #       #    #  ##  ##  #    ");
    println!("#   ##  #    #  ###     ####    ####     #    # #  #  #   ##      #   ##  ######  # ## #  ###  ");
    println!("#    #  #    #  #           #       #    #    #  # #  #    #      #    #  #    #  #    #  #    ");
    println!(" ####    ####   #####  #####   #####   #####  #    #   ####        ####   #    #  #    #  #####\n");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    let mut attempts = 0;
    
    loop {
        attempts += 1;
        let guess: u32;
        guess = match input("Please input your guess: ").trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a number!");
                continue;
            }
        };
        
        print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
        
        match guess.cmp(&secret_number) {
            Ordering::Equal     => {
                print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
                println!(" ###    ###   ####   ####   #####   ###   #####  ##");
                println!("#   #  #   #  #   #  #   #  #      #   #    #    ##");
                println!("#      #   #  ####   ####   ###    #        #    ##               COMPLETED IN {} ATTEMPTS!", attempts);
                println!("#   #  #   #  #  #   #  #   #      #   #    #      ");
                println!(" ###    ###   #   #  #   #  #####   ###     #    ##\n");
                break;
            }
            Ordering::Greater   => {
                println!("#####   ###    ###      ####   #####   ###   ##");
                println!("  #    #   #  #   #     #   #    #    #      ##");
                println!("  #    #   #  #   #     ####     #    #  ##  ##                      ATTEMPTS: {}", attempts);
                println!("  #    #   #  #   #     #   #    #    #   #    ");
                println!("  #     ###    ###      ####   #####   ###   ##\n");
            }
            Ordering::Less      => {
                println!("#####   ###    ###       ####  #    #   ###   #      #      ##");
                println!("  #    #   #  #   #     #      ##  ##  #   #  #      #      ##");
                println!("  #    #   #  #   #      ###   # ## #  #####  #      #      ##       ATTEMPTS: {}", attempts);
                println!("  #    #   #  #   #         #  #    #  #   #  #      #        ");
                println!("  #     ###    ###      ####   #    #  #   #  #####  #####  ##\n");
            }
        }
    }
}

fn input(message: &str) -> String {
    print!("{}", message);
    io::stdout().flush().expect("Flush failed!");

    let mut inp: String = String::new();

    io::stdin()
        .read_line(&mut inp)
        .expect("Failed to read line!");

    return inp;
}