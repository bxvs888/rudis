---
title: Generic Commands
titleTemplate: Commands
description: Overview of Rudis generic commands including AUTH, CLIENT, ECHO, PING, and SELECT commands.
---

# Generic Commands

Generic commands are a set of commands used to manage client connections and basic server interactions. These commands don't directly manipulate data, but provide connection management, server status checking, and other basic functionalities.

## Command List

- [AUTH](./generic/auth.md) - Used to authenticate the server connection password
- [CLIENT](./generic/client.md) - Used to get or set client connection related information
- [ECHO](./generic/echo.md) - Prints the given string, mainly used for testing connections
- [PING](./generic/ping.md) - Used to test if the connection to the server is normal
- [SELECT](./generic/select.md) - Switches to the specified database

## Use Cases

Generic commands are typically used immediately after a client connection is established, or when verifying connection status. For example, the PING command is often used for heartbeat detection to ensure the connection remains valid.

For detailed usage and parameters of each command, please refer to the individual command documentation linked above.