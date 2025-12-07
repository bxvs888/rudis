---
title: Key Commands
titleTemplate: Commands
description: Overview of Rudis key commands including DEL, EXISTS, EXPIRE, EXPIREAT, KEYS, MOVE, PERSIST, PEXPIRE, PEXPIREAT, PTTL, RANDOMKEY, RENAME, RENAMENX, TTL, and TYPE commands.
---

# Key Commands

Key commands are used to manage keys in Redis. These commands allow you to create, delete, check, and modify keys and their attributes, such as expiration time.

## Command List

- [DEL](./key/del.md) - Deletes the specified key
- [EXISTS](./key/exists.md) - Checks if one or more given keys exist
- [EXPIRE](./key/expire.md) - Sets the expiration time for a given key (in seconds)
- [EXPIREAT](./key/expireat.md) - Sets the expiration timestamp for a given key (Unix timestamp in seconds)
- [KEYS](./key/keys.md) - Finds all keys matching the given pattern
- [MOVE](./key/move.md) - Moves the specified key from the current database to the specified database
- [PERSIST](./key/persist.md) - Removes the expiration time from a given key, making it persistent
- [PEXPIRE](./key/pexpire.md) - Sets the expiration time for a given key (in milliseconds)
- [PEXPIREAT](./key/pexpireat.md) - Sets the expiration timestamp for a given key (Unix timestamp in milliseconds)
- [PTTL](./key/pttl.md) - Similar to the TTL command, but returns the remaining survival time of a key in milliseconds
- [RANDOMKEY](./key/randomkey.md) - Randomly returns a key from the current database
- [RENAME](./key/rename.md) - Renames a key to a new key
- [RENAMENX](./key/renamenx.md) - Renames a key to a new key only if the new key does not exist
- [TTL](./key/ttl.md) - Returns the remaining survival time of a key (in seconds)
- [TYPE](./key/type.md) - Returns the type of value stored in a key

## Use Cases

Key commands are among the most fundamental and important commands in Redis. They are used to manage the lifecycle of keys, including creating, querying, updating, and deleting keys. EXPIRE and TTL related commands are particularly suitable for caching scenarios, where expired data can be automatically cleaned up.

For detailed usage and parameters of each command, please refer to the individual command documentation linked above.