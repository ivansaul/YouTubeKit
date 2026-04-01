use crate::{
    client::{
        innertube::{config::ClientType, endpoint::YTEndpoint},
        QPlayer, RequestOpts, RusTube,
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

        let body = QPlayer {
            video_id: &self.video_id,
        };

        self.client
            .inner
            .execute_request::<PlayerResponse, _, _>(
                &ctype,
                YTEndpoint::Player,
                &self.video_id,
                &body,
                &self.opts,
            )
            .await
    }
}
