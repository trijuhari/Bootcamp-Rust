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

fn  datatype (){
    let  b1 = true; //Boolean
    println!( "=======================");
    println!( "variable  b1 is {b1}");
    // unsigned integers 
    println!( "====== unsigned integers ========");

    let nilai_u8: u8 = 125;
    let nilai_u16: u16 = 65535;
    let nilai_u32: u32 = 4294967295;
    let nilai_u64: u64 = 18446744073709551615;
    let nilai_u128: u128 = 340282366920938463463374607431768211455;
    
    println!("Nilai u8: {}", nilai_u8);
    println!("Nilai u16: {}", nilai_u16);
    println!("Nilai u32: {}", nilai_u32);
    println!("Nilai u64: {}", nilai_u64);
    println!("Nilai u128: {}", nilai_u128);
    // Signed Integers
    println!( "====== signed integers ========");

    let nilai_i8: i8 = -128;
    let nilai_i16: i16 = 32767;
    let nilai_i32: i32 = -2147483648;
    let nilai_i64: i64 = 9223372036854775807;
    let nilai_i128: i128 = -170_141_183_460_469_231_731_687_303_715_884_105_727;

    println!("Nilai i8: {}", nilai_i8);
    println!("Nilai i16: {}", nilai_i16);
    println!("Nilai i32: {}", nilai_i32);
    println!("Nilai i64: {}", nilai_i64);
    println!("Nilai i128: {}", nilai_i128);

    println!( "====== floating ========");

    let nilai_f32: f32 = -123.12654;
    let nilai_f64: f64 = 4567.10;

    println!("Nilai f32: {}", nilai_f32);
    println!("Nilai f64: {}", nilai_f64);

    println!( "====== usize ========");

    let nilai_usize1: usize = 1;
    let nilai_usize2: usize =50;

    println!("Nilai: {}", nilai_usize1);
    println!("Nilai: {}", nilai_usize2);

    let length_array: usize =10;
    let indeks: usize =2;
    let array = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let elemen = array[indeks];

    println!( "Length array {}", array.len());
    println!( " index {} elemen array {}",indeks, elemen);

    println!( "====== char,str, string ========");
    let  huruf = 'a';
    println!("Ini adalah karakter: {}", huruf);

    let str1: &str = "Hello World!";
    println!("{}",str1);

    let mut string: String = String::from("Halo, ");
    let slice = "Ini adalah string";
    string.push_str(slice);
    println!("{}",string);
    let mut man_string = string;
    println!("{}",man_string);
    let slice2 = "Bagus";
    man_string.push_str(slice2);
    println!("{}",man_string);





     



}


fn main() {
    variables_data();
    datatype();

}

