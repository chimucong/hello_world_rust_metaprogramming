#[macro_use]
extern crate hello_world_derive;


trait HelloWorld {
    fn hello_world();
}

#[HelloWorldName = "the best Pancakes"]
#[derive(HelloWorld)]
struct Pancakes;

#[derive(HelloWorld)]
struct FrenchToast;

#[derive(HelloWorld)]
struct Waffles;

fn main() {
    Pancakes::hello_world();
    FrenchToast::hello_world();
    Waffles::hello_world();
}


