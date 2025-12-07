---
title: String Commands
titleTemplate: Commands
description: Overview of Rudis string commands including APPEND, DECR, DECRBY, GET, GETRANGE, GETSET, INCR, INCRBY, INCRBYFLOAT, MGET, MSET, SET, and STRLEN commands.
---

# String Commands

String commands are the most basic data type commands, which can store strings, integers, or floating-point numbers. Strings can store up to 512 MB of data.

## Command List

- [APPEND](./string/append.md) - If the key already exists and is a string, appends the value to the end of the key
- [DECR](./string/decr.md) - Decrements the numeric value stored in the key by one
- [DECRBY](./string/decrby.md) - Decrements the numeric value stored in the key by the specified value
- [GET](./string/get.md) - Gets the value of the specified key
- [GETRANGE](./string/getrange.md) - Returns a substring of the string value in the key
- [GETSET](./string/getset.md) - Sets the value of the given key to a new value and returns the old value of the key
- [INCR](./string/incr.md) - Increments the numeric value stored in the key by one
- [INCRBY](./string/incrby.md) - Increments the numeric value stored in the key by the specified value
- [INCRBYFLOAT](./string/incrbyfloat.md) - Increments the numeric value stored in the key by the specified float value
- [MGET](./string/mget.md) - Gets the values of all given keys
- [MSET](./string/mset.md) - Sets one or more key-value pairs simultaneously
- [SET](./string/set.md) - Sets the value of the specified key
- [STRLEN](./string/strlen.md) - Returns the length of the string value stored in the key

## Use Cases

String commands are the most commonly used data type commands, suitable for various scenarios such as caching, counters, and distributed locks. Counter functionality can be easily implemented through INCR and DECR series commands.

For detailed usage and parameters of each command, please refer to the individual command documentation linked above.