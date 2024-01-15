fn variables_data (){
    // CReation
    let a = 5;
    println!("variabel a is {a}");
// Mutability
    let mut b = 10;
    b =20;
    println!( "variable b is {b}");
// Shadowing 
    let c = 120;
    let c =30;
    println!( "variable c is {c}");
// Scope
    let d =70;
    {
        let d =60;
    println!( "variable inner d is {d}");
        
    }
    println!( "variable d is {d}");




}
fn main() {
    variables_data()

}

