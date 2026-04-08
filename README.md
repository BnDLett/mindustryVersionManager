# Mindustry Version Manager (Rust Branch)
A program to switch between Mindustry versions and profiles \
*(click on alpha-chan for a demo)* \
[![alphaaaa](src/assets/alphaaaa_128.png)](https://youtu.be/A8TwZQHvblY) \
*Note: this is designed **only** for Linux (so far)!*

## Goal of the Rust branch
The current goal of the Rust branch (or, rather, rewriting it in Rust) is to make it multiplatform. The issue with
Python is that it needs the interpreter installed on the machine already. Sounds fine in theory, but I don't feel like
guiding anyone through how to install Python, nor should it have to be a dependency for a simple version manager. As
such, I decided that Rust may be the best language to rewrite the version manager in. The structure system—alongside the
libraries—are extremely useful for what I intend on doing. 

# Installation
## Linux
1. Git clone the repository to any directory
2. `cd` to the root of the repository
3. Run `python3 -m mind_ver install`
