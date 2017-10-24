# tkzip

`tkzip` is a command to take matching files from an zip archive.

## Why do you need?

Some file systems are ignoring case of file paths, and others are not.
Git remote repositories is able to contains both and the problems sometimes occurs between contributors.

If your repository contains any case sensitive files and you work on `non case sensitive file system`, it is possible that you may not fetch all files.

The `tkzip` command helps such a situation.

After you download an zip archive of the repository from Github, you can extract specific files from the archive by `tkzip`.

In other words, you can extract case-sensitive files into different directories on `non case sensitive file system`.

## Usage

```
$ tkzip {zip file} {path prefix} {dest dir}
```

## Installation

```
$ git clone git@github.com:thara/tkzip.git
$ cd tkzip
$ cargo install
```
