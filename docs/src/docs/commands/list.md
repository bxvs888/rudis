---
title: List Commands
titleTemplate: Commands
description: Overview of Rudis list commands including LINDEX, LLEN, LPOP, LPUSH, LPUSHX, LRANGE, LSET, RPUSH, RPUSHX, and RPOP commands.
---

# List Commands

List commands allow you to store string elements in lists, supporting insertion and popping from both ends. Lists are ordered and can contain duplicate elements.

## Command List

- [LINDEX](./list/lindex.md) - Returns the element at the specified index position in the list
- [LLEN](./list/llen.md) - Returns the length of the list
- [LPOP](./list/lpop.md) - Removes and returns the first element of the list
- [LPUSH](./list/lpush.md) - Inserts one or more values at the head of the list
- [LPUSHX](./list/lpushx.md) - Inserts a value at the head of an existing list
- [LRANGE](./list/lrange.md) - Returns elements in the specified range of the list
- [LSET](./list/lset.md) - Sets the value of a list element by index
- [RPUSH](./list/rpush.md) - Inserts one or more values at the tail of the list
- [RPUSHX](./list/rpushx.md) - Inserts a value at the tail of an existing list
- [RPOP](./list/rpop.md) - Removes and returns the last element of the list

## Use Cases

List commands are ideal for message queues, timelines, recent browsing records, and other scenarios. The combination of LPUSH and RPOP (or RPUSH and LPOP) can implement queue functionality, while the combination of LPUSH and LPOP can implement stack functionality.

For detailed usage and parameters of each command, please refer to the individual command documentation linked above.