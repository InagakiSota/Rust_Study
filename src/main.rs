// ...existing code...

use std::{fmt::{Error, format}, result};

#[derive(Debug)]
struct MobileSuit {
    name: String,
    model_nubmer: String,
    height: f32,
    weight: f32,
}

fn error_handring(result: Result<i32, String>) -> Result<i32, String> {
    let Ok(code) = result else {
        let err = result.unwrap_err();
        println!("Error!:{}", err);
        return Err(err);
    };
    println!("{}", code);
    Ok(100)
}

struct Iter {
    current: usize,
    max: usize,
}

impl Iterator for Iter {
    type Item = usize;

    fn next(&mut self) -> Option<usize> {
        self.current += 1;
        if self.current -1 < self.max {
            Some(self.current -1)
        } else {
            None
        }
    }
}

struct Angel {
    name: String,
    number: i32,
}

impl Angel {
    fn get_name(&self) -> Result<String, String> {
        if self.name.is_empty() {
            return Err("Error!".to_string());
        }

        Ok(self.name.to_string())
    }

    fn get_num(&self) -> Result<i32, String> {
        if self.number < 0 || self.number >= 19 {
            return Err("Error!".to_string());
        }

        Ok(self.number)
    }
}

fn test() {
    let st = {
        let s1: String = String::from("Hello, world");
        let s2: &str = &s1;
        let s3: String = s2.to_string();
        s3
    };

    let arry = {
        let arr1: [i32; 5] = [1; 5];
        arr1
    };

    let freedom: MobileSuit = MobileSuit {
        name: String::from("フリーダム"),
        model_nubmer: String::from("ZGMF-X10A"),
        height: 18.0,
        weight: 75.0,
    };

    println!("{}", st);
    println!("{:?}", arry);

    println!("{:?}", freedom.model_nubmer);

    let result: Result<i32, String> = Ok(100);
    println!("code: {}", result.unwrap_or(-1));
    let result: Result<i32, String> = Err("Death".to_string());
    println!("code: {}", result.unwrap_or(-1));

    let result = error_handring(Ok(2));
    println!("{}", result.unwrap());

    let v1 = vec![5; 5];
    let data = v1.get(6);

    match data {
        Some(v) => println!("{}", v),
        None => println!("None!"),
    }

    //println!("{}", data.unwrap());
    for elm in v1.iter() {
        print!("{} ", elm);
    }
    println!("");

    fn print(s: Box<[u8]>)
    {
        println!("{:?}", s);
    }

    let byte_array = [b'h', b'o', b'l',b'l',b'o',b'w'];
    print(Box::new(byte_array));

    for i in 1..10 {
        match i {
            1 => println!("1"),
            2 => println!("2"),
            _ => println!("Other"),
        }
    }

    println!("");

    let it = Iter {
        current: 0,
        max : 100,
    };

    for num in it {
        println!("{}", num);
    }
}

struct Person {
    name: String, 
    age: i32,
}

impl Person {
    fn new(name: &str, age: i32) -> Person {
        Person { 
            name: String::from(name),
            age: age
        }
    }
}

fn error_test() -> Result<i32, String> {
    let mut cnt = 0;
    loop {
        println!("{}", cnt);
        cnt += 1;
        if cnt >= 10 {
            return Err("Error!".to_string());
        };
    };

    //Ok(cnt)
}

fn angel() {
    use std::io::{self, Write};

    print!("数字を1～18で入力してください: ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    if let Err(_) = io::stdin().read_line(&mut input) {
        println!("入力読み取りに失敗しました");
        return;
    }
    let input_trim: &str = input.trim();

    match input_trim.parse::<u32>() {
        Ok(n) => {
            let name = match n {
                1 => "アダム",
                2 => "リリス",
                3 => "サキエル",
                4 => "シャムシエル",
                5 => "ラミエル",
                6 => "ガギエル",
                7 => "イスラフェル",
                8 => "サンダルフォン",
                9 => "マトリエル",
                10 => "サハクィエル",
                11 => "イロウル",
                12 => "レリエル",
                13 => "バルディエル",
                14 => "ゼルエル",
                15 => "アラエル",
                16 => "アルミサエル",
                17 => "タブリス",
                18 => "リリン",
                _ => "",
            };
            if name.is_empty() {
                println!("{}：不明な値", input_trim);
            } else {
                println!("第{}使徒　{}", input_trim, name);
            }
        }
        Err(_) => {
            println!("{}：数値ではありません", input_trim);
        }
    }
}

enum Emotion {
    Anger,
    Happy,
}

trait Emotional {
    fn get_happy(&mut self) -> String;
    fn get_anger(&mut self) -> String;
    fn tell_state(&self) -> String;
}

struct HappyPerson {
    name: String, 
    state: Emotion,
}

impl Emotional for HappyPerson {
    fn get_anger(&mut self) -> String {
        unimplemented!()
    }

    fn get_happy(&mut self) -> String {
        format!("{} is always happy", self.name)
    }

    fn tell_state(&self) -> String {
        todo!()
    }
}

fn get_value(v: bool) -> Option<usize> {
    if v {
        Some(100)
    } else {
        None
    }
}

fn main() {
    //angel();

    test();

    let result = error_test();
    match result {
        Ok(v)  => println!("Success:{}", v),
        Err(e) => println!("Fail:{}", e),
    }

    let angl = Angel {
        name: String::from("Adam"),
        number: 1,
    };

    let angle_num = angl.get_num().unwrap();
    let angel_name = angl.get_name().unwrap();
    
    println!("第{0}使徒　{1}", angle_num, angel_name);

    println!("file:{}", line!());

    let mut p = HappyPerson {
        name: "shinji".to_string(),
        state: Emotion::Happy,
    };

    //println!("{}", p.get_anger());

    let mut test_result = 0;
    match get_value(true) {
        Some(test_result) => println!("Success: {}", test_result),
        None => println!("failure"),
    }

}
// ...existing code...