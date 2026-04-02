use futures::future;

use crate::{
    client::{
        innertube::{config::ClientType, endpoint::YTEndpoint},
        QVideo, RequestOpts, RusTube,
    },
    error::Error,
    models::VideoDetails,
    response::player::PlayerResponse,
};

pub struct VideoBuilder {
    client: RusTube,
    pub(crate) video_id: String,
    opts: RequestOpts,
}

impl VideoBuilder {
    pub(crate) fn new(client: RusTube, video_id: String, opts: RequestOpts) -> Self {
        Self {
            client,
            video_id,
            opts,
        }
    }
}

impl VideoBuilder {
    pub async fn send(self) -> Result<VideoDetails, Error> {
        let ctype = ClientType::WEB;

        let request_body = QVideo {
            video_id: &self.video_id,
        };

        let future_player = self.client.inner.execute_request::<PlayerResponse, _, _>(
            &ctype,
            YTEndpoint::Player,
            &self.video_id,
            &request_body,
            &self.opts,
        );

        let future_next = self
            .client
            .inner
            .execute_request::<crate::response::next::VideoDetails, _, _>(
                &ctype,
                YTEndpoint::Next,
                &self.video_id,
                &request_body,
                &self.opts,
            );

        let (player, mut next) = future::try_join(future_player, future_next).await?;
        enrich_with_player(&mut next, player);
        Ok(next)
    }
}

fn enrich_with_player(next: &mut VideoDetails, player: VideoDetails) {
    next.duration = player.duration;
    next.is_short = player.is_short;
    next.keywords = player.keywords;
    next.thumbnail = player.thumbnail;
    next.category = player.category;
    next.publish_date = player.publish_date;
}
