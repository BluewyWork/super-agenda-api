pub mod error;
pub mod prelude;
pub mod routes {
   pub mod rpc;
}
pub mod handlers {
   pub mod user {
      pub mod authentication;
   }
}

fn main() {
   println!("Hello, world!");
}
