use devand_core::{DaySchedule, Schedule, WeekSchedule};
use yew::{prelude::*, Properties};

pub enum Msg {
    ResetSchedule,
    ToggleDayHour(WeekDay, usize),
}

#[derive(Debug, Copy, Clone)]
pub enum WeekDay {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday,
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub schedule: Schedule,
    pub on_change: Callback<Schedule>,
}

pub struct ScheduleTable {
    props: Props,
    link: ComponentLink<Self>,
}

impl Component for ScheduleTable {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { props, link }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::ResetSchedule => {
                self.props.schedule = Schedule::Weekly(WeekSchedule::default());
                true
            }
            Msg::ToggleDayHour(d, h) => {
                if let Schedule::Weekly(week) = &mut self.props.schedule {
                    let day = match d {
                        WeekDay::Monday => &mut week.mon,
                        WeekDay::Tuesday => &mut week.tue,
                        WeekDay::Wednesday => &mut week.wed,
                        WeekDay::Thursday => &mut week.thu,
                        WeekDay::Friday => &mut week.fri,
                        WeekDay::Saturday => &mut week.sat,
                        WeekDay::Sunday => &mut week.sun,
                    };
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

impl ScheduleTable {
    fn view_schedule_panel(&self, schedule: &Schedule) -> Html {
        match schedule {
            Schedule::Never => self.view_schedule_never(),
            Schedule::Weekly(week_schedule) => self.view_schedule_weekly(week_schedule),
        }
    }

    fn view_schedule_day(&self, schedule: &DaySchedule, day: WeekDay) -> Html {
        let hours = schedule.hours.iter().enumerate().map(|(h, &on)| {
            html! {
                <td>
                    <input type="checkbox" checked=on onclick=self.link.callback(move |_| Msg::ToggleDayHour(day, h)) />
                </td>
            }
        });

        html! {
            <tr>
                <td>{ format!("{:?}", day) }</td>
                { for hours }
            </tr>
        }
    }

    fn view_schedule_weekly(&self, schedule: &WeekSchedule) -> Html {
        let hours = (1..=DaySchedule::HOURS_IN_DAY).map(|h| html! { <th>{ h }</th> });

        html! {
            <fieldset>
                <legend>{ "Your current weekly schedule. Check your available hours. All hours are in UTC" }</legend>
                <div class="schedule-table-wrapper">
                    <table class="pure-table pure-table-striped schedule-table">
                        <thead>
                            <tr>
                                <th>{ "Day" }</th>
                                { for hours }
                            </tr>
                        </thead>
                        <tbody>
                            { self.view_schedule_day(&schedule.mon, WeekDay::Monday) }
                            { self.view_schedule_day(&schedule.tue, WeekDay::Tuesday) }
                            { self.view_schedule_day(&schedule.wed, WeekDay::Wednesday) }
                            { self.view_schedule_day(&schedule.thu, WeekDay::Thursday) }
                            { self.view_schedule_day(&schedule.fri, WeekDay::Friday) }
                            { self.view_schedule_day(&schedule.sat, WeekDay::Saturday) }
                            { self.view_schedule_day(&schedule.sun, WeekDay::Sunday) }
                        </tbody>
                    </table>
                </div>
            </fieldset>
        }
    }

    fn view_schedule_never(&self) -> Html {
        html! {
            <fieldset>
                <legend>{ "You haven't scheduled anything yet" }</legend>
                <div><button class="pure-button" onclick=self.link.callback(move |_| Msg::ResetSchedule)>{ "Schedule your availability" }</button></div>
            </fieldset>
        }
    }
}
