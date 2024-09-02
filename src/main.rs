use std::{thread, thread::{JoinHandle}, io, sync::mpsc::sync_channel};

const MIN_SIZE: i32 = 0;

// понимаю что подход с паникой плохой, если вы против `expect`
// то дайте пожалуйста знать
fn main() {
    let mut input= String::new();

    // читаем из stdin
    io::stdin()
        .read_line(&mut input)
        .expect("failed to read line");


    // валидируем `input` из stdin и проводим к типу i32
    let number: usize = input.trim().parse().expect("input not an integer");
    if number as i32 <= MIN_SIZE {
        println!("number should be greater {}", MIN_SIZE);
        return;
    }

    //
    let (tx, rx) = sync_channel(number);

    // создаем вектора для результата квадратов числа
    // создаем вектор JoinHandle для того чтобы дождаться всех тредов
    let mut results: Vec<usize> = (1..=number).collect();
    let handlers: Vec<JoinHandle<()>> = (1..=results.len()).map(|i| {
        let tx = tx.clone();
        thread::spawn(move || {
            let multiply = i * i;
            tx.send((i-1, multiply)).unwrap();
        })
    }).collect();

    // закрываем tx
    drop(tx);

    // переменная для суммы квадратов
    let mut sum = 0;

    // консьюним сообщения, суммируем квадраты числе
    // и присваиваем квадрат числа к индексу вектора
    while let Ok((index, value)) = rx.recv() {
        sum += value;
        results[index] = value
    }

    // дожидаемся завершения наших потоков
    // я понимаю что факт того что мы получили сообщение на 42 строке
    // но это не значит что тред закончил свою работу
    for handler in handlers {
        handler.join().unwrap();
    }

    println!("results of sum: {}", sum)
}
