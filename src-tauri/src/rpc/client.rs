use std::borrow::Borrow;

use aliyun_openapi_core_rust_sdk::RPClient;
use serde::de::DeserializeOwned;

use std::fmt::Debug;

pub struct ClientCore {
    rpc_client: RPClient,
}

impl ClientCore {
    pub fn new(access_key_id: String, access_key_secret: String) -> Self {
        ClientCore {
            rpc_client: RPClient::new(
                access_key_id,
                access_key_secret,
                String::from("https://ecs.aliyuncs.com"),
                String::from("2014-05-26"),
            ),
        }
    }

    pub fn request<'a, T, R>(
        &'a self,
        action: &'a str,
        params: T,
    ) -> Result<R, Box<dyn std::error::Error>>
    where
        T: IntoIterator,
        T::Item: Borrow<(&'a str, &'a str)>,
        R: DeserializeOwned,
    {
        let res = self.rpc_client.get(action).query(params).send()?;
        println!("原始数据{}", &res[0..4]);
        let a: R = serde_json::from_str(res.as_str())?;
        Ok(a)
    }
}

// pub fn get_client() -> RPClient {
//     RPClient::new(
//         String::from("LTAI5tBtNCk4QCbgdGM8ckaw"),
//         String::from("9hjsBdKzKU4JS7OaqrNJqr6LFRWrsw"),
//         String::from("https://ecs.aliyuncs.com"),
//         String::from("2014-05-26"),
//     )
// }
