mod arena;
mod characters;
mod equipments;
mod rules;
mod actions;
mod tables;
mod console;

fn main() {
    let mut arena = arena::Arena::test();
    arena.run_simulation()
}