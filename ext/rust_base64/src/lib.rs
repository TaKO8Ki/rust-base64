use base64::{engine::general_purpose, Engine as _};
use magnus::{define_module, function, prelude::*, Error};
use rb_allocator::ruby_global_allocator;

fn encode(input: String) -> String {
    general_purpose::STANDARD.encode(input)
}

fn decode(input: String) -> String {
    let bytes = general_purpose::STANDARD.decode(input).unwrap();
    String::from_utf8(bytes).unwrap()
}

ruby_global_allocator!();

#[magnus::init]
fn init() -> Result<(), Error> {
    let module = define_module("RustBase64")?;
    module.define_singleton_method("encode", function!(encode, 1))?;
    module.define_singleton_method("decode", function!(decode, 1))?;
    Ok(())
}
