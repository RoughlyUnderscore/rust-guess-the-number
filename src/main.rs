use std::cmp::Ordering;
use std::io::{self, Write};
use rand::prelude::*;

fn main() {
  println!("================");
  println!("Guess the Number");
  println!("================");
  println!("\n");

  loop {
    game_loop();
    let go_again = new_game_request_loop();

    if !go_again { break; }
    else { println!("\n"); }
  }

  println!("Thanks for playing Guess the Number.")
}

fn immediate_print(arg: &str) {
  print!("{arg}");
  io::stdout().flush().unwrap();
}

fn game_loop() {
  let mut lower_bound: i64;
  let mut upper_bound: i64;
  let number: i64;

  loop {
    let (lower_bound_string, upper_bound_string) = input_loop();
    if let Ok(num) = lower_bound_string.trim().parse::<i64>() {
      lower_bound = num;
    } else {
      println!("Did not recognize the first number as a valid 64-bit integer. Try again.");
      println!("\n");
      continue;
    }

    if let Ok(num) = upper_bound_string.trim().parse::<i64>() {
      upper_bound = num;
    } else {
      println!("Did not recognize the second number as a valid 64-bit integer. Try again.");
      println!("\n");
      continue;
    }

    if lower_bound >= upper_bound {
      println!("The lower bound must be strictly less than the upper bound. Try again.");
      println!("\n");
      continue;
    }

    println!("\n");

    number = thread_rng().gen_range(lower_bound..upper_bound);
    break;
  }

  println!("\n");

  let mut current_guess: i64;
  let mut known_lower_bound: i64 = lower_bound;
  let mut known_upper_bound: i64 = upper_bound;
  let mut guesses: u32 = 0;

  loop {
    println!("Input your current guess. Current known bounds: {known_lower_bound} < x < {known_upper_bound}");

    let mut input = String::new();
    immediate_print("> ");
    match io::stdin().read_line(&mut input) {
      Ok(_) => (),
      Err(_) => {
        println!("Could not read the string. Try again.");
        println!("\n");
        continue;
      }
    }

    if let Ok(num) = input.trim().parse::<i64>() {
      current_guess = num;
    } else {
      println!("Your guess was not a valid 64-bit integer. Try again.");
      println!("\n");
      continue;
    }

    guesses += 1;

    match &current_guess.cmp(&number) {
      Ordering::Less => {
        println!("Your guess is too low. Try again.");
        println!("\n");

        if known_lower_bound < current_guess { known_lower_bound = current_guess; }
      },

      Ordering::Greater => {
        println!("Your guess is too high. Try again.");
        println!("\n");

        if known_upper_bound > current_guess { known_upper_bound = current_guess; }
      },

      Ordering::Equal => {
        println!("Incredible! The number was, in fact, {number}.");
        println!("The game took you {guesses} guesses.");
        println!("\n");

        return;
      }
    }
  }
}

fn input_loop() -> (String, String) {
  let mut lower_bound_string = String::new();
  let mut upper_bound_string = String::new();

  loop {
    println!("Input the lower bound");
    immediate_print("> ");
    match io::stdin().read_line(&mut lower_bound_string) {
      Ok(_) => (),
      Err(_) => {
        println!("Could not read the string. Try again.");
        continue;
      }
    };

    println!("Input the upper bound");
    immediate_print("> ");
    match io::stdin().read_line(&mut upper_bound_string) {
      Ok(_) => (),
      Err(_) => {
        println!("Could not read the string. Try again.");
        continue;
      }
    };

    break;
  }

  (lower_bound_string, upper_bound_string)
}

fn new_game_request_loop() -> bool {
  loop {
    let mut input = String::new();
    immediate_print("Would you like to play again? y/n ");
    match io::stdin().read_line(&mut input) {
      Ok(_) => (),
      Err(_) => {
        println!("Could not read the string. Try again.");
        continue;
      }
    }

    match input.to_lowercase().as_bytes()[0] {
      b'y' => return true,
      b'n' => return false,
      _ => {
        println!("Did not recognize y/n as the first character. Try again.");
        continue;
      }
    }
  }
}
