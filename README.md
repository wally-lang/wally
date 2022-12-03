# Wally
[![](https://img.shields.io/badge/Version-0.0.1-blue)]( https://www.github.com/0x1CA3/Wally/releases/tag/v0.0.1 )

> **Warning**
> Wally is currently in the pre-beta stages in development meaning that anything here could change at any time, it is not recommended to use this language at the moment for any serious projects due to this reason.

Wally is a object-oriented statically-typed programming language inspired by Rust, Java, C, C++ and more. It is designed to be a general purpose language that can be used for many different things such as web development, game development, system programming and more. It is designed to be a language that is easy to learn and use, but also powerful enough to be used for many different things.

## Table of Contents
- [Hello World](#hello-world)
- [Compiling](#compiling)
- [Documentation](#documentation)
- [Contributing](#contributing)
- [License](#license)

## Hello World
To create a Hello World program in Wally, you would do the following:
1. Create a file called `main.wly`
2. Write the following code in the file:
```c++
fn main(args: array<string>): int {
    println("Hello World!");
    return 0;
}
```
3. Compile the file using the `wallyc` compiler
4. Run the compiled file

## Compiling
To compile a Wally file, you would use the `wallyc` compiler. The compiler is written in Rust and can be found [here](compiler).

You can compile a Wally file by running the following command, and yes, you need to have the compiler installed for this to work (see the link above for more information on how to install the compiler):
```bash
wallyc <file> -o <output>
```

## Documentation
The documentation for Wally can be found [here](docs).

## Contributing
If you would like to contribute to Wally, you can do so by creating a pull request on the [GitHub repository](lang-repo). If you would like to help with the documentation, you can do so by creating a pull request on the [GitHub repository](docs-repo).

## License
Wally is licensed under the MIT license. You can find the license [here](licence).

[compiler]: https://www.github.com/wally-lang/wallyc
[docs]: https://wally-lang.github.io/docs/
[lang-repo]: https://www.github.com/wally-lang/wally
[docs-repo]: https://www.github.com/wally-lang/docs
[licence]: https://www.github.com/wally-lang/wally/blob/master/LICENSE
