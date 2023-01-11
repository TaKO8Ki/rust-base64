use magnus::{define_module, function, prelude::*, Error};
use rb_allocator::ruby_global_allocator;

ruby_global_allocator!();

fn foo(foo: &str) {}

#[magnus::init]
fn init() -> Result<(), Error> {
    let module = define_module("RustBase64")?;
    module.define_singleton_method("encode", function!(foo, 1))?;
    Ok(())
}
