use std::io;

fn main() {
    let mut buffer = String::new(); // переменная-буфер, которая хранит предыдущую строку
    loop {
        let mut input = String::new(); // переменная которая хранит текущую строку
        io::stdin().read_line(&mut input).unwrap(); // считываем строку из стандартного ввода
        if input != buffer {
            print!("{}", input); //выводим полученную строку в терминал, в том случае если она не совпадает с предыдущей
        }
        buffer = input; // записываем полученную строку в буфер
    }
}
