/**
*Description==>TODO  
*BelongsProject==>rust_basic01  
*BelongsPackage==>  
*CreateTime==>2022-09-22 08:14:22  
*Version==>1.0  
*Author==>02雪乃赤瞳楪祈校条祭制作委员会 wyq_start  
*/

enum IPADDR{
    v4,
    v6,
}

fn main(){

    let a4 = IPADDR::v4;

    let a6 = IPADDR::v6;

    route(a4);


    let id04 = IPADDRDetail::v4(127,0,0,0);

    let id06 = IPADDRDetail::v6(String::from("呵呵O(∩_∩)O~"));


}

enum IPADDRDetail {
    v4(u32,u32,u32,u32),
    v6(String),
}

fn route(ap:IPADDR){

}



