use crate::auth::{AuthData, LoggedUser};
use crate::CodeNowUsers;
use crate::PgDevandConn;
use chrono::prelude::*;
use chrono::Duration;
use devand_core::{Availability, DaySchedule, User, UserAffinity, WeekSchedule};
use rocket::{Route, State};
use rocket_contrib::json::Json;
use serde::{Deserialize, Serialize};

pub fn routes() -> Vec<Route> {
    routes![settings, settings_put, affinities, code_now]
}

#[get("/settings")]
fn settings(user: LoggedUser) -> Json<User> {
    Json(user.into())
}

#[put("/settings", data = "<user>")]
fn settings_put(auth_data: AuthData, user: Json<User>, conn: PgDevandConn) -> Option<Json<User>> {
    // Note: here we don't need LoggedUser (needs db access) but only
    // auth_data to check if we are modifiyng the right user.
    if !auth_data.matches_user(&user) {
        return None;
    }

    devand_db::save_user(user.0, &conn.0).map(|x| Json(x))
}

#[get("/affinities")]
fn affinities(user: LoggedUser, conn: PgDevandConn) -> Option<Json<Vec<UserAffinity>>> {
    let users = devand_db::load_users(&conn.0)?;
    let affinities = devand_core::calculate_affinities(&user.into(), users);
    Some(Json(affinities.collect()))
}

#[get("/code-now")]
fn code_now(user: LoggedUser, code_now_users: State<CodeNowUsers>) -> Json<devand_core::CodeNow> {
    let user: User = user.into();

    // We always need a write lock, because we are updating cache ttl
    let mut cache = code_now_users.0.write().unwrap();

    let all_users: devand_core::CodeNowUsers = {
        if !cache.touch(user.id) {
            // Now we can add the current user to the cache
            cache.add(user.clone());
        }

        cache.clone().into()
    };

    Json(devand_core::CodeNow {
        current_user: user,
        all_users: all_users.0,
    })
}

#[get("/availability-match")]
fn availability_match(user: LoggedUser, conn: PgDevandConn) -> Json<AvailabilityMatch> {
    let now = Utc::now();
    let next_week = now.checked_add_signed(Duration::days(7)).unwrap();
    let days = days_from(7, next_week);
    let User { settings, .. } = user.into();
    let availability = settings.schedule;
    let future_availability = attach_schedule(days, availability);

    let res = AvailabilityMatch {};

    Json(res)
}

#[derive(Serialize, Deserialize)]
struct AvailabilityMatch {}

fn get_day_schedule(date: Date<Utc>, week_schedule: &WeekSchedule) -> (Date<Utc>, DaySchedule) {
    let sched = match date.weekday() {
        Weekday::Mon => week_schedule.mon.clone(),
        Weekday::Tue => week_schedule.tue.clone(),
        Weekday::Wed => week_schedule.wed.clone(),
        Weekday::Thu => week_schedule.thu.clone(),
        Weekday::Fri => week_schedule.fri.clone(),
        Weekday::Sat => week_schedule.sat.clone(),
        Weekday::Sun => week_schedule.sun.clone(),
    };

    (date, sched)
}

fn attach_schedule(
    days: Vec<Date<Utc>>,
    availability: Availability,
) -> Vec<(Date<Utc>, DaySchedule)> {
    match availability {
        Availability::Never => todo!(),
        Availability::Weekly(week_schedule) => days
            .iter()
            .map(|day| get_day_schedule(*day, &week_schedule))
            .collect(),
    }
}

fn days_from(n: usize, from: DateTime<Utc>) -> Vec<Date<Utc>> {
    (0..n)
        .into_iter()
        .filter_map(|x| from.checked_add_signed(Duration::days(x as i64)))
        .map(|x| x.date())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn availability_match_ok() {
        dbg!(days_from(7, Utc::now()));
    }

    #[test]
    fn future_availability_ok() {
        let now = Utc::now();
        let next_week = now.checked_add_signed(Duration::days(7)).unwrap();
        let days = days_from(7, next_week);
        let User { settings, .. } = devand_core::mock::user();
        let availability = settings.schedule;
        let future_availability = attach_schedule(days, availability);
        dbg!(future_availability);
    }
}
