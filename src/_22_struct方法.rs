/**
*Description==>TODO  
*BelongsProject==>rust_basic01  
*BelongsPackage==>  
*CreateTime==>2022-09-21 17:08:26  
*Version==>1.0  
*Author==>02雪乃赤瞳楪祈校条祭制作委员会 wyq_start  
*/
#[derive(Debug)]



struct Anime{
    animeName:i32,
    time:i32
}

impl Anime {
    fn sayHello(&self)->i32{
        return self.animeName+self.time;
    }
}

fn main(){

    let a01 = Anime{
        animeName:36,
        time:2022,
    };

    println!("01 {}",a01.sayHello());

    println!("02 {:#?}",a01);

}


