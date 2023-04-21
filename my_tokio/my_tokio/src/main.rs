mod task;
pub use task::Task;

mod executor;
pub use executor::MyTokio;

fn main() {
    let mut my_tokio = MyTokio::new();

    my_tokio.spawn(async {
        println!("Task 1");
    });

    my_tokio.spawn(async {
        println!("Task 2");
    });

    my_tokio.spawn(async {
        println!("Task 3");
    });

    println!("Run my tokio");
    my_tokio.run();
}