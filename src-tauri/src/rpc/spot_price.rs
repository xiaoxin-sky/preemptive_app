use serde::{Deserialize, Serialize};

use crate::rpc::client;

use super::client::ClientCore;
#[derive(Debug, Serialize, Deserialize)]
pub struct SpotPrices {
    SpotPriceType: Vec<SpotPriceType>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SpotPriceType {
    pub SpotPrice: f32,
    pub OriginPrice: f32,
    pub ZoneId: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Response {
    RequestId: String,
    Currency: String,
    SpotPrices: SpotPrices,
}

/// 获取最小价格
pub fn get_low_price_spot(
    client: &ClientCore,
) -> Result<Option<SpotPriceType>, Box<dyn std::error::Error>> {
    let res: Response = client.request(
        "DescribeSpotPriceHistory",
        &[
            ("RegionId", "cn-hongkong"),
            ("NetworkType", "vpc"),
            ("InstanceType", "ecs.xn4.small"),
        ],
    )?;
    // let res: Response = serde_json::from_str(&response)?;

    let mut min_price = f32::MAX;
    let mut spot_price_obj: Option<SpotPriceType> = None;
    for item in res.SpotPrices.SpotPriceType.iter() {
        min_price = min_price.min(item.SpotPrice);

        if min_price.eq(&item.SpotPrice) {
            spot_price_obj = Some(item.clone())
        }
    }
    Ok(spot_price_obj)
}
