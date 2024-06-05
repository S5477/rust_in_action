//Переполнение буфера

fn main() {
   let fruit = vec!['_', '_', '_'];

   let buffer_overflow = fruit[4];

   assert_eq!(buffer_overflow, '_')
}
