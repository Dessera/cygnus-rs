# cygnus-rs

跨平台的JLU Drcom实现

## 使用

```shell
# 创建用户数据
cygnus user create -u <username> -p <password> -m <mac_addr> -f cygnus.usr
# 使用用户数据登录
cygnus auth -f cygnus.usr
```

> MAC地址以`:`分隔
