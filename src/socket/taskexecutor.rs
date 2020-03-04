// use std::thread;
// use client::*;
// use server::*;
//
// fn receive_task(){
//     let new_thread = thread::spawn(move || {
//         server::socketserver();
//       println!("I am a new thread,task is receive.");
//   });
//   new_thread.join().unwrap();
// }
//
//
//
// fn send_task(){
//       // 创建一个线程
//       let new_thread = thread::spawn(move || {
//         client::socketclient();
//         println!("I am a new thread,task is send.");
//     });
//     // 等待新建线程执行完成
//     new_thread.join().unwrap();
// }
//
