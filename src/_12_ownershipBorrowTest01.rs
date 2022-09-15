/**
*Description==>TODO  
*BelongsProject==>rust_basic01  
*BelongsPackage==>  
*CreateTime==>2022-09-13 20:13:35  
*Version==>1.0  
*Author==>02雪乃赤瞳楪祈校条祭制作委员会 wyq_start  
*/

fn main(){

    let str01 = String::from("EGOIST&Emilico");

    let (str02,str01_length) = m01(str01);

    println!("value:{},length:{}",str02,str01_length);

}

fn m01(str01 :String)->(String,usize){
    let str01_length = str01.len();
    return (str01,str01_length);
}
