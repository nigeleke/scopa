# scopa

[![BSD 3 Clause License](https://img.shields.io/github/license/nigeleke/scopa?style=plastic)](https://github.com/nigeleke/scopa/blob/master/LICENSE)
[![Language](https://img.shields.io/badge/language-Gleam-blue.svg?style=plastic)](https://gleam.run/)
[![Build](https://img.shields.io/github/actions/workflow/status/nigeleke/scopa/ci.yml?style=plastic)](https://github.com/nigeleke/scopa/actions/workflows/ci.yml)
[![Coverage](https://img.shields.io/codecov/c/github/nigeleke/scopa?style=plastic)](https://codecov.io/gh/nigeleke/scopa)
![Version](https://img.shields.io/github/v/tag/nigeleke/scopa?style=plastic)

  [Site](https://nigeleke.github.io/scopa) \| [GitHub](https://github.com/nigeleke/scopa) \| [API](https://nigeleke.github.io/scopa/doc/scopa/index.html) \| [Coverage Report](https://nigeleke.github.io/scopa/llvm-cov/html/index.html) \| [App](https://nigeleke.github.io/scopa/app/)

[Scopa](https://en.wikipedia.org/wiki/Scopa) is a popular Italian card game played by two to four players.

## Background

I first discovered Scopa while on holiday in Puglia in August 2024. As a beginner, I found the scoring surprisingly tricky to keep track of, especially across multiple rounds. To make things easier, I quickly put together a spreadsheet to record each player’s score. It worked well enough, but it was awkward to reset, cumbersome to navigate, and not particularly flexible when adding extra players.

This app grew out of that experience. Its goal is simple: make scoring a game of Scopa easier, quicker, and less error-prone, so you can focus on playing rather than doing scorekeeping.

## Tech

The original version was written in [Rust](https://rust-lang.org/) using [Dioxus](https://dioxuslabs.com/), partly as a comparison with earlier work using [Leptos](https://www.leptos.dev/).

I have since rewritten in [Gleam](https://gleam.run) using [Lustre](https://hexdocs.pm/lustre/lustre.html), which has been a great fit for the project.

Suggestions, improvements, and bug reports are always welcome via the project's issue tracker: [scopa/issues](https://github.com/nigeleke/scopa/issues).

## Development

### Mise

```bash
mise build
mise test
mise run
```

### Non-mise

```bash
cd dev
gleam run --module scripts/generate_version
```

```bash
cd core
gleam build
gleam test
gleam run -m lustre/dev start
```

- Open the browser to http://127.0.0.1:1234/

## Acknowledgements

- Original icons are no attribution `CC0` and `PD` svg files on [svgrepo.com](https://www.svgrepo.com/). Derived images can be found under `./design/images/`.
- Flags by [flagicons](https://flagicons.lipis.dev/); Attribution: `./design/attribution/flags_license`.


# scopa

[![Package Version](https://img.shields.io/hexpm/v/scopa)](https://hex.pm/packages/scopa)
[![Hex Docs](https://img.shields.io/badge/hex-docs-ffaff3)](https://hexdocs.pm/scopa/)

Further documentation can be found at <https://hexdocs.pm/scopa>.
