---
title: 集合命令
titleTemplate: 命令
description: Rudis 集合命令概述，包括 SADD、SCARD、SINTER、SISMEMBER、SMEMBERS、SPOP、SREM、SUNION、SUNIONSTORE 命令。
---

# 集合命令

集合命令允许您存储唯一的字符串元素。集合是无序的，不允许重复元素。支持多种集合操作，如交集、并集等。

## 命令列表

- [SADD](./set/sadd.md) - 向集合中添加一个或多个成员
- [SCARD](./set/scard.md) - 返回集合中元素的数量
- [SINTER](./set/sinter.md) - 返回给定所有集合的交集
- [SISMEMBER](./set/sismember.md) - 判断成员是否是集合的成员
- [SMEMBERS](./set/smembers.md) - 返回集合中的所有成员
- [SPOP](./set/spop.md) - 移除并返回集合中的一个随机元素
- [SREM](./set/srem.md) - 移除集合中的一个或多个成员
- [SUNION](./set/sunion.md) - 返回给定所有集合的并集
- [SUNIONSTORE](./set/sunionstore.md) - 将给定所有集合的并集存储在指定的集合中

## 使用场景

集合命令非常适合用于标签系统、好友关系、去重统计等场景。由于集合的唯一性特性，可以轻松实现去重功能，而集合运算则可以方便地处理关系查询。

详细了解每个命令的用法和参数，请参阅上述链接的各个命令文档。