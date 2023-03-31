// 1. if statement
//     An if statement consists of a Boolean expression followed by one or more statements.

// 2. if...else statement
//     An if statement can be followed by an optional else statement, which executes when the Boolean expression is false.

// 3. else...if and nested ifstatement
//     You can use one if or else if statement inside another if or else if statement(s).

// 4. match statement
//     A match statement allows a variable to be tested against a list of values.
    
// In the example given below, state_code is matched with a list of values MH, KL, KA, GA − if any match is found, a string value is returned to variable state. If no match is found, the default case _ matches and value Unkown is returned.

fn main(){
   let state_code = "MH";
   let state = match state_code {
      "MH" => {println!("Found match for MH"); "Maharashtra"},
      "KL" => "Kerala",
      "KA" => "Karnadaka",
      "GA" => "Goa",
      _ => "Unknown"
   };
   println!("State name is {}",state);
}

// Output
// Found match for MH
// State name is Maharashtra
