#!/bin/bash
ssrPackageName=shadowsocks-v1.14.3.x86_64-unknown-linux-musl.tar.xz
# ssr 下载地址
shadowsocks_rust_url="https://github.com/shadowsocks/shadowsocks-rust/releases/download/v1.14.3/$ssrPackageName"
# ssr 下载地址
downPath='/root/ssr'


if [ ! -d $downPath ]
        then
        echo "创建 $downPath"
        mkdir $downPath
        else
        echo "文件夹已存在" 
fi

if [ -e "$downPath/$ssrPackageName" ]
        then
        echo "$ssrPackageName 已存在，开始解压"
        tar -xf "$downPath/$ssrPackageName" -C $downPath
        echo -e "解压完成"
        else
        wget -c -P $downPath  "$shadowsocks_rust_url"
        echo -e "下载 $ssrPackageName 完成"
        tar -xf "$downPath/$ssrPackageName" -C $downPath
        echo -e "解压完成"
fi

nohup "$downPath/ssserver" -s "[::]:33330" -m "aes-256-gcm" -k "xiaoze123" > "$downPath/ssr.log" 2>&1 &