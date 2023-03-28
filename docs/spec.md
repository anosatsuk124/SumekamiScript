# The Specifications of SumekamiScript

# Syntax

See [grammar.pest](../src/grammar.pest). It's defined as PEG(Parsing Expression Grammar) rules using `pest` format.

# Semantics

## Overview

- All the SumekamiScript programs are repetitions of S-expressions.
- Each of the symbols is a lambda expression, a primitive type value, a pointer, or a reference. And also it can have some side-effects.
- The primitive types are `b8`, `b16`, `b32`, `b64`, `b128`, and `bsize`. They are defined only from memory ranges. And there are the pointer and the reference for each of the types.
- There are literals for numbers, strings, and characters. All the literals are syntax sugar.

## Symbols

### Immutability

## Typing System

## Literals Syntax Sugar

## Pointer and Reference
