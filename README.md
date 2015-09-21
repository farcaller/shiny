Ooh, shiny!
===========

[![Build Status](https://travis-ci.org/farcaller/shiny.svg)](https://travis-ci.org/farcaller/shiny)

Shiny makes you less distracted with copying over initialization code in test cases. It also has a fancy syntax similar to Ruby's RSpec or Objective-C's Kiwi.

Installation
------------

Install in the usual way with cargo.

Usage
-----

Add the `shiny` crate:

```rust
#![feature(plugin)]
#![plugin(shiny)]
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

All the items from `before_each` are inserted as-is into each generated test case function. Mind the final semicolon in `before_each block`!

TODO
----

 * add support for `context` to do recursive prologue injections
 * better filtering for test case name

License
-------

Shiny is distributed under Apache-2.0, see LICENSE for more details.
