# incept readme:
# 📘 INCEPT — A Language Explained from First Principles

INCEPT is a programming language. Before we explain the language, we must first explain what a programming language is.

---

## 🔹 What Is a “Programming Language”?

A **programming language** is a system of words and rules that humans use to give instructions to a computer.

- **Instruction** means a command that tells the computer what to do.
- A **computer** is an electronic device that follows instructions step by step.

Humans write instructions in text form, and then a computer uses a tool (called a **compiler**) to turn those instructions into something it can run.

Every programming language has:
- **Words** — the basic pieces of text
- **Syntax** — the rules for how words combine
- **Semantics** — the meaning of those combinations

INCEPT is designed so that **every valid program** in it must be:
- ✅ Correct (no invalid logic)
- ✅ Deterministic (same output for same input)
- ✅ Verifiable (provable behavior)
- ✅ High‑performance
- ✅ Infinitely adaptable to many domains and styles

---

## 🔹 Your Principles, Restated in Simple Words

### ✔ Verifiable Correctness

- **Correctness**: the instructions do what they are supposed to do and nothing else.
- **Verifiable**: we can check this fact with certainty.

Example: If you have an `Age`, INCEPT won’t let you store a negative number in it — the code won’t compile.

---

### ✔ Infinity

A combination of two ideas:
- The language can be **customized** (changed, extended).
- It is **not limited** to certain styles or problem areas.

The user can express any idea clearly and correctly — no structure is forced.

---

### ✔ Determinism

- A program is **deterministic** if it always gives the same result for the same input.
- INCEPT does not allow hidden state or unpredictable behavior.

---

### ✔ Performance

- Code must run **efficiently**.
- This includes low memory, fast execution, and no unnecessary abstraction overhead.

---

### ✔ Innovation‑as‑Exploration

- INCEPT allows new ideas to be expressed and tested.
- “Bad ideas from history” are not rejected automatically — everything gets a fair chance under new light.

---

## 🔹 Basic Concepts Before We Show Code

### 📌 Value

A **value** is a piece of data the program uses.
Examples: `5`, `"hello"`, `true`

---

### 📌 Type

A **type** is a label that tells what kind of value something is.
- Example: `Number` = whole numbers
- Example: `Text` = sequence of letters

---

### 📌 Variable

A **variable** is a name that refers to a value.  
Think of it like a labeled box that holds a value.

---

### 📌 Function

A **function** is named code that takes input, does something, and returns output.
Example: `add(a, b)` returns the sum of two numbers.

---

### 📌 Statement

A **statement** is one instruction the computer performs.

---

### 📌 Expression

An **expression** is anything that produces a value when evaluated.

---

## 🔹 Example: A Simple INCEPT Program

### Line-by-line:

```incept
program SimpleExample
```

- `program`: declares a new program
- `SimpleExample`: the program’s name

---

```incept
type Age = Integer where value >= 0 and value < 150
```

- `type`: we are defining a custom type
- `Age`: the name of the type
- `Integer`: the base type
- `where`: adds a **constraint**
- Constraints: must be between 0 (inclusive) and 150 (exclusive)

---

```incept
function birthday(personAge: Age) -> Age
```

- `function`: defines a named behavior
- `birthday`: function name
- `personAge: Age`: input variable of type `Age`
- `-> Age`: the function returns an `Age`

---

```incept
    return personAge + 1
```

- `return`: sends the result back
- `personAge + 1`: adds 1 to the age

This line is valid **only if** `personAge + 1` is still a valid `Age`.

---

## 🔹 What This Example Shows

- ✅ Types can encode meaningful constraints
- ✅ Invalid states are unrepresentable
- ✅ Behavior is deterministic
- ✅ Compiler rejects unprovable logic

---

## 🔹 A More Complex Example: Banking Logic

```incept
type Balance = Integer where value >= 0

type Withdrawal(amount: Integer)
    where amount >= 0 and amount <= accountBalance
```

- `Balance` ensures non-negative account balances
- `Withdrawal` ensures no overdrawing:
  - `amount >= 0`
  - `amount <= accountBalance`

You **cannot write** code that withdraws too much. The language stops you.

---

## 🔹 Principles Illustrated

