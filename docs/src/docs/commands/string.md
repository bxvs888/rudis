---
title: String Commands
titleTemplate: Commands
description: Overview of Rudis string commands including APPEND, DECR, DECRBY, GET, GETRANGE, GETSET, INCR, INCRBY, INCRBYFLOAT, MGET, MSET, SET, and STRLEN commands.
---

# String Commands

String commands are the most basic data type commands, which can store strings, integers, or floating-point numbers. Strings can store up to 512 MB of data.

## Command List

<div class="command-cards">
  <a href="./string/append.md" class="command-card">
    <div class="card-title">APPEND</div>
    <div class="card-description">If the key already exists and is a string, appends the value to the end of the key</div>
  </a>
  <a href="./string/decr.md" class="command-card">
    <div class="card-title">DECR</div>
    <div class="card-description">Decrements the numeric value stored in the key by one</div>
  </a>
  <a href="./string/decrby.md" class="command-card">
    <div class="card-title">DECRBY</div>
    <div class="card-description">Decrements the numeric value stored in the key by the specified value</div>
  </a>
  <a href="./string/get.md" class="command-card">
    <div class="card-title">GET</div>
    <div class="card-description">Gets the value of the specified key</div>
  </a>
  <a href="./string/getrange.md" class="command-card">
    <div class="card-title">GETRANGE</div>
    <div class="card-description">Returns a substring of the string value in the key</div>
  </a>
  <a href="./string/getset.md" class="command-card">
    <div class="card-title">GETSET</div>
    <div class="card-description">Sets the value of the given key to a new value and returns the old value of the key</div>
  </a>
  <a href="./string/incr.md" class="command-card">
    <div class="card-title">INCR</div>
    <div class="card-description">Increments the numeric value stored in the key by one</div>
  </a>
  <a href="./string/incrby.md" class="command-card">
    <div class="card-title">INCRBY</div>
    <div class="card-description">Increments the numeric value stored in the key by the specified value</div>
  </a>
  <a href="./string/incrbyfloat.md" class="command-card">
    <div class="card-title">INCRBYFLOAT</div>
    <div class="card-description">Increments the numeric value stored in the key by the specified float value</div>
  </a>
  <a href="./string/mget.md" class="command-card">
    <div class="card-title">MGET</div>
    <div class="card-description">Gets the values of all given keys</div>
  </a>
  <a href="./string/mset.md" class="command-card">
    <div class="card-title">MSET</div>
    <div class="card-description">Sets one or more key-value pairs simultaneously</div>
  </a>
  <a href="./string/set.md" class="command-card">
    <div class="card-title">SET</div>
    <div class="card-description">Sets the value of the specified key</div>
  </a>
  <a href="./string/strlen.md" class="command-card">
    <div class="card-title">STRLEN</div>
    <div class="card-description">Returns the length of the string value stored in the key</div>
  </a>
</div>

## Use Cases

String commands are the most commonly used data type commands, suitable for various scenarios such as caching, counters, and distributed locks. Counter functionality can be easily implemented through INCR and DECR series commands.

For detailed usage and parameters of each command, please refer to the individual command documentation linked above.