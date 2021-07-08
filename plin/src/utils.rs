use std::fmt::Debug;

use reqwest::{
    header::{HeaderMap, ACCEPT, AUTHORIZATION},
    Client,
};

pub trait PrintPass {
    fn println_and_pass(self);
}

#[allow(unused_must_use)]
impl<T, E: Debug> PrintPass for Result<T, E> {
    #[inline]
    fn println_and_pass(self) {
        self.map_err(|e| println!("{:?}", e));
    }
}

#[inline]
pub fn new_client_and_header(cr_token: &str) -> (Client, HeaderMap) {
    let mut header = HeaderMap::new();
    header.insert(ACCEPT, "application/json".parse().unwrap());
    header.insert(
        AUTHORIZATION,
        format!("Bearer {}", cr_token).parse().unwrap(),
    );
    (Client::new(), header)
}
