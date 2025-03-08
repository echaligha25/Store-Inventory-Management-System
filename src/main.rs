fn main(){
    let string1= String::from("Hi,");
    let string2= String::from("There!");

    let concatenate_string=concatenate_strings(&string1, &string2);

    println!("{}",concatenate_string)
}
fn concatenate_strings (e1: &str, e2: &str)->String{
    let mut result = String::new();
    result.push_str(e1);
    result.push_str(e2);
    result

}