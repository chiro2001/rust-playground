use std::fmt::{Display, Formatter};
use actix::{Actor, Addr, ArbiterHandle, Context, Running, System};

struct MyActor;

impl Actor for MyActor {
    type Context = Context<Self>;

    fn started(&mut self, _ctx: &mut Self::Context) {
        println!("I am alive!");
        System::current().stop(); // <- stop system
    }

    fn stopped(&mut self, _ctx: &mut Self::Context) {
        println!("I am stopping!");
    }
}

fn main() {
    let system = System::new();

    let _addr = system.block_on(async { MyActor.start() });

    system.run().unwrap();
}