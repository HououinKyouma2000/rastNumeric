fn main() {
    let guess: u32 = "42".parse().expect("Not a number!");//указываем какой тип ожидаем от метода , так как он сам не выкупает
    //u это положительные , а i и полож и отриц
    //u32 = 2^32-2
    let guess: isize = -1;//тут i^N где N зависит от биттовасти процессора
    let guess = -1i8; //8 бит положительных
    let guess: isize = -100_000;//можно писать _
    let guess = 0xff; //можно писать в 16 ричной по дефолту он дают тип i32 . ff указывает на 16 ричность
    let guess = 0o77; //это 8 ричная система. 77 указывает на 8 ричность
    let guess = 0b1111_0000; // двоичный формат числа
    let guess = b'A'; //байт

    let num: i32 = 4; //int
    let num: i64 = 4; //long

    let x = 2.0; // f64 по дефолту
    let y: f32 = 3.0; // f32

    // addition
    let sum = 5 + 10;
    // subtraction
    let difference = 95.5 - 4.3;
    // multiplication
    let product = 4 * 30;
    
    // division
    let quotient = 56.7 / 32.2;
    let quotient = 56 / 32; //1

    let truncated = 5 / 3; // 1
    let truncated = -5 / 3; // Results in -1
    let truncated = 5 / -3; //-1
    let truncated = -5 / -3; // 1

    // remainder
    let remainder = 43 % 5; //3
    let remainder = -43 % 5; //-3
    let remainder = 43 % -5; //3
    let remainder = -43 % -5; //-3
}
