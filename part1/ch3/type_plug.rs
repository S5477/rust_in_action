#![allow(unused_variables)] //Отключение предупреждений компилятора при обработке замыслов WTF!

type File = String;

fn open(f: &mut File) -> bool {
     true
}

fn close(f: &mut File) -> bool {
    true
}

#[allow(dead_code)] //подавление тревоги компилятора по незадействованой функции
fn  read(f: &mut File, save_to: &mut Vec<u8>) -> ! { // ! показывает компилятору  что эта функция вообще не возвращает значение
    unimplemented!() // это макрос, при обнаружении которого происходит сбой программы
}

fn main() {
    let mut f = File::from("f.txt");
    open(&mut f);

    //read(f1,  vec![]);

    close(&mut f);
}