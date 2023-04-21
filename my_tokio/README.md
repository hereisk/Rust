# Welcome to My Tokio
***

## Task
This is a simple implementaion of how Rust works with asynchrounos.

## Description
There are three major components of the program.

1# a Task.
    is a vector (ThreadPool) that holds JoinHandle()
2# a Spawner.
     A Spawn sends a Task struct wrapped in Atomic RC through the channel to the receiver.
3# an Executor.
    An Executor is recv on the other side. It takes the task and executes it inside a Context.


## Installation
To run the program please run the "cargo run" command from inside my_tokio folder.

## Usage
The program runs the following sample asynchronous code:

// fn main() {
//     let mut my_tokio = MyTokio::new();
//     my_tokio.spawn(async {
//         println!("Task 1");
//     });
//     my_tokio.spawn(async {
//         println!("Task 2");
//     });
//     my_tokio.spawn(async {
//         println!("Task 3");
//     });
//     println!("Run my tokio");
//     my_tokio.run();
// }

### The Core Team
The project is compelted by Konstantin Melkov

<span><i>Made at <a href='https://qwasar.io'>Qwasar SV -- Software Engineering School</a></i></span>
<span><img alt='Qwasar SV -- Software Engineering School's Logo' src='https://storage.googleapis.com/qwasar-public/qwasar-logo_50x50.png' width='20px'></span>