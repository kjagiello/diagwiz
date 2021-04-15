# diagwiz -- diagrams as code

[![CI](https://github.com/kjagiello/diagwiz/actions/workflows/diagwiz-ci.yml/badge.svg?event=push)](https://github.com/kjagiello/diagwiz/actions/workflows/diagwiz-ci.yml) [![Security audit](https://github.com/kjagiello/diagwiz/actions/workflows/diagwiz-audit.yml/badge.svg)](https://github.com/kjagiello/diagwiz/actions/workflows/diagwiz-audit.yml)

**Warning**: This project is in early experimental stage. Functionality is
subject to change and YMMV. Feel free to open an issue if you have any
suggestions, Rust tips & tricks (my first Rust project) or stumbled upon any
bugs.

## Supported diagrams

- [Sequence diagrams](https://en.wikipedia.org/wiki/Sequence_diagram)

## Online playground

You can experiment building diagrams with diagwiz using the
[online playground](https://diagwiz.io/playground).

# Installation

As the package is currently in experimental stage, you can install it by
building it from the source or by downloading the pre-compiled binary from the
latest nightly release.

## Download a nightly binary

Visit the [nightly release page](https://github.com/kjagiello/diagwiz/releases/tag/nightly)
and download a binary compatible with your system.

## Install from source

```bash
cargo install --git https://github.com/kjagiello/diagwiz
```

# Usage example

```bash
$ cat <<EOF > example.diag
alias ali = "Alice"

ali->Bob: "Hello!"
Bob->Bob: "(Bob thinks)"
Bob-->ali: "Hello back!"
EOF

$ diagwiz < example.diag
┌───────┐        ┌─────┐
│ Alice │        │ Bob │
└───────┘        └─────┘
    │    Hello!     │
    │──────────────▶│
    │               │
    │               │─┐
    │               │ │ (Bob thinks)
    │               │◀┘
    │               │
    │  Hello back!  │
    │◀--------------│
    │               │
┌───────┐        ┌─────┐
│ Alice │        │ Bob │
└───────┘        └─────┘
```


# Known issues

- Characters with a column width other than 1 cause artifacts in the ASCII
  representation of a diagram
- For some inputs, the generated layout might change on every run. This is due
  to the [Cassowary algorithm implementation](https://github.com/dylanede/cassowary-rs)
  not being deterministic and finding multiple optimal solutions for the given
  constraints.
