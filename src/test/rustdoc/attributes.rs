// Copyright 2017 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![crate_name = "foo"]
#![feature(non_exhaustive)]

// @has foo/fn.f.html '//*[@class="docblock attributes"]' '#[no_mangle]'
#[no_mangle]
pub extern "C" fn f() {}

// @has foo/fn.g.html '//*[@class="docblock attributes"]' '#[export_name = "bar"]'
#[export_name = "bar"]
pub extern "C" fn g() {}

// @has foo/enum.Foo.html '//*[@class="docblock attributes"]' '#[repr(i64)]'
// @has foo/enum.Foo.html '//*[@class="docblock attributes"]' '#[must_use]'
// @has foo/enum.Foo.html '//*[@class="docblock attributes"]' '#[non_exhaustive]'
#[repr(i64)]
#[must_use]
#[non_exhaustive]
pub enum Foo {
    Bar,
}
