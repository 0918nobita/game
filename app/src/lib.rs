#[macro_use]
extern crate log;

mod command_pool;
pub mod instance;
mod logical_device;
mod physical_device;
mod queue_family_index;

use derive_builder::Builder;

#[derive(Debug, Default, Builder)]
struct Foo {
    a: i32,
    b: String,
}

pub fn test() {
    let builder = Foo::builder();
    let res = builder.a(200).b("hello".to_owned()).build();
    println!("{:?}", res); // => Foo { a: 200, b: "hello" }
}
