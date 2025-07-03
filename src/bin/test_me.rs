fn main () {

let val1 = 5;
let val2 = 2;
let ans = val1 % val2;
println!("The result of {} % {} is: {}", val1, val2, ans);
print_vector();
concate_strings("world".to_string());
control_flow(5);
}


fn print_vector() {
let mut vector = vec![2,4,6,8,10];
  println!("{:?}", vector);
  vector.pop();
  println!("{:?}", vector);
  vector.push(12);
  print!("{:?}", vector);
}

fn concate_strings(s: String) -> String {
  let string1 = String::from("Hello");
  let result = string1 + " " +  &s;
  println!("{}", result);
  return result;
}

fn control_flow(n: u64) {
  if n > 50 {
    println!("the value is greater than 50");
  } else if n == 1 {
    println!("n is equal 1");
  } else if n < 25 {
    println!("n is less than 25");
} else if n > 25 && n < 50 {
   println!( "n value is greater than 25 but less than 50");
}
  }
