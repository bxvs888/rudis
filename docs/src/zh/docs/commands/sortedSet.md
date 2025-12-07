---
title: 有序集合命令
titleTemplate: 命令
description: Rudis 有序集合命令概述，包括 ZADD、ZCARD、ZCOUNT、ZRANK、ZREM 和 ZSCORE 命令。
---

# 有序集合命令

有序集合命令类似于集合，但每个成员都关联一个分数（score），用于排序。成员是唯一的，但分数可以重复。有序集合按分数从小到大排序。

## 命令列表

- [ZADD](./sortedSet/zadd.md) - 向有序集合中添加一个或多个成员，或更新已存在成员的分数
- [ZCARD](./sortedSet/zcard.md) - 返回有序集合中元素的数量
- [ZCOUNT](./sortedSet/zcount.md) - 计算在有序集合中指定分数范围内的成员数量
- [ZRANK](./sortedSet/zrank.md) - 返回有序集合中指定成员的排名（从0开始）
- [ZREM](./sortedSet/zrem.md) - 移除有序集合中的一个或多个成员
- [ZSCORE](./sortedSet/zscore.md) - 返回有序集合中指定成员的分数

## 使用场景

有序集合命令非常适合用于排行榜、时间线、带权重的队列等场景。通过分数可以轻松实现排序功能，同时还能快速查询特定成员的排名和分数。

详细了解每个命令的用法和参数，请参阅上述链接的各个命令文档。