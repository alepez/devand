use super::{FetchCallback, ScheduleServiceContent};
use devand_core::schedule_matcher::AvailabilityMatch;
use devand_core::{PublicUserProfile, UserId};
use yew::format::{Json, Nothing};
use yew::services::fetch::{FetchService, FetchTask, Request, Response};

const API_AFFINITIES_MATCH_URL: &'static str = "/api/availability-match";

fn api_url_get_user(user_id: UserId) -> String {
    format!("/api/u/{}", user_id.0)
}

pub struct ScheduleService {
    callback: FetchCallback,

    service: FetchService,
    task: Option<FetchTask>,
}

impl ScheduleService {
    pub fn new(callback: FetchCallback) -> Self {
        Self {
            callback,
            service: FetchService::new(),
            task: None,
        }
    }

    pub fn load(&mut self) {
        let url = API_AFFINITIES_MATCH_URL;
        let req = Request::get(url).body(Nothing).unwrap();

        let callback = self.callback.clone();

        let handler = move |response: Response<Json<Result<AvailabilityMatch, anyhow::Error>>>| {
            let (meta, Json(data)) = response.into_parts();
            if let Ok(data) = data {
                callback.emit(Ok(ScheduleServiceContent::AvailabilityMatch(data)));
            } else {
                log::error!("{:?}", &meta);
                // TODO callback.emit();
            }
        };

        self.task = self.service.fetch(req, handler.into()).ok();
    }

    pub fn load_public_profile(&mut self, user_id: UserId) {
        let url = api_url_get_user(user_id);

        let req = Request::get(url).body(Nothing).unwrap();

        let callback = self.callback.clone();

        let handler = move |response: Response<Json<Result<PublicUserProfile, anyhow::Error>>>| {
            let (meta, Json(data)) = response.into_parts();
            if let Ok(data) = data {
                callback.emit(Ok(ScheduleServiceContent::PublicUserProfile(data)));
            } else {
                log::error!("{:?}", &meta);
                // TODO callback.emit();
            }
        };

        self.task = self.service.fetch(req, handler.into()).ok();
    }
}
