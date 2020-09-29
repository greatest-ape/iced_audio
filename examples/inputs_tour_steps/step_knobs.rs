use iced::{Column, Element, Length, Row, Text};

use iced_audio::{
    knob, text_marks, tick_marks, FloatRange, FreqRange, IntRange, Knob,
    LogDBRange,
};

use crate::{style, Step};

/// Unique identifier for each parameter. Note you may also use u32, i32, or
/// Strings if you wish.
#[derive(Debug, Copy, Clone)]
pub enum KnobsID {
    Float,
    Int,
    DB,
    Freq,
    Style1,
    Style2,
    Style3,
    Style4,
}

#[derive(Debug, Clone)]
pub enum Message {
    KnobMoved(KnobsID),
}

pub struct KnobStep {
    float_range: FloatRange,
    int_range: IntRange,
    db_range: LogDBRange,
    freq_range: FreqRange,

    knob_float_state: knob::State<KnobsID>,
    knob_int_state: knob::State<KnobsID>,
    knob_db_state: knob::State<KnobsID>,
    knob_freq_state: knob::State<KnobsID>,
    knob_style1_state: knob::State<KnobsID>,
    knob_style2_state: knob::State<KnobsID>,
    knob_style3_state: knob::State<KnobsID>,
    knob_style4_state: knob::State<KnobsID>,

    float_tick_marks: tick_marks::Group,
    int_tick_marks: tick_marks::Group,
    db_tick_marks: tick_marks::Group,
    freq_tick_marks: tick_marks::Group,

    float_text_marks: text_marks::Group,
    int_text_marks: text_marks::Group,
    db_text_marks: text_marks::Group,
    freq_text_marks: text_marks::Group,

    output_text: String,
}

impl Default for KnobStep {
    fn default() -> Self {
        // initalize parameters

        let float_range = FloatRange::default_bipolar();
        let int_range = IntRange::new(0, 5);
        let db_range = LogDBRange::default();
        let freq_range = FreqRange::default();

        // create application

        Self {
            float_range,
            int_range,
            db_range,
            freq_range,

            // initialize the state of the Knob widget
            knob_float_state: knob::State::new(
                float_range.create_param_default(KnobsID::Float),
            ),

            knob_int_state: knob::State::new(
                int_range.create_param_default(KnobsID::Int),
            ),

            knob_db_state: knob::State::new(
                db_range.create_param_default(KnobsID::DB),
            ),

            knob_freq_state: knob::State::new(freq_range.create_param(
                KnobsID::Freq,
                1000.0,
                1000.0,
            )),

            knob_style1_state: knob::State::new(
                float_range.create_param_default(KnobsID::Style1),
            ),

            knob_style2_state: knob::State::new(
                float_range.create_param_default(KnobsID::Style2),
            ),

            knob_style3_state: knob::State::new(
                float_range.create_param_default(KnobsID::Style3),
            ),

            knob_style4_state: knob::State::new(
                float_range.create_param_default(KnobsID::Style4),
            ),

            float_tick_marks: tick_marks::Group::subdivided(
                1,
                1,
                1,
                Some(tick_marks::Tier::Two),
            ),

            int_tick_marks: tick_marks::Group::evenly_spaced(
                6,
                tick_marks::Tier::Two,
            ),

            db_tick_marks: vec![
                (db_range.to_normal(0.0), tick_marks::Tier::One),
                (db_range.to_normal(1.0), tick_marks::Tier::Two),
                (db_range.to_normal(3.0), tick_marks::Tier::Two),
                (db_range.to_normal(6.0), tick_marks::Tier::Two),
                (db_range.to_normal(12.0), tick_marks::Tier::Two),
                (db_range.to_normal(-1.0), tick_marks::Tier::Two),
                (db_range.to_normal(-3.0), tick_marks::Tier::Two),
                (db_range.to_normal(-6.0), tick_marks::Tier::Two),
                (db_range.to_normal(-12.0), tick_marks::Tier::Two),
            ]
            .into(),

            freq_tick_marks: vec![
                (freq_range.to_normal(20.0), tick_marks::Tier::Two),
                (freq_range.to_normal(50.0), tick_marks::Tier::Two),
                (freq_range.to_normal(100.0), tick_marks::Tier::One),
                (freq_range.to_normal(200.0), tick_marks::Tier::Two),
                (freq_range.to_normal(400.0), tick_marks::Tier::Two),
                (freq_range.to_normal(1000.0), tick_marks::Tier::One),
                (freq_range.to_normal(2000.0), tick_marks::Tier::Two),
                (freq_range.to_normal(5000.0), tick_marks::Tier::Two),
                (freq_range.to_normal(10000.0), tick_marks::Tier::One),
                (freq_range.to_normal(20000.0), tick_marks::Tier::Two),
            ]
            .into(),

            float_text_marks: text_marks::Group::min_max_and_center(
                "-1", "+1", "0",
            ),
            int_text_marks: text_marks::Group::evenly_spaced(&[
                "A", "B", "C", "D", "E", "F",
            ]),
            db_text_marks: text_marks::Group::min_max_and_center(
                "-12", "+12", "0",
            ),
            freq_text_marks: vec![
                (freq_range.to_normal(100.0), "100"),
                (freq_range.to_normal(1000.0), "1k"),
                (freq_range.to_normal(10000.0), "10k"),
            ]
            .into(),

            output_text: String::from("Move a widget"),
        }
    }
}

