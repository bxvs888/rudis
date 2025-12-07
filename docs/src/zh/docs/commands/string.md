---
title: 字符串命令
titleTemplate: 命令
description: Rudis 字符串命令概述，包括 APPEND、DECR、DECRBY、GET、GETRANGE、GETSET、INCR、INCRBY、INCRBYFLOAT、MGET、MSET、SET 和 STRLEN 命令。
---

# 字符串命令

字符串命令是最基本的数据类型命令，可以存储字符串、整数或浮点数。字符串最大可以存储 512 MB 的数据。

## 命令列表

- [APPEND](./string/append.md) - 如果键已经存在并且是一个字符串，追加值到键的末尾
- [DECR](./string/decr.md) - 将键中存储的数字值减一
- [DECRBY](./string/decrby.md) - 将键中存储的数字值减去指定的数值
- [GET](./string/get.md) - 获取指定键的值
- [GETRANGE](./string/getrange.md) - 返回键中字符串值的子字符
- [GETSET](./string/getset.md) - 将给定键的值设为新值，并返回键的旧值
- [INCR](./string/incr.md) - 将键中存储的数字值增一
- [INCRBY](./string/incrby.md) - 将键中存储的数字值加上指定的数值
- [INCRBYFLOAT](./string/incrbyfloat.md) - 将键中存储的数字值加上指定的浮点数
- [MGET](./string/mget.md) - 获取所有给定键的值
- [MSET](./string/mset.md) - 同时设置一个或多个键值对
- [SET](./string/set.md) - 设置指定键的值
- [STRLEN](./string/strlen.md) - 返回键所存储的字符串值的长度

## 使用场景

字符串命令是最常用的数据类型命令，适用于各种场景，如缓存、计数器、分布式锁等。通过 INCR 和 DECR 系列命令可以轻松实现计数器功能。

详细了解每个命令的用法和参数，请参阅上述链接的各个命令文档。