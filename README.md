# scopa

[![BSD 3 Clause License](https://img.shields.io/github/license/nigeleke/scopa?style=plastic)](https://github.com/nigeleke/scopa/blob/master/LICENSE)
[![Language](https://img.shields.io/badge/language-Rust-blue.svg?style=plastic)](https://www.rust-lang.org/)
[![Build](https://img.shields.io/github/actions/workflow/status/nigeleke/scopa/acceptance.yml?style=plastic)](https://github.com/nigeleke/scopa/actions/workflows/acceptance.yml)
[![Coverage](https://img.shields.io/codecov/c/github/nigeleke/scopa?style=plastic)](https://codecov.io/gh/nigeleke/scopa)
![Version](https://img.shields.io/github/v/tag/nigeleke/scopa?style=plastic)

  [Site](https://nigeleke.github.io/scopa) \| [GitHub](https://github.com/nigeleke/scopa) \| [API](https://nigeleke.github.io/scopa/api/index.html) \| [Coverage Report](https://nigeleke.github.io/scopa/coverage/index.html)

[Scopa](https://en.wikipedia.org/wiki/Scopa) is a popular Italian card game played by two to four players.

## Background

I first discovered this card game during a holiday in Puglia in August 2024. In order to complete the scoring I set up a spreadsheet to track each round for each player. It worked but was not easy to navigate or reset.

I decided to write this program:

    a) to have a simpler way to score the game and
    b) to complete a development using the [Dioxus](https://dioxuslabs.com/) framework (by way of a comparison against earlier work developed using [Leptos](https://www.leptos.dev/)).

The project is essentially complete but some further work will be done for usabiity.

## Development

```bash
cargo test
```

## Run

```bash
dx serve --hot-reload
```

- Open the browser to http://localhost:8080
