use super::{FetchCallback, ScheduleServiceContent};
use devand_core::schedule_matcher::AvailabilityMatch;
use devand_core::{PublicUserProfile, UserId};
use std::collections::{BTreeMap, BTreeSet};
use std::sync::{Arc, Mutex};
use yew::format::{Json, Nothing};
use yew::services::fetch::{FetchService, FetchTask, Request, Response};

const API_AFFINITIES_MATCH_URL: &'static str = "/api/availability-match";

fn api_url_get_user(user_id: UserId) -> String {
    format!("/api/u/{}", user_id.0)
}

pub struct ScheduleService {
    callback: FetchCallback,

    service: FetchService,

    // TODO Possible memory leak (tasks can grow forever)
    tasks: Vec<FetchTask>,

    user_cache: Arc<Mutex<BTreeMap<UserId, PublicUserProfile>>>,
    user_requests: BTreeSet<UserId>,
}

impl ScheduleService {
    pub fn new(callback: FetchCallback) -> Self {
        Self {
            callback,
            service: FetchService::new(),
            tasks: Vec::new(),
            user_cache: Arc::new(Mutex::new(BTreeMap::default())),
            user_requests: BTreeSet::default(),
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

        let task = self.service.fetch(req, handler.into()).ok();
        self.enqueue_task(task);
    }

    pub fn load_public_profile(&mut self, user_id: UserId) {
        if self.user_requests.contains(&user_id) {
            if let Some(data) = self.user_cache.lock().unwrap().get(&user_id) {
                self.callback
                    .emit(Ok(ScheduleServiceContent::PublicUserProfile(data.clone())));
            }
            return;
        }

        self.user_requests.insert(user_id);

        let url = api_url_get_user(user_id);

        let req = Request::get(url).body(Nothing).unwrap();

        let callback = self.callback.clone();

        let user_cache = self.user_cache.clone();

        let handler = move |response: Response<Json<Result<PublicUserProfile, anyhow::Error>>>| {
            let (meta, Json(data)) = response.into_parts();
            if let Ok(data) = data {
                user_cache.lock().unwrap().insert(data.id, data.clone());
                callback.emit(Ok(ScheduleServiceContent::PublicUserProfile(data)));
            } else {
                log::error!("{:?}", &meta);
                // TODO callback.emit();
            }
        };

        let task = self.service.fetch(req, handler.into()).ok();
        self.enqueue_task(task);
    }

    fn enqueue_task(&mut self, task: Option<FetchTask>) {
        if let Some(task) = task {
            self.tasks.push(task);
        }
    }
}
