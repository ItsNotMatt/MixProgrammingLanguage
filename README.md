# MixProgrammingLanguage
Turing complete programming language written in Rust from scratch. No dependencies

Language features
Variables: int, string, bool. Vars can also be const. Enums exist but aren't usable currently
While loops with nesting
If and else statements 
Native functions: print(), input(), to_string(), to_int()
Custom functions

Code examples
```
import core;

let name = input("Give me your name");
print("Hello ", name, ", how are you?");

-------------------------------------------

import core;

let age_string = input("How old are you?"); 
let age = age_string.to_int();

if age >= 21 { 
  print("You are able to consume alcohol");
} else { 
  print("You are not able to consume alcohol"); 
}

-------------------------------------------

import core;

let count = 0;

while count < 10 { 
  count += 1; print("Count is: ", count); 
}

-------------------------------------------

import core;

let guess = input("Give me a num").to_int(); 
let rand = 72;

while guess != rand { 
  if guess > rand { 
    let response = "Too high"; 
  } 
  if guess < rand { 
    let response = "Too low"; 
  } 
  guess = input(response).to_int(); 
} 
print("You won!");

-------------------------------------------

import core;
import core::collections;

let x = [10, 20, 30];

let iter = 0;
let sum = 0;

while iter < x.len() {
    sum += x[iter];
    iter += 1;
}

print("Sum is: ", sum);

-------------------------------------------

import core;

fn add_ten(x: int) : int {
    return x + 10;
}

let x = add_ten(0);
print("X is: ", x);
```
