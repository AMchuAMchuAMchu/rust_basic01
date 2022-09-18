use std::time::SystemTime;

#[derive(Debug)]
/**
*Description==>TODO  
*BelongsProject==>rust_basic01  
*BelongsPackage==>  
*CreateTime==>2022-09-18 11:22:19  
*Version==>1.0  
*Author==>02雪乃赤瞳楪祈校条祭制作委员会 wyq_start  
*/
struct Rectangle {
    width: i32,
    height: i32,
}

fn main() {
    let rect01 = Rectangle {
        width: 10,
        height: 20,
    };

    let res01 = rect_area(&rect01);

    println!("结果: {};矩形的信息:{:#?}", res01, rect01);
}

fn rect_area(rect: &Rectangle) -> i32 {
    return rect.height * rect.width;
}
