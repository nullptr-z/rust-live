use futures::{stream, Stream};
use std::{pin::Pin, sync::Arc};
use tokio_stream::wrappers::ReceiverStream;

use crate::{
    pb::abi::{CommandResponse, Publish, Subscribe, Unsubscribe},
    topic::Topic,
};

pub type StreamingResponse = Pin<Box<dyn Stream<Item = Arc<CommandResponse>> + Send>>;
pub trait TopicService {
    fn execute(self, topic: impl Topic) -> impl Stream<Item = Arc<CommandResponse>> + Send;
}

impl TopicService for Subscribe {
    fn execute(self, topic: impl Topic) -> impl Stream<Item = Arc<CommandResponse>> + Send {
        let rx = topic.subscript(self.topic);
        ReceiverStream::new(rx)
    }
}

impl TopicService for Unsubscribe {
    fn execute(self, topic: impl Topic) -> impl Stream<Item = Arc<CommandResponse>> + Send {
        topic.unsubscript(self.topic, self.id);
        stream::once(async { Arc::new(CommandResponse::ok()) })
    }
}

impl TopicService for Publish {
    fn execute(self, topic: impl Topic) -> impl Stream<Item = Arc<CommandResponse>> + Send {
        topic.publish(self.topic, Arc::new(self.data.into()));
        stream::once(async { Arc::new(CommandResponse::ok()) })
    }
}
