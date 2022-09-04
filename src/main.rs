use std::io;

fn main(){
    let mut degree = String::new();
    let mut label = String::new();
    
    println!("What is the temprature degree you wanna convert?");
    io::stdin().read_line(&mut degree).expect("Err reading the degree");
    
    println!("Label your entry, for Celsius type c, in Fahrenheit type f ");
    io::stdin().read_line(&mut label).expect("Err reading label");

    let degree:f32 = degree.trim().parse().expect("Err parsing degree");
    if label.starts_with('c')
    {
        println!("Celsius degree {},Equals {} Fahrenheit",degree,convert_degree_to_fahrenheit(degree));
    }

    if label.starts_with('f')
    {
        println!("Fahrenheit degree {},Equals {} Celsius",degree,convert_degree_to_celsius(degree));
    }

}

fn convert_degree_to_fahrenheit(c_degree:f32)->f32
{
    // f = c * 1.8 + 32.0
    c_degree * 1.8 + 32.0
}

fn convert_degree_to_celsius(f_degree:f32)->f32
{
    // c = f-32.0 / 1.8
    (f_degree - 32.0) / 1.8
}

