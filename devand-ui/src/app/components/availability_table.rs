use chrono::Weekday;
use devand_core::{Availability, DaySchedule, WeekSchedule};
use yew::{prelude::*, Properties};

pub struct AvailabilityTable {
    props: Props,
    link: ComponentLink<Self>,
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub schedule: Availability,
    pub on_change: Callback<Availability>,
}

pub enum Msg {
    ResetSchedule,
    ToggleDayHour(Weekday, usize),
}

impl Component for AvailabilityTable {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { props, link }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::ResetSchedule => {
                self.props.schedule = Availability::Weekly(WeekSchedule::default());
                true
            }
            Msg::ToggleDayHour(d, h) => {
                if let Availability::Weekly(week) = &mut self.props.schedule {
                    let day = &mut week[d];
                    day.hours[h] ^= true;
                }
                self.props.on_change.emit(self.props.schedule.clone());
                true
            }
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props = props;
        true
    }

    fn view(&self) -> Html {
        self.view_schedule_panel(&self.props.schedule)
    }
}

impl AvailabilityTable {
    fn view_schedule_panel(&self, schedule: &Availability) -> Html {
        match schedule {
            Availability::Never => self.view_schedule_never(),
            Availability::Weekly(week_schedule) => self.view_week(week_schedule),
        }
    }

    fn view_week(&self, schedule: &WeekSchedule) -> Html {
        html! {
            <fieldset>
                <legend>{ "Your current weekly schedule. Check your available hours. All hours are in UTC" }</legend>
                <div class="schedule-table-wrapper">
                    { self.view_days(schedule) }
                </div>
            </fieldset>
        }
    }

    fn view_days(&self, schedule: &WeekSchedule) -> Html {
        html! {
            <ul class="availability-week">
                { self.view_day(&schedule.mon, Weekday::Mon) }
                { self.view_day(&schedule.tue, Weekday::Tue) }
                { self.view_day(&schedule.wed, Weekday::Wed) }
                { self.view_day(&schedule.thu, Weekday::Thu) }
                { self.view_day(&schedule.fri, Weekday::Fri) }
                { self.view_day(&schedule.sat, Weekday::Sat) }
                { self.view_day(&schedule.sun, Weekday::Sun) }
            </ul>
        }
    }

    fn view_day(&self, schedule: &DaySchedule, day: Weekday) -> Html {
        let hours = schedule.hours.iter().enumerate().map(|(h, &on)| {
            let active = if on {
                Some("pure-button-active pure-button-primary")
            } else {
                None
            };

            html! {
                <button
                    class=("pure-button", active)
                    onclick=self.link.callback(move |_| Msg::ToggleDayHour(day, h))>{ h }</button>
            }
        });

        html! {
            <li class="availability-day">
                <h3 class="availability-day-header">{ format!("{:?}", day) }</h3>
                <div class="availability-day-column">
                    { for hours }
                </div>
            </li>
        }
    }

    fn view_schedule_never(&self) -> Html {
        html! {
            <fieldset>
                <legend>{ "You haven't scheduled anything yet" }</legend>
                <div>
                    <button
                        class="pure-button"
                        onclick=self.link.callback(move |_| Msg::ResetSchedule)>
                        { "Set your availability" }
                    </button>
                </div>
            </fieldset>
        }
    }
}
