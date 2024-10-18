# scopa

[![BSD 3 Clause License](https://img.shields.io/github/license/nigeleke/scopa?style=plastic)](https://github.com/nigeleke/scopa/blob/master/LICENSE)
[![Language](https://img.shields.io/badge/language-Rust-blue.svg?style=plastic)](https://www.rust-lang.org/)
[![Build](https://img.shields.io/github/actions/workflow/status/nigeleke/scopa/acceptance.yml?style=plastic)](https://github.com/nigeleke/scopa/actions/workflows/acceptance.yml)
[![Coverage](https://img.shields.io/codecov/c/github/nigeleke/scopa?style=plastic)](https://codecov.io/gh/nigeleke/scopa)
![Version](https://img.shields.io/github/v/tag/nigeleke/scopa?style=plastic)

  [Site](https://nigeleke.github.io/scopa) \| [GitHub](https://github.com/nigeleke/scopa) \| [API](https://nigeleke.github.io/scopa/api/scopa/index.html) \| [Coverage Report](https://nigeleke.github.io/scopa/coverage/index.html) \| [App](https://nigeleke.github.io/scopa/app/)

[Scopa](https://en.wikipedia.org/wiki/Scopa) is a popular Italian card game played by two to four players.

## Background

I initially discovered Scopa during a holiday in Puglia in August 2024. For a beginner especially it's scoring seemed somewhat complicated and, at the time, I setup a quick spreadsheet to track each round of scores for each player. It worked but was not easy to navigate or reset or add additional players.

I decided to write this program:

  - to have a simpler way to score the game and
  - to complete a development using the [Dioxus](https://dioxuslabs.com/) framework (by way of a comparison against earlier work developed using [Leptos](https://www.leptos.dev/)).

As of October 2024 the project is essentially complete but some further work will be done on the UI graphics and usability.

## Development

```bash
cargo test
```

## Run

```bash
dx serve
```

- Open the browser to http://localhost:8080/scopa/app/

## Acknowledgements

  - Scopa broom logo - [Clean icons created by Freepik - Flaticon](https://www.flaticon.com/free-icons/clean)
  - Playing card images - [Cardmeister](https://cardmeister.github.io/)

