# randline

This tool picks one or more random lines from a file (or anything else you pass to it).

```console
$ randline < /usr/share/dict/words
Urania

$ randline 3 < /usr/share/dict/words
foolhardiness
rhinoscopic
wormhood
```




## How it works

This tool uses [reservoir sampling](https://en.wikipedia.org/wiki/Reservoir_sampling) to select the random lines; in particular Algorithm L.

I wrote it as a way to understand how reservoir sampling works, and to try using Rust generics.
Although the final tool only deals with strings, the underlying `reservoir_sample` can sample iterators of any type.





## Installation

You can download compiled binaries from the [GitHub releases](https://github.com/alexwlchan/randline/releases).

Alternatively, you can install from source.
You need Rust installed; I recommend using [Rustup].
Then clone this repository and compile the code:

```console
$ git clone "https://github.com/alexwlchan/emptydir.git"
$ cd emptydir
$ cargo install --path .
```

[Rustup]: https://rustup.rs/





## Usage

You need to pipe input to `randline`.
If you don't pass an argument, it will print a single random line.

```console
$ randline < /usr/share/dict/words
blithen
```

You can choose the number of random lines to print by passing a single argument `k`:

```console
$ randline 3 < /usr/share/dict/words
unprofessed
ragout
Tarpeia
```

You can also pipe the output of another command to it, for example if I wanted to find 5 random words starting with 'a':

```console
$ grep '^a' /usr/share/dict/words | randline 5
approachabl
autecological
alogical
ambrain
anticonstitutionally
```







## License

MIT.
