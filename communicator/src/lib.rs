#![allow(unused)]

#[cfg(test)]
mod tests {
    use super::client;
    use super::network;

    #[test]
    fn client_connection() {
        client::connect();
    }
    #[test]
    fn network_server_connection(){
         network::server::connect();
    }
}

pub mod client;

pub mod network;

mod outermost {
    pub fn middle_function() {
        inside::inner_function();
    }

    fn middle_secret_function() {}

     pub mod inside {
        pub fn inner_function() {
            secret_function();
        }

        fn secret_function() {}
    }
}

fn try_me() {
    outermost::middle_function();
    //outermost::middle_secret_function();
    outermost::inside::inner_function();
    //outermost::inside::secret_function();
}
