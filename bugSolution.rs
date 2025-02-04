fn main() {
    let mut v = vec![1, 2, 3];
    { //Scope to ensure v remains valid
        let ptr = v.as_mut_ptr();
        unsafe {
            *ptr = 10;  
        }
        println!("First element within scope: {:?}", v[0]);
    } 
    // v is dropped here
    // Accessing v[0] after dropping v is dangerous 
    // Instead, work with a copy of the data if needed after v is dropped 
    println!("Vector is dropped!");
}