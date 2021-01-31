use tokio::sync::*;

use async_trait::async_trait;

use std::marker::PhantomData;

use crate::message::*;

#[async_trait]
pub trait Actor: Sized + Send + 'static {
    type MessageGroup: MessageGroup;

    async fn handle_all(
        &mut self,
        msg: Self::MessageGroup,
    ) -> <Self::MessageGroup as MessageGroup>::ReplyGroup;
    fn start(mut self) -> ActorHandle<Self> {
        let (sender, mut rx) = mpsc::channel(32);
        let handle = ActorHandle {
            sender,
            _actor: PhantomData,
        };

        tokio::spawn(async move {
            while let Some((msg, tx_reply)) = rx.recv().await {
                let _ = tx_reply.send(self.handle_all(msg).await);
            }
        });

        handle
    }
}

#[derive(Clone)]
pub struct ActorHandle<A: Actor> {
    sender: mpsc::Sender<(
        A::MessageGroup,
        oneshot::Sender<<A::MessageGroup as MessageGroup>::ReplyGroup>,
    )>,
    _actor: PhantomData<A>,
}

impl<A: Actor> ActorHandle<A> {
    pub async fn send<M, R>(&self, msg: M) -> R
    where
        M: Message<Group = A::MessageGroup, Reply = R>,
        R: Reply<Group = <A::MessageGroup as MessageGroup>::ReplyGroup>,
    {
        let (tx, rx) = oneshot::channel();
        let _ = self.sender.send((msg.to_group(), tx)).await;
        R::from_group(rx.await.unwrap()).unwrap()
    }
}
