# pic-image-api
基于rust编写的异步随机图片后端

### 使用方法
1. 从[release](https://github.com/wuliya336/pic-image-api/releases)页面下载对应版本的二进制文件
2. 运行二进制文件
3. 在二进制文件所在目录创建一个data目录，再创建子文件夹名，子文件夹里面放图片
4. 访问127.0.0.1:337200/api/[子文件夹名]


## 配置文件
手动在程序目录创建config文件夹
`server.toml`
```
name = "pic-image-api" // 站点名称
host = "127.0.0.1" // 站点host
port = 33720 // 站点端口
```
`api.toml`
```
[[api]]
name = "萝莉图" /// 图片名称
description = "萝莉图描述" /// 图片描述
folder_name = "loli" /// 图片存放的子文件夹名
```