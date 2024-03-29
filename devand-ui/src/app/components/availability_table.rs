use chrono::Weekday;
use devand_core::{Availability, DaySchedule, WeekSchedule};
use devand_text::Text;
use yew::{prelude::*, Properties};
use yewtil::NeqAssign;

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
        self.props.neq_assign(props)
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
            <fieldset class="pure-u-1">
                <legend>{ Text::YourCurrentWeeklySchedule }</legend>
                { self.view_days(schedule) }
            </fieldset>
        }
    }

    fn view_days(&self, schedule: &WeekSchedule) -> Html {
        html! {
            <ul class="devand-availability-week pure-g">
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
                vec!["pure-button-active", "pure-button-primary"]
            } else {
                vec![]
            };

            html! {
                <button
                    class=classes!("pure-button", active)
                    onclick=self.link.callback(move |_| Msg::ToggleDayHour(day, h))>{ h }</button>
            }
        });

        html! {
            <li class="devand-availability-day pure-u-1 pure-u-sm-1-3 pure-u-xl-1-4">
                <h3>{ format!("{:?}", day) }</h3>
                <div class="devand-availability-day-hours">
                    { for hours }
                </div>
            </li>
        }
    }

    fn view_schedule_never(&self) -> Html {
        html! {
            <fieldset class="pure-u-1">
                <legend>{ Text::YouHaventScheduled }</legend>
                <div>
                    <button
                        class="pure-button"
                        onclick=self.link.callback(move |_| Msg::ResetSchedule)>
                        { Text::SetYourAvailability }
                    </button>
                </div>
            </fieldset>
        }
    }
}
