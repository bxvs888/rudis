---
title: Hash Commands
titleTemplate: Commands
description: Overview of Rudis hash commands including HDEL, HEXISTS, HGET, HGETALL, HKEYS, HLEN, HMGET, HMSET, HSET, HSETNX, HSTRLEN, and HVALS commands.
---

# Hash Commands

Hash commands allow you to store key-value pairs as hash tables (also known as maps or dictionaries). Each hash can store up to 2^32 - 1 field-value pairs.

## Command List

- [HDEL](./hash/hdel.md) - Deletes one or more specified fields from a hash table
- [HEXISTS](./hash/hexists.md) - Checks if a specified field exists in a hash table
- [HGET](./hash/hget.md) - Gets the value of a specified field stored in a hash table
- [HGETALL](./hash/hgetall.md) - Gets all fields and values in a hash table
- [HKEYS](./hash/hkeys.md) - Gets all field names in a hash table
- [HLEN](./hash/hlen.md) - Gets the number of fields in a hash table
- [HMGET](./hash/hmget.md) - Gets the values of all given fields
- [HMSET](./hash/hmset.md) - Sets multiple field-value pairs to a hash table simultaneously
- [HSET](./hash/hset.md) - Sets a field-value pair to a hash table
- [HSETNX](./hash/hsetnx.md) - Sets the value of a field in a hash table only if the field does not exist
- [HSTRLEN](./hash/hstrlen.md) - Returns the string length of the value of a specified field in a hash table
- [HVALS](./hash/hvals.md) - Returns all values in a hash table

## Use Cases

Hash commands are ideal for representing objects, such as user profiles, product information, etc. You can store all properties of an object in a single hash, with each property as a field.

For detailed usage and parameters of each command, please refer to the individual command documentation linked above.