use std::convert::TryInto;

fn main() {
    let twenty = 20;

    let twenty_one: i32 = 21;
    let twenty_two = 22i32;

    let addition = twenty + twenty_one +twenty_two;

    let one_million: i64 = 1_000_000;

    let forty_twos = [
        42.0,
        42f32,
        42.0_f32,
    ];

    let xob = [
        0b11,
        0o36,
        0x12C,
    ];

    // b-2, o-8, x-16
    println!("{:b}, {:o}, {:x}", 20, 20, 20); // 10100, 24, 14

    let a: i32 = 10;
    let b: u32 = 11;

    if (a as u32) < b  {
        println!("10 < 11");
    }

    let a: i32 = 10;
    let b: u16 = 100;

    let b_ = b.try_into().unwrap();

    if a < b_ {
        println!("Ten is less then one hundread.");
    }

    let result: f32 = 0.1 + 0.1;
    let desired: f32 = 0.2;

    let absolute_difference = (desired - result).abs();

    println!("{} {}",absolute_difference, absolute_difference <= f32::EPSILON);

    let (mut x,mut y,mut z) = (0, 0, 0);

    'outer: for a in 0.. {
        for b in 0.. {
            if x + y + z > 1000 {
                println!(" break 'outer");
                break 'outer;
            }
            (x, y, z) = (x + 1, y + 1, z + 1);
        }
     }
}