impl KnobStep {
    pub fn title(&self) -> &str {
        "Knobs"
    }

    pub fn update(&mut self, message: Message) {
        match message {
            Message::KnobMoved(id) => {
                // Update the output text with the new value of the parameter.
                match id {
                    KnobsID::Float => {
                        self.output_text = crate::info_text_f32(
                            id,
                            self.float_range
                                .to_value(self.knob_float_state.param.normal),
                        );
                    }
                    KnobsID::Int => {
                        // Integer parameters must be snapped for the widget to
                        // "step" when moved.
                        self.int_range
                            .snap_normal(&mut self.knob_int_state.param.normal);

                        self.output_text = crate::info_text_i32(
                            id,
                            self.int_range
                                .to_value(self.knob_int_state.param.normal),
                        );
                    }
                    KnobsID::DB => {
                        self.output_text = crate::info_text_db(
                            id,
                            self.db_range
                                .to_value(self.knob_db_state.param.normal),
                        );
                    }
                    KnobsID::Freq => {
                        self.output_text = crate::info_text_freq(
                            id,
                            self.freq_range
                                .to_value(self.knob_freq_state.param.normal),
                        );
                    }
                    KnobsID::Style1 => {
                        self.output_text = crate::info_text_f32(
                            id,
                            self.float_range
                                .to_value(self.knob_style1_state.param.normal),
                        );
                    }
                    KnobsID::Style2 => {
                        self.output_text = crate::info_text_f32(
                            id,
                            self.float_range
                                .to_value(self.knob_style2_state.param.normal),
                        );
                    }
                    KnobsID::Style3 => {
                        self.output_text = crate::info_text_f32(
                            id,
                            self.float_range
                                .to_value(self.knob_style3_state.param.normal),
                        );
                    }
                    KnobsID::Style4 => {
                        self.output_text = crate::info_text_f32(
                            id,
                            self.float_range
                                .to_value(self.knob_style4_state.param.normal),
                        );
                    }
                }
            }
        }
    }

    pub fn view(&mut self, _debug: bool) -> Element<Message> {
        // create each of the Knob widgets, passing in the value of
        // the corresponding parameter

        let knob_float =
            Knob::new(&mut self.knob_float_state, Message::KnobMoved)
                .tick_marks(&self.float_tick_marks)
                .text_marks(&self.float_text_marks);

        let knob_int = Knob::new(&mut self.knob_int_state, Message::KnobMoved)
            .tick_marks(&self.int_tick_marks)
            .text_marks(&self.int_text_marks);

        let knob_db = Knob::new(&mut self.knob_db_state, Message::KnobMoved)
            .tick_marks(&self.db_tick_marks)
            .text_marks(&self.db_text_marks);

        let knob_freq =
            Knob::new(&mut self.knob_freq_state, Message::KnobMoved)
                .tick_marks(&self.freq_tick_marks)
                .text_marks(&self.freq_text_marks);

        let knob_style1 =
            Knob::new(&mut self.knob_style1_state, Message::KnobMoved)
                .style(style::KnobCustomStyleCircle)
                .text_marks(&self.float_text_marks);

        let knob_style2 =
            Knob::new(&mut self.knob_style2_state, Message::KnobMoved)
                .style(style::KnobCustomStyleLine);

        let knob_style3 =
            Knob::new(&mut self.knob_style3_state, Message::KnobMoved)
                .style(style::KnobCustomArc);

        let knob_style4 =
            Knob::new(&mut self.knob_style4_state, Message::KnobMoved)
                .style(style::KnobCustomArcBipolar);

        // push the widgets into rows
        let knob_row = Row::new()
            .spacing(20)
            .push(
                Column::new()
                    .width(Length::Fill)
                    .spacing(20)
                    .push(Text::new("Float Range"))
                    .push(knob_float)
                    .push(Text::new("Log DB Range"))
                    .push(knob_db)
                    .push(Text::new("Custom Style 1"))
                    .push(knob_style1),
            )
            .push(
                Column::new()
                    .width(Length::Fill)
                    .spacing(20)
                    .push(Text::new("Int Range"))
                    .push(knob_int)
                    .push(Text::new("Freq Range"))
                    .push(knob_freq)
                    .push(Text::new("Custom Style 2"))
                    .push(knob_style2),
            )
            .push(
                Column::new()
                    .width(Length::Fill)
                    .spacing(20)
                    .push(Text::new("Custom Style 3"))
                    .push(knob_style3)
                    .push(Text::new("Custom Bipolar Style 4"))
                    .push(knob_style4),
            );

        let content = Column::new()
            .spacing(20)
            .padding(20)
            .push(knob_row)
            .push(Text::new(&self.output_text).size(16));

        Step::container("Knobs").push(content).into()
    }
}
