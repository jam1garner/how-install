# how-install

A CLI for helping find how to install a given command.

### Credit

This would be nothing without the core functionality, none of which is mine.

Huge thanks to:

- https://tldr.sh for command descriptions
- https://dbrgn.github.io/tealdeer/ for tldr page terminal display
- https://command-not-found.com/ for command install information

### Install

```
cargo install how-install
```

### Example Usage

Let's say you want to know how to install `make` on your distro. You can use `how-install` to take care of this:

```
$ how-install make

TLDR
  Task runner for targets described in Makefile.
  Mostly used to control the compilation of an executable from source code.
  More information: <https://www.gnu.org/software/make/manual/make.html>.

  Call the first target specified in the Makefile (usually named "all"):

      make

  Call a specific target:

      make target

[...]

INSTALL
  sudo pacman -S make
```

### Command Arguments

```
how-install
A CLI for helping find how to install a given command

Credit to:
   - https://tldr.sh for descriptions
   - https://dbrgn.github.io/tealdeer/ for tldr console output
   - https://command-not-found.com/ for command install information

USAGE:
    how-install [OPTIONS] <CMD>

ARGS:
    <CMD>
            Command to lookup how to install

OPTIONS:
        --distro <DISTRO>
            OS to install for

            [possible values: debian, ubuntu, alpine, arch, kali, centos, fedora, raspbian, docker]

    -h, --help
            Print help information

    -i, --install
            Run install command

        --no-tldr
            Don't output TLDR info about the given command

    -y
            Automatically run install command without prompting
```
