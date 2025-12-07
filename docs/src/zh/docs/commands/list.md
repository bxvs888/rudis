---
title: 列表命令
titleTemplate: 命令
description: Rudis 列表命令概述，包括 LINDEX、LLEN、LPOP、LPUSH、LPUSHX、LRANGE、LSET、RPUSH、RPUSHX 和 RPOP 命令。
---

# 列表命令

列表命令允许您将字符串元素存储在列表中，支持从两端插入和弹出元素。列表是有序的，可以包含重复元素。

## 命令列表

- [LINDEX](./list/lindex.md) - 返回列表中指定索引位置的元素
- [LLEN](./list/llen.md) - 返回列表的长度
- [LPOP](./list/lpop.md) - 移除并返回列表的第一个元素
- [LPUSH](./list/lpush.md) - 将一个或多个值插入到列表头部
- [LPUSHX](./list/lpushx.md) - 将值插入到已存在的列表头部
- [LRANGE](./list/lrange.md) - 返回列表中指定范围内的元素
- [LSET](./list/lset.md) - 通过索引设置列表元素的值
- [RPUSH](./list/rpush.md) - 将一个或多个值插入到列表尾部
- [RPUSHX](./list/rpushx.md) - 将值插入到已存在的列表尾部
- [RPOP](./list/rpop.md) - 移除并返回列表的最后一个元素

## 使用场景

列表命令非常适合用于消息队列、时间线、最近浏览记录等场景。LPUSH 和 RPOP（或 RPUSH 和 LPOP）组合可以实现队列功能，而 LPUSH 和 LPOP 组合可以实现栈功能。

详细了解每个命令的用法和参数，请参阅上述链接的各个命令文档。