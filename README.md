# 阿里云抢占式服务器创建工具

> 用于创建国外抢占式服务器，作用很多：爬虫、翻墙，短时间算力提升，目前主要用于翻墙，google 查询资料，其他用法自行探索。

### 基础用法
1. 配置：mac 直接下载 release 包，安装配置好阿里云密钥即可
2. 开启翻墙：创建实例完成后，点开启按钮即可创建翻墙进程，翻墙子程序监听本地 127.0.0.1:1081 
3. 配置 chrome `Proxy SwitchyOmega` 浏览器插件. auto switch 中配置规则，规则地址：https://raw.githubusercontent.com/gfwlist/gfwlist/master/gfwlist.txt

### 阿里云配置教程
配置教程参见 https://github.com/rockswang/alispot/blob/master/README.md

### 感谢
本项目来自于 https://github.com/rockswang/alispot 项目, 使用 rust 开发，完成 mac 版本。
本项目的特性，目前不开启 bbr，创建实例百分百成功，日常浏览页面速度可观。