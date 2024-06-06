Числовые типы

i8, i16, i32, i64 - целые
u8, u16, u32, u64 - целые без знака
f32, f64 - с плавуйщей точкой
isize, usize - разряд по процессору 

приведение типов a < (b as i32) (не стоит сужать тип)

Альтернатива as
```
let a: i32 = 10;
let b: u16 = 100;

let b_ = b.try_into.unwrap();

if a < b_ {
    println!("Ten is less then one hundread.");
}
```

> Числа с плавающей лучше не сравнивать на прямую (есть сложности в сложности представления компом таких чисел)
> Лучше через f32::EPSILON c = ( a - b).abs(); c < f32::EPSILON

циклы
```

for i in collect По факту владения

for i in &collect чтение 

for i in &mut collect чтение/запись

for _ in 0..1001 выполнения цикла по диапазону (тупо то сколько раз надо прокрутить) (_ можно заменить на i и использовать)

 wile a < b {continue;} 
```

```

 loop {
    if x < y {
        break;
    }
 }
 ```


 Прерывание вложеных циклов

```
 'outer for x in 0.. {
    for y in 0.. {
        if x+y+z > 1000 {
            break 'outer;
        }
    }
 }
 ```


Ветвление
```
 if a == 10 {
    ...
 } else if a == 20 {
    ....
 } else {
    ...
 }
 ```


Присваивание из ветвления
```
 let desc = if is_even(n) {
    "even"
 } else {
    "odd"
 }
 ```

Присваивание из match
```
 let desc = match is_even(n) {
    true => "a",
    false => "b",
 }
 ```

Присваивание break
```
 let n = loop {
    break 123;
 }
 ```

 Соответсвие образцу match
 ```
match item {
    0         => {},
    10 ..= 20 => {},
    40 | 80   => {},
    _         => {},

}
 ```