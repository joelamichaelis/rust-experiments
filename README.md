# RUST-EXPERIMENTS

## Context
A personal repo containing a collection of rust files, experiments, and notes which I generate while familiarizing myself with using the rust programming language, coming primarily from a place of proficiency with C & Python.


## Why Rust?
That's what I'm investigating first-hand.

Beyond the obvious, being safety, C-like performance with improved functionality / Python-like convenience, of course.

I'm particularly interested to experience first-hand what its like to write embedded code in rust instead of C, generate quick automation scripts in rust instead of Python, and take it from there.


## Initial Observations
1. I'm appreciating the C-like syntax and strictness when it comes to necessitating variables being immutable by default unless intentionally declared as mutable.

2. I'm appreciating how compiler warnings and errors were designed to flag weird scenarios which compile and I've normalized in my head from my experience with C. Things like being able to perform arithmetic between type mismatched variables, being able to assign a uint8 to be 256 and roll over, etc.

3. I realized very quickly the importance of understanding how cargo works and what all the essential commands do. Especially within the scope of building a monorepo containing a collection of independent rust files. 

## Essential Commands To Remember
Work in progress

VScode side-by-side markdown viewer:
Ctrl + K -> V 

cargo run --bin experiment_name

new .rs experiments are placed in src/bin/
