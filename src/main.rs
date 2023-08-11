use rand::prelude::*;
use std::cmp::Ordering;
use std::{io::Write, *};

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
  let mut lower_limit: i128 = 0;
  let mut upper_limit: i128 = 0;
  loop {
    let (lower_limit_string, upper_limit_string) = input_loop();
    match (
      lower_limit_string.trim().parse::<i128>(),
      upper_limit_string.trim().parse::<i128>(),
    ) {
      (Ok(num1), Ok(num2)) => {
        lower_limit = num1;
        upper_limit = num2;
      }
      (Err(_), Err(_)) => (),
      (Ok(_), Err(_)) => (),
      (Err(_), Ok(_)) => (),
    };

    if lower_limit != 0 && upper_limit != 0 {
      if lower_limit < upper_limit {
        break;
      } else {
        println!("The lower limit must be lower than the upper limit!");
      }
    }
  }

  let mut rng = thread_rng();
  let number = rng.gen_range(lower_limit..upper_limit);

  println!("Attempt to guess the number.");

  let mut current_guess: i128;
  let mut guesses = 0;

  loop {
    let mut input = String::new();

    immediate_print("> ");
    io::stdin().read_line(&mut input).unwrap();
    println!("{}", input.trim());
    current_guess = input.trim().parse().unwrap();

    match current_guess.cmp(&number) {
      Ordering::Less => println!("Your guess is too low."),
      Ordering::Greater => println!("Your guess is too high"),
      Ordering::Equal => break,
    }

    guesses += 1;
  }

  println!("Good job! The number was {number}.");
  guesses
}

fn input_loop() -> (String, String) {
  let mut lower_limit_string = String::new();
  let mut upper_limit_string = String::new();

  immediate_print("Lower limit: ");
  io::stdin().read_line(&mut lower_limit_string).unwrap();

  immediate_print("Upper limit: ");
  io::stdin().read_line(&mut upper_limit_string).unwrap();

  (lower_limit_string, upper_limit_string)
}

fn immediate_print(arg: &str) {
  print!("{arg}");
  io::stdout().flush().unwrap();
}
