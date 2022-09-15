/**
*Description==>TODO  
*BelongsProject==>rust_basic01  
*BelongsPackage==>  
*CreateTime==>2022-09-15 21:19:10  
*Version==>1.0  
*Author==>02雪乃赤瞳楪祈校条祭制作委员会 wyq_start  
*/

fn main(){

    let mut s = String::from("bye..bye...");

    {
        let s1 = &mut s;
    }

    let s2 = &mut s;

    println!("{}",s2);



}
