# chikatetsu

An(other) actor framework for tokio.

## Why?

Most other actor frameworks use some form of `dyn Any`, which is basically telling the compiler, "screw your type checking, I have `Box`es".

Instead of using dynamic dispatch, we generate and enum that contains all possible types of messages an actor will handle. This is done automagically through a derive macro, and is handeled behind-the-scenes, so you technically never have to use the generated enum in your code (see example below).

```Rust
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
```

## Caveats
Currently, all message and reply types must be valid enum names, so no primitives or paths. Simple workarounds include using a type alias. A permanent fix would be to allow custom names in the attributes, but I'm bad at macros so that'll take a while.

Also, the generated enums cannot be renamed at the moment.

If you use this (please don't), definitely report any issues you face (or even better, make a PR).
