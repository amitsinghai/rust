// Copyright 2012 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

struct dog {
    mut cats_chased: uint,
}

impl dog {
    fn chase_cat() {
        let p: &static/mut uint = &mut self.cats_chased; //~ ERROR illegal borrow
        *p += 1u;
    }

    fn chase_cat_2() {
        let p: &blk/mut uint = &mut self.cats_chased;
        *p += 1u;
    }
}

fn dog() -> dog {
    dog {
        cats_chased: 0u
    }
}

fn main() {
    let d = dog();
    d.chase_cat();
    debug!("cats_chased: %u", d.cats_chased);
}

