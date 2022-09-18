/**
*Description==>TODO  
*BelongsProject==>rust_basic01  
*BelongsPackage==>  
*CreateTime==>2022-09-17 20:47:00  
*Version==>1.0  
*Author==>02雪乃赤瞳楪祈校条祭制作委员会 wyq_start  
*/

struct anime {
    animeName: String,
    character01: String,
    character02: String,
    time: i32,
}

fn main() {
    let shadow_house = anime {
        animeName: String::from("影宅"),
        character01: String::from("艾米丽可"),
        character02: String::from("凯特"),
        time: 2020,
    };

    // println!("{}", shadow_house.character02);

    let anime02 = anime02(String::from("莉可丽丝"), 2022);
    // println!("{}",anime02.character01);


    struct anime03(String, String);
    struct anime04(String, String);

    let _anime03 = anime03(String::from("锦木千束"), String::from("井上泷奈"));
    let _anime04 = anime03(String::from("凯特"), String::from("艾米丽可"));

    println!("{}", _anime03.0);
    println!("{}", _anime04.1);
}

fn anime02(animeName: String, time: i32) -> anime {
    return anime {
        animeName,
        character01: String::from("锦木千束"),
        character02: String::from("井上泷奈"),
        time,
    };
}




