# <span style="color:black">C</span><span style="color:red">o</span><span style="color:green">l</span><span style="color:yellow">o</span><span style="color:blue">r</span><span style="color:magenta">e</span><span style="color:cyan">d</span>

[![Build
Status](https://travis-ci.org/mackwic/colored.svg?branch=master)](https://travis-ci.org/mackwic/colored)

Coloring terminal so simple, you already know how to do it !

```rust
    "this is blue".blue();
    "this is red".red();
    "this is red on blue".red().on_blue();
    "this is also red on blue".on_blue().red();
    "you can also make bold comments".bold();
    println!("{} {} {}", "or use".cyan(), "any".italic().yellow(), "string type".cyan());
    "or change advice. This is red".yellow().blue().red();
    "or clear things up. This is default color and style".red().bold().clear()
    "purple and magenta are the same".purple().magenta()
    "and so are normal and clear".normal().clear()
    String::new("this works also !").green().bold()
```

## How to use

Add this in your `Cargo.toml`:

```toml
    [dependencies]
    colored = "1.0"
```

and add this to your `lib.rs` or `main.rs`:

```rust
    extern crate colored;

    use colored::*;
    
    fn main() {
        // TADAA !
        println!("{} {} !", "it".green(), "works".blue().bold());
    }
```

#### Colors:

- black
- red
- green
- yellow
- blue
- magenta (or purple)
- cyan
- white

Background colors: prepend the color by `on_`. Simple as that.

#### Styles:

- bold
- underline
- italic
- dimmed
- reversed
- blink
- hidden

You can clear color _and_ style anytime by using `normal()` or `clear()`

## Todo

- *Windows console support*: this works only with ansi term. I plan to support
  the windows console also.
- *More tests ?*: We always wecome more tests ! Please contribute !

## Licence

Mozilla Public Licence 2.0.

In non legal terms it means that:
- if you fix a bug, you MUST give me the code (it's only fair)
- if you change/extend the API, you MUST give me the code
- you CAN'T sue me for anything about this code
- appart from that, you can do almost whatever you want. See the LICENCE file
  for details.


