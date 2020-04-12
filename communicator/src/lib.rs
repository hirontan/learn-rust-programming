// mod network {
//     fn connect() {
//     }
// }
// 
// mod client {
//     fn connect() {
//     }
// }

pub mod client;

// mod network {
//     fn connect() {
//     }
// 
//     mod server {
//         fn connect() {
//         }
//     }
// }

pub mod network;

#[cfg(test)]
mod tests {
    use super::client;

    #[test]
    fn it_works() {
        // assert_eq!(2 + 2, 4);
        client::connect();
    }
}

