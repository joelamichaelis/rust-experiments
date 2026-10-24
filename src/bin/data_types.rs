fn main() {
    
    //signed integers
    let temperature: i8 = -20;
    let voltage_mv: i32 = 3300;
    
    //unsigned integers
    let adc_reading: u16 = 4095;
    
    //floats
    let voltage: f32 = 3.3;
    let current: f64 = 0.02;
    
    println!("Temperature: {temperature} C");
    println!("Voltage: {voltage_mv} mV");
    println!("ADC reading: {adc_reading}");
    println!("Voltage: {voltage} V");
    println!("Current: {current} A");
    
    //intentionally trying to multiple a f32 by a f64;
    //in C this would be legal and result in the f32 getting upgraded to a f64 during the multiplication
    //rust flags E0308: mismatched types
    //let power = voltage * current;
    
    //The first fix:
    let power = (voltage as f64) * current;
    
    //prints out Power: 0.06599999904632568 W
    println!("Power64: {power} W");
    
    //cleaner formatting to 0.066:
    println!("Power64 formatted: {power:.3} W");
    
    
    //the other fix: 64->32 float math
    //I initially tried reusing power here, making it mutable in its declaration
    //but then realized that power was inferred to be a f64 thus would not work
    //to hold the result of f32 multiplication, which makes sense.
    let power32 = voltage * (current as f32);
    println!("Power32: {power32:.3} W");
   
}
