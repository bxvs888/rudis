---
title: Sorted Set Commands
titleTemplate: Commands
description: Overview of Rudis sorted set commands including ZADD, ZCARD, ZCOUNT, ZRANK, ZREM, and ZSCORE commands.
---

# Sorted Set Commands

Sorted set commands are similar to sets, but each member is associated with a score for sorting. Members are unique, but scores can be duplicated. Sorted sets are sorted from smallest to largest score.

## Command List

- [ZADD](./sortedSet/zadd.md) - Adds one or more members to a sorted set, or updates the score of an existing member
- [ZCARD](./sortedSet/zcard.md) - Returns the number of elements in a sorted set
- [ZCOUNT](./sortedSet/zcount.md) - Counts the number of members in a sorted set within a specified score range
- [ZRANK](./sortedSet/zrank.md) - Returns the rank of a specified member in a sorted set (starting from 0)
- [ZREM](./sortedSet/zrem.md) - Removes one or more members from a sorted set
- [ZSCORE](./sortedSet/zscore.md) - Returns the score of a specified member in a sorted set

## Use Cases

Sorted set commands are ideal for leaderboards, timelines, weighted queues, and other scenarios. Sorting functionality can be easily implemented through scores, while specific member rankings and scores can be quickly queried.

For detailed usage and parameters of each command, please refer to the individual command documentation linked above.