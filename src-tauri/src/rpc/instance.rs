use std::{error::Error, thread::sleep, time::Duration};

use serde::{Deserialize, Serialize};

use crate::manage::config::{Config, ConfigKey};

use super::client::ClientCore;

#[derive(Debug, Serialize, Deserialize, Clone)]
enum DescribeInstanceStatus {
    Running,
    Pending,
    Starting,
    Stopping,
    Stopped,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PublicIpAddress {
    IpAddress: Vec<String>,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DescribeInstance {
    Status: DescribeInstanceStatus,
    PublicIpAddress: PublicIpAddress,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DescribeInstances {
    Instance: Vec<DescribeInstance>,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DescribeInstancesResponse {
    Instances: DescribeInstances,
}
/// 等待实例启动成功，并返回 ip
pub fn check_instance_run(
    client: &ClientCore,
    region_id: &str,
    instance_id: &str,
) -> Result<String, Box<dyn Error>> {
    // 循环查询 vpc
    let mut times: i16 = 30;
    let sleepSec = 3;
    let query = [
        ("RegionId", region_id),
        ("InstanceIds", &(String::from("[\"") + instance_id + "\"]")),
    ];

    let mut res: DescribeInstancesResponse = client.request("DescribeInstances", &query)?;
    println!("✅ 实例状态 {:?}", res);
    while times.gt(&0) {
        res = client.request("DescribeInstances", &query)?;
        println!("✅ 实例状态 {:?}", res);

        if res.Instances.Instance.len().eq(&0) {
            println!("❌没找到实例,再次查询");
            times -= 1;
            sleep(Duration::new(sleepSec, 0));
            continue;
        }
        match res.Instances.Instance[0].Status {
            DescribeInstanceStatus::Running => break,
            _ => {
                times -= 1;
                sleep(Duration::new(sleepSec, 0))
            }
        }
    }
    if times.lt(&0) {
        // return Err(Box::new(std::fmt::Error));
        panic!("实例启动状态，重试次数已用完，退出程序")
    }
    println!("✅ 实例已启动{:#?}", res);
    Ok(res.Instances.Instance[0].PublicIpAddress.IpAddress[0].clone())
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct InstanceIdSets {
    pub InstanceIdSet: Vec<String>,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RunInstancesResponse {
    InstanceIdSets: InstanceIdSets,
}

/// 创建实例
pub fn create_instance(
    client: &ClientCore,
    region:&str,
    zone_id: &str,
    security_group_id: &str,
    vswitch_id: &str,
    config: &Config,
) -> Result<InstanceIdSets, Box<dyn Error>> {
    println!("开始创建实例-{}--{}--{}",zone_id,security_group_id,vswitch_id);
    // 创建实例
    let res: RunInstancesResponse = client.request(
        "RunInstances",
        &[
            ("RegionId", region),
            ("ImageId", "centos_7_06_64_20G_alibase_20190711.vhd"),
            ("InstanceType", "ecs.n1.tiny"),
            ("InternetChargeType", "PayByTraffic"),
            ("SystemDisk.Size", "20"),
            ("SystemDisk.Category", "cloud_efficiency"),
            ("SpotStrategy", "SpotAsPriceGo"),
            ("ZoneId", zone_id),
            ("SecurityGroupId", security_group_id),
            ("VSwitchId", vswitch_id),
            ("InstanceName", "alispotCreatedInstance"),
            (
                "Password",
                config.get_config_by_key(ConfigKey::password).as_str(),
            ),
            ("InternetMaxBandwidthOut", "10"),
            (
                "AutoReleaseTime",
                config.get_config_by_key(ConfigKey::release_time).as_str(),
            ),
        ],
    )?;

    println!(
        "抢占式实例创建成功，id -> {}",
        res.InstanceIdSets.InstanceIdSet[0]
    );
    Ok(res.InstanceIdSets)
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RebootInstanceResponse {}
/// 重启实例
pub fn reboot_instance(client: &ClientCore, instance_id: String) -> Result<(), Box<dyn Error>> {
    println!("重启实例");
    let res: RebootInstanceResponse = client
        .request("RebootInstance", &[("InstanceId", instance_id.as_str())])
        .unwrap();
    Ok(())
}
