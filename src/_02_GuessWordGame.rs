use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("阔你吃哇...请输入^_^");

    let num = rand::thread_rng().gen_range(1, 100);

    println!("神秘数字是 >> {}", num);

    let mut guess = String::new();

    io::stdin().read_line(&mut guess).expect("客官您的输入不正确喔^_^~~~");

    println!("比较一波中...");

    let guess: u32 = guess.trim().parse().expect("异常数据....");

    println!("您的输入是 >> {}", guess);

    match guess.cmp(&num) {
        Ordering::Less => println!("少于..."),
        Ordering::Greater => println!("大于..."),
        Ordering::Equal => println!("Win..."),
    }
}



