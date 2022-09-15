/**
*Description==>TODO  
*BelongsProject==>rust_basic01  
*BelongsPackage==>  
*CreateTime==>2022-09-15 21:29:15  
*Version==>1.0  
*Author==>02雪乃赤瞳楪祈校条祭制作委员会 wyq_start  
*/


fn main(){

    m01();
}

fn m01() -> &String{//悬空指针...这个方法执行完了之后但是引依然的话是指向这里的位空的置的说...
    let s1 = String::from("Rust..");
    return &s1
}
