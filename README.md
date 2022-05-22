# scoop-find

Find scoop apps

## About

A rust implementation similar to [`scoop-search`](https://github.com/shilangyu/scoop-search). This is a toy repository. It is recommended to use `scoop-search`.

*Non-goal*: 100% replicate of the original `scoop search`. Please use `scoop-search` instead.

## Installation

Add [siku](https://github.com/amorphobia/siku) bucket and then install with scoop.

```PowerShell
scoop bucket add siku https://github.com/amorphobia/siku
scoop install scoop-find
```

## Usage

Directly run the app

```PowerShell
scoop-find <query>
```

Or hook `scoop` with [`scoop-hook`](https://github.com/amorphobia/scoop-hook) and run as a sub-command of `scoop`

```PowerShell
# Invoke-Expression (&scoop-hook --hook)
scoop find <query>
```

## License

[AGPL 3.0](LICENSE) or later.
