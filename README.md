# jiggle

[![CI](https://github.com/Piszmog/jiggle/actions/workflows/ci.yml/badge.svg)](https://github.com/Piszmog/jiggle/actions/workflows/ci.yml)
[![Latest Release](https://img.shields.io/github/v/release/Piszmog/jiggle)](https://img.shields.io/github/v/release/Piszmog/jiggle)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

jiggle is a cli tool that bubbles up changes from a root branch through child branches.

## Use Case

Take the following tree structure,

```text
main
└── branch-1
    └── branch-2
       └── branch-3
       ...
```

`main` is at the root and there is a branch, `branch-1`, that is a child of `main`. There is also another
branch, `branch-2` but it is coming off of `branch-1`.

To get changes from `main` to `branch-1`, and changes from `main` and `branch-1` to `branch-2` can be time-consuming to
manually push, pull, and merge changes.

`jiggle` automates the `push` and `pull` aspects of getting changes up the tree.

## Installation

Head over to [Releases](https://github.com/Piszmog/jiggle/releases) and download the artifact for your architecture.

## Usage

```shell
$ ./jiggle -d /path/to/repo/or/directory/of/repos -t "main>branch-1>branch-2>branch-3"
```

### Options

| Option            | Default | Required  | Description                                           |
|:------------------|:-------:|:---------:|:------------------------------------------------------|
| `--tree`, `-t`    |   N/A   | **True**  | The tree of branches the updates should flow through. |
| `--dir`, `-d`     |  `./`   | **False** | The directory containing the repository to update.    |
| `--help`, `-h`    |   N/A   | **False** | Shows help                                            |
| `--version`, `-V` |   N/A   | **False** | The version of `jiggle`                               |
