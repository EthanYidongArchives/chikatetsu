use chikatetsu::prelude::*;

#[derive(PartialEq, Debug)]
pub struct Add(i32, i32);
#[derive(PartialEq, Debug)]
pub struct AddResult(i32);

#[derive(PartialEq, Debug)]
pub struct Subtract(i32, i32);
#[derive(PartialEq, Debug)]
pub struct SubtractResult(i32);

/*
Generated enums:
pub enum MathActorMessages {
    Add(Add),
    Subtract(Subtract),
}

pub enum MathActorReplies {
    AddResult(AddResult),
    SubtractResult(SubtractResult),
}
*/

#[derive(Actor)]
#[handles(Add, AddResult)]
#[handles(Subtract, SubtractResult)]
struct MathActor;

#[async_trait]
impl Handler<Add> for MathActor {
    async fn handle(&mut self, add: Add) -> AddResult {
        AddResult(add.0 + add.1)
    }
}

#[async_trait]
impl Handler<Subtract> for MathActor {
    async fn handle(&mut self, add: Subtract) -> SubtractResult {
        SubtractResult(add.0 - add.1)
    }
}

#[tokio::main]
async fn main() {
    let actor = MathActor.start();

    assert_eq!(AddResult(3), actor.send(Add(1, 2)).await);
    assert_eq!(SubtractResult(-1), actor.send(Subtract(1, 2)).await);
}
