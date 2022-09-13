
/**
*Description==>TODO  
*BelongsProject==>rust_basic01  
*BelongsPackage==>  
*CreateTime==>2022-09-13 20:07:17  
*Version==>1.0  
*Author==>02雪乃赤瞳楪祈校条祭制作委员会 wyq_start  
*/

fn main(){


    let str01 = m01();

    let str02 = String::from("Emilico");

    let str03 = m02(str02);

    println!("1.0 {}",str01);
    // println!("2.0 {}",str02);
    println!("3.0 {}",str03);



}

fn m01()-> String{
    let str01 = String::from("Hello,Rust");
    return str01;
}


fn m02(str01 : String)->String{
    return str01;
}
