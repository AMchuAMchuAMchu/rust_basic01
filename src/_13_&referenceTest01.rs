/**
*Description==>TODO  
*BelongsProject==>rust_basic01  
*BelongsPackage==>  
*CreateTime==>2022-09-15 21:13:49  
*Version==>1.0  
*Author==>02雪乃赤瞳楪祈校条祭制作委员会 wyq_start  
*/

fn main(){
    let str01 = String::from("Hello,EGOIST...");

    let len = str_len(&str01);

    println!("{} - {}",str01,len);

}

fn str_len(s:&String)-> usize{
    return s.len();
}



