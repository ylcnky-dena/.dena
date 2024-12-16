![dena](https://github.com/ylcnky/Dena/blob/master/logo.png "`.dena`")

Coding a new programming language named `.dena`. This project is inspired from the book [Crafting Interpreters](https://craftinginterpreters.com/) which covers almost everything needed to implement a full-featured, efficient programming language. The book cover these steps in [Java](https://www.java.com/en/) language; and this project tries to implement the same stack with [Rust](https://www.rust-lang.org/) language.

It is obvious that this development does not cover all the features and functionalities included in the [Crafting Interpreters](https://craftinginterpreters.com/) book. However, many essential features are handled.

## Features
* It is a dynamically typed language (`Python` lovers welcome ✅). Code runs with an interpreter.
* Memory management with a garbage collector.
* It is a bit generic language. So not so many data types are supported except the followings:
    * `Booleans`
    * `Numbers`
    * `Strings`
    * `Nil` (or `null` in Java or C)
* Following expressions are supported:
    * **Arithmetic**: simple math operands (`+`, `-`, `*`, `/`)
    * **Comparison** and **Equality**: operators which return a `bool` which we can compare numbers (`<`, `>`, `=<`, `>=`, `==`)
    * Logical Operators: to define `true` or `false` booleans. `!` returns `false` if its operand is `true`
        * `!true` --> `false`
        * `!false` --> `true`
    * The other two logical operands really are control flow. An `and` expression determines if two values are *both* `true`.
    * And an `or` expression determines if *either* of two values are true.
        * `false` or `false` --> `false`
        * `true` or `false` --> `true`
    * Precedence and Grouping: All of the above operators have the same precedence. You can use `()` to group the stuff. `var agerage = (min + max) / 2;`


* The language can be run via Rust by typing `cargo r`, starting an interpreter in Terminal (or CMD)
* Code snippets can be saved in files with `.dena` extension (i.e `my_program.dena`)

## How to build?
Since the background of this code is based on Rust, you can use the `cargo`.

If you want to run without building, simply run `cargo r`. This will start the interpreter of the language.

If you want to run the code in `.dena` files, you can try the test files in [`/test`](https://github.com/ylcnky/.dena/tree/master/test) via `cargo r -- test/file_name.dena`

If you want to run the language as part of your runtime (`CMD`, `Terminal`), then simply build the language from source via `cargo b` and move the pre-build binary in `target/debug/` directory to your `ENV` variables. Then you can start the intepreter with `dena` command.

## Why I named as `.dena`? 
Why not? 

### **I ❤️ Dena**

### Acknowledgement
As being the owner of this repository, I am not a guru of Rust. So great kudos for the people in community who have already done similar projects. Please see the playlist of [CodeScope](https://www.youtube.com/playlist?list=PLj_VrUwyDuXS4K3n7X4U4qmkjpuA8rJ76), and almost 8 hours tutorial of [Jon Gjengset](https://www.youtube.com/watch?v=mNOLaw-_Buc&pp=ygUnY3JlYXRlIGEgcHJvZ3JhbW1pbmcgbGFuZ3VhZ2Ugd2l0aCBydXN0) for excellent and more comprehensive implementation of the same project.