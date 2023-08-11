use std::{*, io::Write};
use rand::prelude::*;

fn main() {
  loop {
    let guesses = game_loop();
    println!("The game took you {guesses} guesses.");

    let mut retry = String::new();
    println!("Do you want to play again? y/n");
    immediate_print("> ");
    io::stdin().read_line(&mut retry).unwrap();

    if retry.trim() == "n".to_string() {
      break;
    }
  }
}

/**
 * Returns the amount of guesses.
 */
fn game_loop() -> i32 {
  let mut lower_limit: i32;
  let mut upper_limit: i32;
  loop {
    (lower_limit, upper_limit) = input_loop();

    if lower_limit < upper_limit {
      break;
    }

    println!("The lower limit must be lower than the upper limit!");
  }

  let mut rng = thread_rng();
  let number = rng.gen_range(lower_limit .. upper_limit);

  println!("Attempt to guess the number.");
  
  let mut current_guess: i32;
  let mut guesses = 0;

  loop {
    let mut input = String::new();

    immediate_print("> ");
    io::stdin().read_line(&mut input).unwrap();
    println!("{}", input.trim());
    current_guess = input.trim().parse().unwrap();

    if current_guess > number {
      println!("Your guess is higher than the number.");
    } else if current_guess < number {
      println!("Your guess is lower than the number.");
    } else {
      break;
    }

    guesses += 1;
  }

  println!("Good job! The number was {number}.");
  guesses
}

fn input_loop() -> (i32, i32) {
  let mut lower_limit_string = String::new();
  let mut upper_limit_string = String::new();

  immediate_print("Lower limit: ");
  io::stdin().read_line(&mut lower_limit_string).unwrap();

  immediate_print("Upper limit: ");
  io::stdin().read_line(&mut upper_limit_string).unwrap();

  let lower_limit: i32 = lower_limit_string.trim().parse().unwrap();
  let upper_limit: i32 = upper_limit_string.trim().parse().unwrap();

  (lower_limit, upper_limit)
}

fn immediate_print(arg: &str) {
  print!("{arg}");
  io::stdout().flush().unwrap();
}
