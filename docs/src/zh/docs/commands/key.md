---
title: 键命令
titleTemplate: 命令
description: Rudis 键命令概述，包括 DEL、EXISTS、EXPIRE、EXPIREAT、KEYS、MOVE、PERSIST、PEXPIRE、PEXPIREAT、PTTL、RANDOMKEY、RENAME、RENAMENX、TTL 和 TYPE 命令。
---

# 键命令

键命令用于管理 Redis 中的键。这些命令允许您创建、删除、检查和修改键及其属性，如过期时间。

## 命令列表

- [DEL](./key/del.md) - 删除指定的键
- [EXISTS](./key/exists.md) - 检查给定的一个或多个键是否存在
- [EXPIRE](./key/expire.md) - 为给定的键设置过期时间（以秒为单位）
- [EXPIREAT](./key/expireat.md) - 为给定的键设置过期时间戳（以秒为单位的 Unix 时间戳）
- [KEYS](./key/keys.md) - 查找所有符合给定模式的键
- [MOVE](./key/move.md) - 将指定的键从当前数据库移动到指定编号的数据库
- [PERSIST](./key/persist.md) - 移除给定键的过期时间，使键成为持久化的键
- [PEXPIRE](./key/pexpire.md) - 为给定的键设置过期时间（以毫秒为单位）
- [PEXPIREAT](./key/pexpireat.md) - 为给定的键设置过期时间戳（以毫秒为单位的 Unix 时间戳）
- [PTTL](./key/pttl.md) - 类似于 TTL 命令，但以毫秒为单位返回键的剩余生存时间
- [RANDOMKEY](./key/randomkey.md) - 从当前数据库中随机返回一个键
- [RENAME](./key/rename.md) - 将键重命名为新键
- [RENAMENX](./key/renamenx.md) - 仅在新键不存在时，将键重命名为新键
- [TTL](./key/ttl.md) - 返回键的剩余生存时间（以秒为单位）
- [TYPE](./key/type.md) - 返回存储在键中的值的类型

## 使用场景

键命令是 Redis 中最基础和最重要的命令之一。它们用于管理键的生命周期，包括创建、查询、更新和删除键。EXPIRE 和 TTL 相关命令特别适用于缓存场景，可以自动清理过期数据。

详细了解每个命令的用法和参数，请参阅上述链接的各个命令文档。