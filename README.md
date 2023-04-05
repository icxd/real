# real

Real is a functional and somewhat object-oriented programming language mainly inspired by Haskell and Kotlin.

## Why Real?

I named it Real because I'm suffering from depression and keep seeing the word "real" everywhere on TikTok and I thought it would be funny to name a programming language after it. Oh wait.. you're not laughing? Oh.. I guess I'm the only one who finds it funny. Oh well.
Oh, you mean why should you use Real? Well, I don't know. To be honest, you probably shouldn't, it's probably filled with dumb mistakes and bugs due to the C++ codegen. But if you want to, you can. I'm not stopping you.

## How to use Real

There is no installation process. Just clone the repository and type `cargo run <file>` to run a file. You can also use `cargo build` to build the project and then run the executable in the `target` folder.

After that, you just use a C++ compiler of your choice to compile the generated C++ code. I recommend using `g++` because I haven't tested it with other compilers.

## Syntax

I don't know, figure it out. Here is a Hello, World! example or something.

```real
module MyModule

import Data.String exposing (String)

procedure Main(args: [String]) -> Unit =
    println("Hello, World!")
```

## Documentation

I'm never writing documentation, too much work for me. Just look at the source code, it's not that hard to understand.

## Contributing

Please contribute, because I'm too lazy to do everything myself. Just make a pull request and I'll probably accept it. Well, it's more like I'll probably never finish the language if nobody contributes because I'm incredible at procrastinating and losing motivation.

## License

This project is licensed under the MIT License. See the [LICENSE](./LICENSE) file for more details.