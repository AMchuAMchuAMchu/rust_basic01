/**
*Description==>TODO  
*BelongsProject==>rust_basic01  
*BelongsPackage==>  
*CreateTime==>2022-09-16 14:45:02  
*Version==>1.0  
*Author==>02雪乃赤瞳楪祈校条祭制作委员会 wyq_start  
*/

fn main() {

    let mut str01 = String::from("Hello Rust !!");

    let world_length = first_world(&str01);

    println!(" >> {}",world_length);
}

fn first_world(str01:&String)->usize {

    let bytes = str01.as_bytes();

    for (i,&item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    return str01.len();
}