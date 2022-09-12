#![crate_name = "foo"]

use std::fmt;

// @!has foo/struct.Bar.html '//*[@id="impl-ToString-for-Bar"]' ''
pub struct Bar;

// @has foo/struct.Foo.html '//*[@id="impl-ToString-for-Foo"]//h3[@class="code-header in-band"]' 'impl<T> ToString for T'
pub struct Foo;
// @has foo/struct.Foo.html '//*[@class="sidebar-elems"]//section//a[@href="#impl-ToString-for-Foo"]' 'ToString'

impl fmt::Display for Foo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Foo")
    }
}