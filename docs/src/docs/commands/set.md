---
title: Set Commands
titleTemplate: Commands
description: Overview of Rudis set commands including SADD, SCARD, SINTER, SISMEMBER, SMEMBERS, SPOP, SREM, SUNION, and SUNIONSTORE commands.
---

# Set Commands

Set commands allow you to store unique string elements. Sets are unordered and do not allow duplicate elements. Multiple set operations such as intersection and union are supported.

## Command List

- [SADD](./set/sadd.md) - Adds one or more members to a set
- [SCARD](./set/scard.md) - Returns the number of elements in a set
- [SINTER](./set/sinter.md) - Returns the intersection of all given sets
- [SISMEMBER](./set/sismember.md) - Determines whether a member is a member of a set
- [SMEMBERS](./set/smembers.md) - Returns all members in a set
- [SPOP](./set/spop.md) - Removes and returns a random element from a set
- [SREM](./set/srem.md) - Removes one or more members from a set
- [SUNION](./set/sunion.md) - Returns the union of all given sets
- [SUNIONSTORE](./set/sunionstore.md) - Stores the union of all given sets in a specified set

## Use Cases

Set commands are ideal for tag systems, friend relationships, deduplication statistics, and other scenarios. Due to the uniqueness characteristic of sets, deduplication functions can be easily implemented, while set operations can conveniently handle relational queries.

For detailed usage and parameters of each command, please refer to the individual command documentation linked above.