Ooh, shiny!
===========

[![Build Status](https://travis-ci.org/farcaller/shiny.svg)](https://travis-ci.org/farcaller/shiny)

Shiny makes you less distracted with copying over initializarion code in test cases. It also has a fancy syntax similar to ruby's rspec or Objective-C's kiwi.

Installation
------------

Install in usual way with cargo.

Usage
-----

Add shiny crate:

```rust
#![feature(phase)]
#[cfg(test)] #[phase(plugin,link)] extern crate shiny;
```

Write your shiny test case:
```rust
#[cfg(test)]
mod test {
  describe!(
    before_each {
      let awesome = true;
    }

    it "is awesome" {
      assert!(awesome);
    }

    it "injects before_each into all test cases" {
      let still_awesome = awesome;
      assert!(still_awesome);
    }
  )
}
```

all the items from `before_each` are inserted as is into generated test case functions. Mind the final semicolon in `before_each block`!

TODO
----

 * add support for `context` to do recursive prologue injections
 * better filtering for test case name

License
-------

Shiny is distributed under Apache-2.0, see LICENSE for more details.
