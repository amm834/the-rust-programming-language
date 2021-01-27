pub fn run(){
    print_it();
    get_name("Aung Myat Moe");
    let mut num:i32 = 3;
    increate_num(&mut num);
    println!("{:?}",num);
}

fn print_it(){
    println!("Hello!");
}

fn get_name(name:&str){
    println!("His name is {}",name);
}
fn increate_num(num:&mut i32){
    *num += 1
}