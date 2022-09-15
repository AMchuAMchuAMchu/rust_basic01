/**
*Description==>TODO  
*BelongsProject==>rust_basic01  
*BelongsPackage==>  
*CreateTime==>2022-09-15 21:24:09  
*Version==>1.0  
*Author==>02雪乃赤瞳楪祈校条祭制作委员会 wyq_start  
*/

fn main(){

    let mut s1 = String::from("999");

    let s12 = &s1;
    let s13 = &s1;
    let s14  =&mut s1;

    println!("{},{},{},{}",s1,s12,s13,s14);


}
