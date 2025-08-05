# 🌟 The `.dena` Programming Language

![dena](https://github.com/ylcnky/Dena/blob/master/logo.png "`.dena`")

> A modern, dynamically typed programming language with functional programming features, object-oriented programming, and unique shell integration capabilities.

`.dena` is a feature-rich programming language implemented in [Rust](https://www.rust-lang.org/), inspired by the excellent book [Crafting Interpreters](https://craftinginterpreters.com/). While the book demonstrates implementation in Java, this project showcases the same concepts using Rust's powerful type system and memory safety features.

## 🚀 Quick Start

### Installation & Usage

```bash
# Clone the repository
git clone https://github.com/ylcnky/Dena.git
cd Dena

# Run interactive REPL
cargo run

# Execute a .dena file
cargo run -- your_script.dena

# Execute code directly
cargo run -- e "print \"Hello, World!\";"

# Build for production
cargo build --release
```

### Your First `.dena` Program

```javascript
// hello.dena
fun greet(name) {
    return "Hello, " + name + "!";
}

var message = "World" |> greet;
print message; // "Hello, World!"
```

## 📋 Language Features

### ✅ **Core Data Types**

`.dena` supports essential data types for general-purpose programming:

- **Numbers**: 64-bit floating point numbers
- **Strings**: Text with escape sequence support
- **Booleans**: `true` and `false`
- **Nil**: Represents null/empty values

```javascript
var age = 25;
var name = "Alice";
var isActive = true;
var nothing = nil;
```

### ✅ **Expressions & Operators**

**Arithmetic Operations:**
```javascript
var result = (10 + 5) * 2 - 3 / 1.5; // 28
```

**Comparison & Equality:**
```javascript
var isGreater = 10 > 5;        // true
var isEqual = "hello" == "hi"; // false
var isLessEqual = 3 <= 3;      // true
```

**Logical Operations:**
```javascript
var canVote = age >= 18 and hasId;
var isWeekend = day == "Saturday" or day == "Sunday";
var isNotReady = !isComplete;
```

### ✅ **Variables & Scoping**

Variables use lexical (block) scoping with proper closure support:

```javascript
var global = "I'm global";

{
    var local = "I'm local";
    print global; // Accessible
    print local;  // Accessible
}

// print local; // Error: undefined variable
```

### ✅ **Control Flow**

**Conditional Statements:**
```javascript
if (temperature > 30) {
    print "It's hot!";
} else if (temperature > 20) {
    print "Nice weather!";
} else {
    print "It's cold!";
}
```

**Loops:**
```javascript
// While loop
var count = 0;
while (count < 5) {
    print count;
    count = count + 1;
}

// For loop
for (var i = 0; i < 3; i = i + 1) {
    print "Iteration: " + i;
}
```

### ✅ **Functions**

**Function Definition & Calls:**
```javascript
fun calculateArea(width, height) {
    return width * height;
}

var area = calculateArea(10, 5); // 50
```

**Anonymous Functions:**
```javascript
var square = fun(x) { return x * x; };
print square(4); // 16

// Higher-order functions
fun applyTwice(fn, value) {
    return fn(fn(value));
}

var result = applyTwice(fun(x) { return x + 1; }, 5); // 7
```

### ✅ **Closures**

Functions capture their lexical environment, creating powerful closures:

```javascript
fun makeCounter() {
    var count = 0;
    
    fun increment() {
        count = count + 1;
        return count;
    }
    
    return increment;
}

var counter1 = makeCounter();
var counter2 = makeCounter();

print counter1(); // 1
print counter1(); // 2
print counter2(); // 1 (independent counter)
```

### ✅ **Object-Oriented Programming**

**Class Definition & Instantiation:**
```javascript
class Animal {
    init(name, species) {
        this.name = name;
        this.species = species;
    }
    
    speak() {
        print this.name + " makes a sound";
    }
    
    getInfo() {
        return this.name + " is a " + this.species;
    }
}

var dog = Animal("Rex", "Dog");
dog.speak(); // "Rex makes a sound"
print dog.getInfo(); // "Rex is a Dog"
```

**Dynamic Properties:**
```javascript
class Person {}

var alice = Person();
alice.age = 30;           // Dynamic property assignment
alice.greet = fun() {     // Dynamic method assignment
    print "Hello!";
};

alice.greet(); // "Hello!"
```

### ✅ **Inheritance**

Full inheritance support with method overriding and super calls:

```javascript
class Vehicle {
    init(brand) {
        this.brand = brand;
    }
    
    start() {
        print this.brand + " is starting...";
    }
}

class Car < Vehicle {
    init(brand, model) {
        super.init(brand);  // Call parent constructor
        this.model = model;
    }
    
    start() {
        super.start();      // Call parent method
        print "Car engine running!";
    }
    
    getFullName() {
        return this.brand + " " + this.model;
    }
}

var myCar = Car("Toyota", "Camry");
myCar.start();
// Output:
// "Toyota is starting..."
// "Car engine running!"

print myCar.getFullName(); // "Toyota Camry"
```

### ✅ **Pipe Operator** (Unique Feature)

Chain function calls elegantly with the pipe operator `|>`:

```javascript
fun add(x) { return x + 10; }
fun multiply(x) { return x * 2; }
fun subtract(x) { return x - 5; }

// Traditional nested calls
var result1 = subtract(multiply(add(5))); // 25

// Using pipe operator (reads left to right)
var result2 = 5 |> add |> multiply |> subtract; // 25

// Mix with anonymous functions
var result3 = 10 
    |> fun(x) { return x * 3; }    // 30
    |> fun(x) { return x + 5; }    // 35
    |> fun(x) { return x / 7; };   // 5
```

### ✅ **Command Functions** (Unique Feature)

Execute shell commands directly within `.dena`:

```javascript
// Define command functions
fun listFiles <- "ls -la";
fun getCurrentDate <- "date";
fun searchCode <- "grep -r 'function' src/";

// Execute commands
var files = listFiles();
var today = getCurrentDate();
var matches = searchCode();

print files;   // Directory listing
print today;   // Current date and time
print matches; // Search results
```

**Advanced Command Usage:**
```javascript
fun gitStatus <- "git status --porcelain";
fun processFiles <- "wc -l *.dena";

var status = gitStatus();
if (status != "") {
    print "You have uncommitted changes:";
    print status;
}

var lineCount = processFiles();
print "Total lines in .dena files: " + lineCount;
```

### ✅ **Built-in Functions**

**`clock()`** - Returns current timestamp:
```javascript
var start = clock();

// Some time-consuming operation
for (var i = 0; i < 1000; i = i + 1) {
    // Do something
}

var end = clock();
var duration = end - start;
print "Operation took " + duration + " seconds";
```

### ✅ **Memory Management**

- **Automatic garbage collection** using reference counting
- **No manual memory management** required
- **Efficient closure capture** for functional programming
- **Safe concurrent access** through Rust's ownership system

## 🧪 Examples

### Functional Programming Style
```javascript
fun map(list, fn) {
    // Note: This is a conceptual example
    // Actual list implementation would require arrays
    return list |> fn;
}

fun filter(predicate) {
    return fun(value) {
        if (predicate(value)) {
            return value;
        }
        return nil;
    };
}

// Usage
var isEven = fun(x) { return x % 2 == 0; };
var double = fun(x) { return x * 2; };

var result = 4 |> filter(isEven) |> double; // 8
```

### Object-Oriented Design Pattern
```javascript
class Shape {
    init(name) {
        this.name = name;
    }
    
    area() {
        print "Override this method";
        return 0;
    }
}

class Rectangle < Shape {
    init(width, height) {
        super.init("Rectangle");
        this.width = width;
        this.height = height;
    }
    
    area() {
        return this.width * this.height;
    }
}

class Circle < Shape {
    init(radius) {
        super.init("Circle");
        this.radius = radius;
    }
    
    area() {
        return 3.14159 * this.radius * this.radius;
    }
}

var shapes = [Rectangle(5, 3), Circle(4)];
// Note: Array syntax is conceptual
```

### System Integration
```javascript
class FileManager {
    init() {
        this.listCmd <- "ls -la";
        this.sizeCmd <- "du -sh";
    }
    
    getFiles() {
        return this.listCmd();
    }
    
    getSize() {
        return this.sizeCmd();
    }
    
    processFiles() {
        var files = this.getFiles();
        var size = this.getSize();
        
        return files |> fun(f) { 
            return "Files: " + f + "\nSize: " + size; 
        };
    }
}

var fm = FileManager();
var report = fm.processFiles();
print report;
```

## 🔧 Building & Development

### Prerequisites
- [Rust](https://rustup.rs/) (latest stable version)
- Git

### Development Commands
```bash
# Run tests
cargo test

# Run with verbose output
cargo run -- --verbose your_script.dena

# Check code formatting
cargo fmt

# Run linter
cargo clippy

# Build optimized release
cargo build --release
```

### Project Structure
```
.dena/
├── src/
│   ├── main.rs          # Entry point and CLI
│   ├── scanner.rs       # Lexical analysis
│   ├── parser.rs        # Syntax analysis
│   ├── resolver.rs      # Variable resolution
│   ├── interpreter.rs   # Runtime execution
│   ├── expr.rs          # Expression types
│   ├── stmt.rs          # Statement types
│   ├── environment.rs   # Variable scoping
│   └── tests/           # Test suite
│       └── cases/       # Test cases (.dena files)
├── Cargo.toml           # Rust project configuration
└── README.md            # This file
```

## 🧪 Testing

The language includes a comprehensive test suite with 40+ test cases covering:

- **Basic syntax** and semantics
- **Variable scoping** and closures
- **Function definitions** and calls
- **Object-oriented features**
- **Inheritance** and method resolution
- **Pipe operations** and functional programming
- **Command functions** and system integration
- **Error handling** and edge cases

```bash
# Run all tests
cargo test

# Run specific test
cargo run -- src/tests/cases/your_test.dena
```

## 🤝 Contributing

Contributions are welcome! Areas for improvement:

- **Standard library** functions
- **Array/List** data structures  
- **String manipulation** functions
- **File I/O** operations
- **Error handling** improvements
- **Performance optimizations**

## 📚 Learning Resources

This project was inspired by and follows concepts from:

- **[Crafting Interpreters](https://craftinginterpreters.com/)** by Robert Nystrom
- **[CodeScope Playlist](https://www.youtube.com/playlist?list=PLj_VrUwyDuXS4K3n7X4U4qmkjpuA8rJ76)** - Rust implementation tutorials
- **[Jon Gjengset's Tutorial](https://www.youtube.com/watch?v=mNOLaw-_Buc)** - 8-hour comprehensive guide

## 💝 Why `.dena`?

Why not? 

### **I ❤️ Dena**

---

## 📄 License

This project is open source. Feel free to use, modify, and distribute according to your needs.

## 🙏 Acknowledgments

Special thanks to the Rust community and the creators of educational content that made this project possible. As the author mentions, they are "not a guru of Rust," but this project demonstrates excellent learning and implementation of complex programming language concepts.
