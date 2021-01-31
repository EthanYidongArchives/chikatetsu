pub trait Message {
    type Group: MessageGroup;
    type Reply;

    fn to_group(self) -> Self::Group;
}

pub trait MessageGroup: Send + 'static {
    type ReplyGroup: Send + 'static;
}

pub trait Reply: Sized {
    type Group;

    fn from_group(group: Self::Group) -> Option<Self>;
}
