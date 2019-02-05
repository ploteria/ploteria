# Contributing to Ploteria

## Ideas, Experiences and Questions

The easiest way to contribute to Ploteria is to use it and report your experiences, ask
questions and contribute ideas. We'd love to hear your thoughts on how to make Ploteria
better, or your comments on why you are or are not currently using it.

Issues, ideas, requests and questions should be posted on the [issue tracker].

## Code

Pull requests are welcome, though please raise an issue for discussion first if none
exists. We're happy to assist new contributors.

To make changes to the code, fork the repo and clone it:

```
git clone git@github.com:your-username/ploteria.git
```


You'll probably want to install [gnuplot] as well. See the gnuplot website for
installation instructions.

Then make your changes to the code. When you're done, run the tests:

```
cargo test --all
cargo bench
```

It's a good idea to run clippy and fix any warnings as well:

```
rustup component add clippy-preview
cargo clippy --all
```

Finally, run Rustfmt to maintain a common code style:

```
rustup component add rustfmt-preview
cargo fmt --all
```

Don't forget to update the documentation. Once you're finished, push to your fork and
submit a pull request. We try to respond to new issues and pull requests quickly.

Some things that will increase the chance that your pull request is accepted:

* Write tests
* Clearly document public methods
* Write a good commit message

## Code of Conduct

We follow the [Rust Code of Conduct].

[issue tracker]: https://github.com/ploteria/ploteria/issues
[gnuplot]: http://www.gnuplot.info/
[Rust Code of Conduct]: http://www.rust-lang.org/conduct.html
