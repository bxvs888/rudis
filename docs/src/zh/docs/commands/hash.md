---
title: 哈希命令
titleTemplate: 命令
description: Rudis 哈希命令概述，包括 HDEL、HEXISTS、HGET、HGETALL、HKEYS、HLEN、HMGET、HMSET、HSET、HSETNX、HSTRLEN 和 HVALS 命令。
---

# 哈希命令

哈希命令允许您将键值对存储为哈希表（也称为映射或字典）。每个哈希可以存储多达 2^32 - 1 个字段-值对。

## 命令列表

- [HDEL](./hash/hdel.md) - 删除哈希表中的一个或多个指定字段
- [HEXISTS](./hash/hexists.md) - 检查哈希表中指定字段是否存在
- [HGET](./hash/hget.md) - 获取存储在哈希表中指定字段的值
- [HGETALL](./hash/hgetall.md) - 获取在哈希表中所有的字段和值
- [HKEYS](./hash/hkeys.md) - 获取哈希表中所有的字段名
- [HLEN](./hash/hlen.md) - 获取哈希表中字段的数量
- [HMGET](./hash/hmget.md) - 获取所有给定字段的值
- [HMSET](./hash/hmset.md) - 同时将多个字段-值对设置到哈希表中
- [HSET](./hash/hset.md) - 将字段-值对设置到哈希表中
- [HSETNX](./hash/hsetnx.md) - 仅当字段不存在时，为哈希表中的字段赋值
- [HSTRLEN](./hash/hstrlen.md) - 返回哈希表中指定字段值的字符串长度
- [HVALS](./hash/hvals.md) - 返回哈希表中所有值

## 使用场景

哈希命令非常适合用于表示对象，如用户资料、商品信息等。您可以将对象的所有属性存储在一个哈希中，每个属性作为一个字段。

详细了解每个命令的用法和参数，请参阅上述链接的各个命令文档。