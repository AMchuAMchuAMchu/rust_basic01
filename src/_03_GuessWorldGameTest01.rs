use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    let num = rand::thread_rng().gen_range(1, 100);
    loop {
        println!("阔你吃哇...请输入^_^");


        let mut guess = String::new();

        io::stdin().read_line(&mut guess).expect("客官您的输入不正确喔^_^~~~");

        println!("比较一波中...");

        // 输入特殊字符的时候会直接报错的说,所以的话为了程序的健壮性的话这里需要异常处理方式一下^_^
        let guess: u32 = match guess.trim().parse() {
            Ok(v) => v,
            Err(_) => continue,
        };

        println!("您的输入是 >> {}", guess);

        match guess.cmp(&num) {
            Ordering::Less => println!("少于..."),
            Ordering::Greater => println!("大于..."),
            Ordering::Equal => {
                println!("Win...游戏结束...");
                break;
            }
        }
    }
}



