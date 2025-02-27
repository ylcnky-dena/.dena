![dena](https://github.com/ylcnky/Dena/blob/master/logo.png "`.dena`")

Coding a new programming language named `.dena`. This project is inspired from the book [Crafting Interpreters](https://craftinginterpreters.com/) which covers almost everything needed to implement a full-featured, efficient programming language. The book cover these steps in [Java](https://www.java.com/en/) language; and this project tries to implement the same stack with [Rust](https://www.rust-lang.org/) language

It is obvious that this development does not cover all the features and functionalities included in the [Crafting Interpreters](https://craftinginterpreters.com/) book. However, many essential features are handled.

# Features
* It is a dynamically typed language (`Python` lovers welcome ✅). Code runs with an interpreter.
* Memory management with a garbage collector.
### **Data Types**: 
* It is a bit generic language. So not so many data types are supported except the followings:
    * `Booleans`
    * `Numbers`
    * `Strings`
    * `Nil` (or `null` in Java or C)
### **Expressions**: Following expressions are supported:
* **Arithmetic**: simple math operands (`+`, `-`, `*`, `/`)
* **Comparison** and **Equality**: operators which return a `bool` which we can compare numbers (`<`, `>`, `=<`, `>=`, `==`)
* **Logical Operators**: to define `true` or `false` booleans. `!` returns `false` if its operand is `true`
        * `!true` --> `false`
        * `!false` --> `true`
    * The other two logical operands really are control flow. An `and` expression determines if two values are *both* `true`.
    * And an `or` expression determines if *either* of two values are true.
        * `false` or `false` --> `false`
        * `true` or `false` --> `true`
* **Precedence and Grouping**: All of the above operators have the same precedence. You can use `()` to group the stuff. `var agerage = (min + max) / 2;`
### **Statements**:
* Where an expression's main job is to produce *value*, a statement's job is to produce an *effect*. Statements do not evaluate to a value. An expression followed by a semicolon `;` promotes the expression to statement-hood. This is called an expression statement.

### **Variables**:
Variables are declared with `var` statements. If you forget the initializer, the variables value defaults to `nil`

Once declared, you can access and assign a variable by using its name.
```js
var breakfast = "bagels";
print breakfast; // "bagels".
breakfast = "beignets";
print breakfast; // "beignets".
```

### **Control Flows**:
An `if` statement executes one of two statements based on some condition.
```js
if (condition) {
  print "yes";
} else {
  print "no";
}
```
A `while` loop executes the body repeatedly as long as the condition expression evaluates to true.
```js
var a = 1;
while (a < 10) {
  print a;
  a = a + 1;
}
```
Finally, we have for loops.
```js
for (var a = 1; a < 10; a = a + 1) {
  print a;
}
```
This loop does the same thing as the previous `while` loop. Most modern languages also have some sort of `for-in` or `foreach` loop for explicitly iterating over various sequence types. In a real language, that’s nicer than the crude C-style `for` loop we got here. `.dena` keeps it basic.

### **Functions**:
A function call expression looks the same as it does in C.
```js
makeBreakfast(bacon, eggs, toast);
```
You can also call a function without passing anything to it.
```js
makeBreakfast();
```
Unlike in, say, Ruby, the parentheses are mandatory in this case. If you leave them off, the name doesn’t call the function, it just refers to it.

A language isn’t very fun if you can’t define your own functions. In `.dena`, you do that with `fun`.
```js
fun printSum(a, b) {
  print a + b;
}
```
The body of a function is always a block. Inside it, you can return a value using a `return` statement.
```js
fun returnSum(a, b) {
  return a + b;
}
```
If execution reaches the end of the block without hitting a `return`, it implicitly returns `nil`.

### **Closures**:
TBC...

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