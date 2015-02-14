#![feature(plugin)]
#![crate_type = "dylib"]

#![plugin(shiny)]

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
  );
}
