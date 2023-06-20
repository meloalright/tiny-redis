#[tokio::main]
async fn main() {
    println!("Hello, world!");
}

// fn main() {
//     let mut rt = tokio::runtime::Runtime::new().unwrap();
//     rt.block_on(async {
//         println!("hello");
//     })
// }