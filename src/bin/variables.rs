//This example intentionally produces compiler warnings
//to demonstrate unused assignments and variable shadowing.

fn main() {
    
    //unlike C, variable types do not seem to need to be explicitly declared.
    //variables declared with let are immutable by default
    
    let voltage = 3.3; //gets inferred as a f64 by default I'm reading.
    //uncommenting the line below would be illegal
    //voltage = 5.0;
    
    //this compiles but warns how the value assigned to current is never read and then reassigned.
    let mut current = 0.02;
    current = 0.03;
    
    //This is legal and called shadowing.
    //When you shadow a variable, the original variable is inaccessible by that name for
    //the remainder of the scope which the shadowing occurred in.
    let power = 9001;
    let power = voltage * current;
    
    //The compiler warns about unused variable power (the shadowed one).
    //The compiler recommends prefixing unused variables with a '_' if intentional.
    
    println!("Voltage: {} V", voltage);
    println!("Current: {} A", current);
    println!("Power: {} W", power);
    
}
