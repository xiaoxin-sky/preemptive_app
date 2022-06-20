use std::{error::Error, thread::sleep, time::Duration};

use chrono::{Duration as chronoDuration, Utc};
use serde::{Deserialize, Serialize};

use crate::rpc::instance::{check_instance_run, create_instance};

use super::client::ClientCore;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SecurityGroupObj {
    pub SecurityGroupId: String,
    pub VpcId: String,
}
impl SecurityGroupObj {
    fn new(SecurityGroupId: String, VpcId: String) -> SecurityGroupObj {
        SecurityGroupObj {
            SecurityGroupId,
            VpcId,
        }
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct SecurityGroups {
    SecurityGroup: Vec<SecurityGroupObj>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct SecurityGroupResponse {
    TotalCount: i16,
    PageSize: i16,
    PageNumber: i16,
    SecurityGroups: SecurityGroups,
}

/// 获取当前安全组信息
pub fn get_security_groups(
    client: &ClientCore,
    region_id: &str,
) -> Result<SecurityGroupObj, Box<dyn Error>> {
    let response: SecurityGroupResponse = client.request(
        "DescribeSecurityGroups",
        &[
            ("RegionId", region_id),
            ("SecurityGroupName", "alispotCreatedSecurityGroup"),
        ],
    )?;
    match response.SecurityGroups.SecurityGroup.len().eq(&0) {
        true => create_VPC_Security_Group(client, region_id),
        false => Ok(response.SecurityGroups.SecurityGroup[0].clone()),
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CreateVpcResponse {
    VpcId: String,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
enum VpcStatus {
    Pending,
    Available,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Vpc {
    Status: VpcStatus,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Vpcs {
    Vpc: Vec<Vpc>,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DescribeVpcs {
    Vpcs: Vpcs,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CreateSecurityGroup {
    SecurityGroupId: String,
}

/// 创建 VPC 和 安全组
pub fn create_VPC_Security_Group(
    client: &ClientCore,
    region_id: &str,
) -> Result<SecurityGroupObj, Box<dyn Error>> {
    let vpc_params = [
        ("regionId", region_id),
        ("CidrBlock", "172.16.0.0/24"),
        ("VpcName", "alispotCreatedVpc"),
    ];

    let res: CreateVpcResponse = client.request("CreateVpc", vpc_params)?;
    let mut securityGroupObj = SecurityGroupObj::new(String::from("SecurityGroupId"), res.VpcId);
    println!("Vpc 创建结果=>{:?}", securityGroupObj);

    // 循环查询 vpc
    let mut times: i16 = 30;
    let sleepSec = 3;
    let query = [("RegionId", region_id), ("VpcId", &securityGroupObj.VpcId)];
    while times.ge(&0) {
        let res: DescribeVpcs = client.request("DescribeVpcs", &query)?;
        match res.Vpcs.Vpc[0].Status {
            VpcStatus::Pending => {
                times -= 1;
                sleep(Duration::new(sleepSec, 0));
            }
            VpcStatus::Available => {
                break;
            }
        }
    }
    if times.lt(&0) {
        // return Err(Box::new(std::fmt::Error));
        panic!("重试次数已用完，退出程序")
    }
    println!("Vpc已创建");
    let query = [
        ("RegionId", region_id),
        ("VpcId", &securityGroupObj.VpcId),
        ("SecurityGroupName", "alispotCreatedSecurityGroup"),
    ];
    let res: CreateSecurityGroup = client.request("CreateSecurityGroup", &query)?;
    securityGroupObj.SecurityGroupId = res.SecurityGroupId;

    Ok(securityGroupObj)
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CreateVSwitchResponse {
    VSwitchId: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AuthorizeSecurityGroupResponse {}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct VSwitch {
    VSwitchId: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct VSwitches {
    VSwitch: Vec<VSwitch>,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct VSwitchesResponse {
    TotalCount: i32,
    VSwitches: VSwitches,
}

/// 安全组开启端口并创建实例
pub fn open_security_port(
    client: &ClientCore,
    region_id: String,
    security_group_id: String,
    vpc_id: String,
    zone_id: String,
) -> Result<(String, String), Box<dyn Error>> {
    let port = String::from("33333");
    let portPre = port[0..port.len() - 1].to_string();
    let port_range = String::from(&portPre) + "0" + "/" + portPre.as_str() + "9";

    println!("port_range--{}", security_group_id.as_str());

    let base = [
        ("RegionId", region_id.as_str()),
        ("SecurityGroupId", security_group_id.as_str()),
        // ("IpProtocol", "tcp"),
        ("SourceCidrIp", "0.0.0.0/0"),
        ("IpProtocol", "icmp"),
        ("PortRange", "-1/-1"),
    ];

    let _rs: AuthorizeSecurityGroupResponse = client.request("AuthorizeSecurityGroup", &base)?;

    // println!("请求{}", rs);

    let base = [
        ("RegionId", region_id.as_str()),
        ("SecurityGroupId", security_group_id.as_str()),
        ("IpProtocol", "tcp"),
        ("SourceCidrIp", "0.0.0.0/0"),
        ("PortRange", "22/22"),
    ];
    let _rs: AuthorizeSecurityGroupResponse = client.request("AuthorizeSecurityGroup", base)?;

    let base = [
        ("RegionId", region_id.as_str()),
        ("SecurityGroupId", security_group_id.as_str()),
        ("IpProtocol", "tcp"),
        ("SourceCidrIp", "0.0.0.0/0"),
        ("PortRange", "80/80"),
    ];
    let _rs: AuthorizeSecurityGroupResponse = client.request("AuthorizeSecurityGroup", base)?;

    let base = [
        ("RegionId", region_id.as_str()),
        ("SecurityGroupId", security_group_id.as_str()),
        ("IpProtocol", "tcp"),
        ("SourceCidrIp", "0.0.0.0/0"),
        ("PortRange", "443/443"),
    ];
    let _rs: AuthorizeSecurityGroupResponse = client.request("AuthorizeSecurityGroup", base)?;

    let base = [
        ("RegionId", region_id.as_str()),
        ("SecurityGroupId", security_group_id.as_str()),
        ("IpProtocol", "tcp"),
        ("SourceCidrIp", "0.0.0.0/0"),
        ("PortRange", &port_range),
    ];

    let _rs: AuthorizeSecurityGroupResponse = client.request("AuthorizeSecurityGroup", base)?;

    println!("安全组开启端口");

    // 创建 VSwitch

    let result: VSwitchesResponse = client.request(
        "DescribeVSwitches",
        &[
            ("RegionId", region_id.as_str()),
            ("VpcId", &vpc_id),
            ("ZoneId", &zone_id),
        ],
    )?;
    let mut VSwitchId = String::from("");
    if result.TotalCount.eq(&0) {
        println!("创建rqid");
        // 没有内容，需要创建 vsSwitch
        let res: CreateVSwitchResponse = client.request(
            "CreateVSwitch",
            &[
                ("RegionId", region_id.as_str()),
                ("CidrBlock", "172.16.0.0/24"),
                ("VpcId", &vpc_id),
                ("ZoneId", &zone_id),
                ("VSwitchName", "alispotCreatedVSwitch"),
            ],
        )?;
        println!("{:#?}", res);
        VSwitchId = res.VSwitchId;
    } else {
        VSwitchId = result.VSwitches.VSwitch[0].VSwitchId.clone();
    }
    println!("VSwitchId 为：{}", VSwitchId);

    // 循环查询 VSwitch 启动状态
    // let mut times: i16 = 30;
    // let sleepSec = 3;
    // let query = [("RegionId", region_id), ("VpcId", &securityGroupObj.VpcId),("ZoneId",zone_id),("VSwitchId",VSwitchId)];
    // while times.ge(&0) {
    //     let res: DescribeVpcs = client.request("DescribeVSwitches", &query)?;
    //     match res.Vpcs.Vpc[0].Status {
    //         VpcStatus::Pending => {
    //             times -= 1;
    //             sleep(Duration::new(sleepSec, 0));
    //         }
    //         VpcStatus::Available => {
    //             break;
    //         }
    //     }
    // }
    // if times.lt(&0) {
    //     // return Err(Box::new(std::fmt::Error));
    //     panic!("重试次数已用完，退出程序")
    // }
    // println!("Vpc已创建");

    // let auto_release_time = Utc::now()
    //     .checked_add_signed(chronoDuration::hours(8))
    //     .unwrap().to_string();

    // 创建实例
    let res = create_instance(client, &zone_id, &security_group_id, &VSwitchId)?;

    let instance_id = res.InstanceIdSet[0].clone();

    let ip_address = check_instance_run(client, region_id.as_str(), &instance_id)?;

    Ok((instance_id, ip_address))
}
