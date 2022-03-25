use std::io;
fn main() {
   
    let command  = "Please Enter A Number To  Convert From Degrees Fahrenheit To Degrees Celsius".to_uppercase();

    loop
    {

    println!("{}",command);
    
    let mut choice = String::new();
    
    io::stdin().read_line(&mut choice).expect("Not A Number");

    let mut num:f32 = choice.trim().parse::<f32>().expect("Not A Number");
    
    convert(&mut num);

    }

}

fn convert(m_choice:&mut f32)
{

    let mut x:f32 = *m_choice - 32.0;

    let y:f32 = 5.0/9.0;

    x = x * y;

    println!("Degrees C = {}",x);
    
}
