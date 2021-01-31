use async_trait::async_trait;
use crate::message::Message;

#[async_trait]
pub trait Handler<M: Message> {
    async fn handle(&mut self, msg: M) -> <M as Message>::Reply;
}