| Principle                 | How This Example Shows It                                        |
|--------------------------|------------------------------------------------------------------|
| Verifiable Correctness   | Types + constraints prevent illegal logic                        |
| Infinity                 | Rules are domain-specific, not hardcoded                         |
| Determinism              | `+1` always returns the same result                              |
| Performance              | No runtime checking is needed — enforced at compile time         |
| Innovation-as-Exploration| The constraint-style syntax is experimental but expressive       |

---

## 🔹 Key Language Concepts (Plain English)

### 📌 Compile Time

The time when your code is **checked before it runs**.

### 📌 Runtime

When your code actually **executes**.

### 📌 Constraint

A rule attached to a type that must always be true.

### 📌 Proof

A logical argument (by math or type system) that guarantees a rule is true.

---

## 🧠 Final Notes

INCEPT is not just syntax — it is a language of **truth**, **structure**, and **controlled expression**.

Every piece of INCEPT code:
- ✅ Must be provably correct
- ✅ Cannot behave in hidden or surprising ways
- ✅ Cannot express invalid logic
- ✅ Can be adapted to any structure or style
- ✅ Is built to explore, extend, and evolve





# Rust readme:
<div align="center">
  <picture>
    <source media="(prefers-color-scheme: dark)" srcset="https://raw.githubusercontent.com/rust-lang/www.rust-lang.org/master/static/images/rust-social-wide-dark.svg">
    <source media="(prefers-color-scheme: light)" srcset="https://raw.githubusercontent.com/rust-lang/www.rust-lang.org/master/static/images/rust-social-wide-light.svg">
    <img alt="The Rust Programming Language: A language empowering everyone to build reliable and efficient software"
         src="https://raw.githubusercontent.com/rust-lang/www.rust-lang.org/master/static/images/rust-social-wide-light.svg"
         width="50%">
  </picture>

[Website][Rust] | [Getting started] | [Learn] | [Documentation] | [Contributing]
</div>

This is the main source code repository for [Rust]. It contains the compiler,
standard library, and documentation.

[Rust]: https://www.rust-lang.org/
[Getting Started]: https://www.rust-lang.org/learn/get-started
[Learn]: https://www.rust-lang.org/learn
[Documentation]: https://www.rust-lang.org/learn#learn-use
[Contributing]: CONTRIBUTING.md

## Why Rust?

- **Performance:** Fast and memory-efficient, suitable for critical services, embedded devices, and easily integrated with other languages.

- **Reliability:** Our rich type system and ownership model ensure memory and thread safety, reducing bugs at compile-time.

- **Productivity:** Comprehensive documentation, a compiler committed to providing great diagnostics, and advanced tooling including package manager and build tool ([Cargo]), auto-formatter ([rustfmt]), linter ([Clippy]) and editor support ([rust-analyzer]).

[Cargo]: https://github.com/rust-lang/cargo
[rustfmt]: https://github.com/rust-lang/rustfmt
[Clippy]: https://github.com/rust-lang/rust-clippy
[rust-analyzer]: https://github.com/rust-lang/rust-analyzer

## Quick Start

Read ["Installation"] from [The Book].

["Installation"]: https://doc.rust-lang.org/book/ch01-01-installation.html
[The Book]: https://doc.rust-lang.org/book/index.html

## Installing from Source

If you really want to install from source (though this is not recommended), see
[INSTALL.md](INSTALL.md).

## Getting Help

See https://www.rust-lang.org/community for a list of chat platforms and forums.

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md).

## License

Rust is primarily distributed under the terms of both the MIT license and the
Apache License (Version 2.0), with portions covered by various BSD-like
licenses.

See [LICENSE-APACHE](LICENSE-APACHE), [LICENSE-MIT](LICENSE-MIT), and
[COPYRIGHT](COPYRIGHT) for details.

## Trademark

[The Rust Foundation][rust-foundation] owns and protects the Rust and Cargo
trademarks and logos (the "Rust Trademarks").

If you want to use these names or brands, please read the
[Rust language trademark policy][trademark-policy].

Third-party logos may be subject to third-party copyrights and trademarks. See
[Licenses][policies-licenses] for details.

[rust-foundation]: https://rustfoundation.org/
[trademark-policy]: https://rustfoundation.org/policy/rust-trademark-policy/
[policies-licenses]: https://www.rust-lang.org/policies/licenses
