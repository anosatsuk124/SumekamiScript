# SumekamiScript

SumekamiScript is an unsafe and low-level scripting language that works in any environment (even in a bare-metal environment). `unsafe` means that the language's runtime does not promise any safety with the program operations.

## Features

- Simple (but strong) typing system (but not statically).
- Any operations are permitted (unsafe).
- Runs on any environment.
- Simple syntax and semantics (with S-expression based).
- Minimal binary size.
- Dynamically Runtime-Based.

## Milestone

- [ ] Defines the syntax.
- [ ] Implements the parser.
- [ ] Implements the runtime (probably it's an interpreter).
  - [ ] Basic runtime features.
  - [ ] Inline Assembly and Inline Binary features.
  - [ ] Some features for FFI (Foreign Function Interface).
- [ ] Implements some features for bare-metal environment's use. (e.g. A subcommand to generate a single file that includes the sources and the runtime executable. etc.)

## Naming

`SumekamiScript`'s correct spelling is that the first letters of `sumekami` and `script` are capitalized, and no spaces require between them.

### `sumekami` comes from

`sumekami` is an Old-Japanese word that has below meanings.

> 1. imperial ancestral deity
> 2. (regional) deity
>    [ONCOJ Dictionary](https://oncoj.orinst.ox.ac.uk/cgi-bin/oncoj_dictionary.sh?search=L050218&part=n)

So if you use this language, you shell get mighty power like deities...

## LICENSE

```
   Copyright 2023- Satsuki Akiba (viterum)

   Licensed under the Apache License, Version 2.0 (the "License");
   you may not use this file except in compliance with the License.
   You may obtain a copy of the License at

       http://www.apache.org/licenses/LICENSE-2.0

   Unless required by applicable law or agreed to in writing, software
   distributed under the License is distributed on an "AS IS" BASIS,
   WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
   See the License for the specific language governing permissions and
   limitations under the License.
```
