# MixProgrammingLanguage
Turing complete programming language written in Rust from scratch. No dependencies

Language features
Variables: int, string, bool. Vars can also be const
While loops(non nested)
If and else statements 
Native functions: print(), input(), to_string(), to_int()

Code examples

let name = input("Give me your name");
print("Hello ", name, ", how are you?");

-------------------------------------------

let age_string = input("How old are you?"); 
let age = age_string.to_int();

if age >= 21 { 
  print("You are able to consume alcohol");
 } 
else { 
  print("You are not able to consume alcohol"); 
}

-------------------------------------------

let count = 0;

while count < 10 { 
  count += 1; print("Count is: ", count); 
}

-------------------------------------------

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

