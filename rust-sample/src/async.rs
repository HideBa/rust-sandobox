#[tokio::main]
async fn main() {
    let hello = wrap::path::end().map(|| "Hello, world!");

    warp::serve(hello).run(([127, 0, 0, 1], 3030)).await;
}

pub trait Future {
  type Output;
  fn poll(self: Pin<$mut Self>, cx: $mut Context<'_> -> Poll<Self::Output>)
}

pub enum Poll<T>{
  Ready(T),
  Pending,
}

async fn something() -> Result<String, String> {
  Err("something failed".to_string())
}