# scopa

[![BSD 3 Clause License](https://img.shields.io/github/license/nigeleke/scopa?style=plastic)](https://github.com/nigeleke/scopa/blob/master/LICENSE)
[![Language](https://img.shields.io/badge/language-Gleam-blue.svg?style=plastic)](https://gleam.run/)
[![Build](https://img.shields.io/github/actions/workflow/status/nigeleke/scopa/ci.yml?style=plastic)](https://github.com/nigeleke/scopa/actions/workflows/ci.yml)
[![Coverage](https://img.shields.io/codecov/c/github/nigeleke/scopa?style=plastic)](https://codecov.io/gh/nigeleke/scopa)
![Version](https://img.shields.io/github/v/tag/nigeleke/scopa?style=plastic)

  [Site](https://nigeleke.github.io/scopa) \| [GitHub](https://github.com/nigeleke/scopa) \| [API](https://nigeleke.github.io/scopa/doc/scopa/index.html) \| [Coverage Report](https://nigeleke.github.io/scopa/llvm-cov/html/index.html) \| [App](https://nigeleke.github.io/scopa/app/)

[Scopa](https://en.wikipedia.org/wiki/Scopa) is a popular Italian card game played by two to four players.

## Background

I initially discovered Scopa during a holiday in Puglia in August 2024. For a beginner especially, the scoring seemed somewhat complicated. At the time I setup a quick spreadsheet to track each round of scores for each player. It worked but was not easy to navigate or reset or add additional players.

I decided to write this program:

  - to have a simpler way to score the game and
  - to complete a development using the [Dioxus](https://dioxuslabs.com/) framework (by way of a comparison against earlier work developed using [Leptos](https://www.leptos.dev/)),
  - then later migrated to a development using [Gleam](https://gleam.run) and [Lustre](https://hexdocs.pm/lustre/lustre.html).

Comments for improvements and any other issues, can be raised on [scopa/issues](https://github.com/nigeleke/scopa/issues).

## Development

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
