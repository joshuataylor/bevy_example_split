use bevy::prelude::*;
use bevy_egui::{egui, EguiContext, EguiPlugin};

fn main() {
    println!("Hello, world!");
}

struct Foo;
struct Bar;
struct Baz;
struct Something;

fn something(
    mut query: Query<(&mut Foo, &mut Bar, &mut Baz), With<Something>>
) {
    // should show here
    let (mut foo, mut bar, mut baz) = query.single_mut().unwrap();
}