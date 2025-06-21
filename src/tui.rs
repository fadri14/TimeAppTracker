/*
Ajouter les paramètres
Montrer une liste avec scroll au lieu de devoir taper les noms d'appli
*/
use chrono::{Datelike, Duration, NaiveDate, Utc};
use crossterm::event::{self, Event, KeyCode, KeyEvent};
use ratatui::{
    DefaultTerminal, Frame,
    layout::{Alignment, Constraint, Flex, Layout, Rect},
    style::{Color, Modifier},
    text::Line,
    widgets::{Bar, BarChart, BarGroup, Block, BorderType, Borders, Clear, Paragraph},
};
use std::{io, str::FromStr};
use tui_input::{Input, backend::crossterm::EventHandler};

use crate::database::{Database, Stat, TimeApp};

pub fn start(database: Database, color: Color) -> io::Result<()> {
    let mut terminal = ratatui::init();
    let app_result = App::new(database, color).run(&mut terminal);
    ratatui::restore();
    app_result
}

struct App {
    database: Database,
    mode: Mode,
    option_day: OptionDay,
    option_app: OptionApp,
    show_input: TypeInput,
    input: Input,
    show_help: bool,
    color: Color,
    exit: bool,
}

impl App {
    fn new(database: Database, color: Color) -> App {
        App {
            database,
            mode: Mode::Day,
            option_day: OptionDay::new(),
            option_app: OptionApp::new(),
            show_input: TypeInput::None,
            input: Input::new(String::new()),
            show_help: false,
            color,
            exit: false,
        }
    }

    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.render(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn render(&mut self, frame: &mut Frame) {
        let title = Line::styled(" Time App Tracker ", Modifier::BOLD);
        let keymaps = Line::styled(" <?> Help | <Q> Quit ", (self.color, Modifier::BOLD));

        let block = Block::bordered()
            .title(title)
            .title_bottom(keymaps)
            .title_alignment(Alignment::Center)
            .border_type(BorderType::Rounded);
        let vertical = Layout::vertical([
            Constraint::Percentage(9),
            Constraint::Percentage(9),
            Constraint::Percentage(82),
        ]);
        let [options, stats, chart] = vertical.areas(block.inner(frame.area()));

        frame.render_widget(block, frame.area());

        match self.mode {
            Mode::Day => self.option_day.render(frame, options),
            Mode::App => self.option_app.render(frame, options),
        };

        if self.mode == Mode::App {
            self.stats(frame, stats);
        }

        self.chart(frame, chart);

        if self.show_input != TypeInput::None {
            App::display_input(frame, self.input.value());
        }

        if self.show_help {
            App::display_help(frame);
        }
    }

    fn handle_events(&mut self) -> io::Result<()> {
        let event = event::read()?;
        if let Event::Key(key) = event {
            match self.show_input {
                TypeInput::None => self.handle_key_event(key),
                _ => match key.code {
                    KeyCode::Enter => self.valide_input(),
                    KeyCode::Esc => self.cancel_input(),
                    _ => {
                        self.input.handle_event(&event);
                    }
                },
            }
        }
        Ok(())
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        if self.show_help {
            match key_event.code {
                KeyCode::Char('q') | KeyCode::Esc => self.show_help = false,
                _ => {}
            }
            return;
        }

        match self.mode {
            Mode::Day => match key_event.code {
                KeyCode::Char('q') | KeyCode::Esc => self.exit(),
                KeyCode::Char('m') => self.mode = Mode::App,
                KeyCode::Char('r') => self.option_day.switch_reverse(),
                KeyCode::Char('d') => {
                    self.show_input = TypeInput::Date;
                    self.input = Input::new(self.option_day.date.to_string());
                }
                KeyCode::Char('t') => self.option_day.date = Utc::now().date_naive(),
                KeyCode::Up | KeyCode::Char('k') => self.option_day.date += Duration::days(1),
                KeyCode::Down | KeyCode::Char('j') => self.option_day.date -= Duration::days(1),
                KeyCode::Right | KeyCode::Char('l') => self.option_day.index_bar += 1,
                KeyCode::Left | KeyCode::Char('h') if self.option_day.index_bar > 0 => {
                    self.option_day.index_bar -= 1
                }
                KeyCode::Char('?') => {
                    self.show_input = TypeInput::None;
                    self.show_help = true;
                }
                _ => {}
            },
            Mode::App => match key_event.code {
                KeyCode::Char('q') | KeyCode::Esc => self.exit(),
                KeyCode::Char('m') => self.mode = Mode::Day,
                KeyCode::Char('r') => self.option_app.switch_reverse(),
                KeyCode::Char('d') => {
                    self.show_input = TypeInput::Date;
                    self.input = Input::new(self.option_app.date.to_string());
                }
                KeyCode::Char('n') => self.show_input = TypeInput::Number,
                KeyCode::Char('a') => self.show_input = TypeInput::App,

                KeyCode::Char('t') => self.option_app.date = Utc::now().date_naive(),
                KeyCode::Up | KeyCode::Char('k') => self.option_app.date += Duration::days(1),
                KeyCode::Down | KeyCode::Char('j') => self.option_app.date -= Duration::days(1),
                KeyCode::Right | KeyCode::Char('l') => self.option_app.index_bar += 1,
                KeyCode::Left | KeyCode::Char('h') if self.option_app.index_bar > 0 => {
                    self.option_app.index_bar -= 1
                }
                KeyCode::Char('?') => {
                    self.show_input = TypeInput::None;
                    self.show_help = true;
                }
                _ => {}
            },
        }
    }

    fn exit(&mut self) {
        self.exit = true;
    }

    fn chart(&mut self, frame: &mut Frame, area: Rect) {
        let mut data = self.get_data();
        let mut bars: Vec<Bar> = Vec::new();

        let max_value = data.iter().max_by_key(|d| d.min_total).map(|d| d.min_total);
        match self.mode {
            Mode::Day => data.sort_by_key(|d| d.min_total),
            Mode::App => data.sort_by_key(|d| d.date),
        }
        if !self.is_reverse() {
            data.reverse();
        }

        for d in data {
            bars.push(
                Bar::default()
                    .label(match self.mode {
                        Mode::Day => d.name.into(),
                        Mode::App => d.date.to_string()[5..].to_string().into(),
                    })
                    .value(d.min_total as u64)
                    .text_value(d.time.to_string()),
            );
        }

        let horizontal = Layout::horizontal([
            Constraint::Length(2),
            Constraint::Fill(1),
            Constraint::Length(2),
        ]);
        let [left, area, right] = horizontal.areas(area);

        let mut width = area.width;
        let (index_start, index_end) = self.slice_bars(bars.len(), &mut width);

        if index_start > 0 {
            App::display_arrow(frame, left, "<=");
        }

        if index_end < bars.len() - 1 {
            App::display_arrow(frame, right, "=>");
        }

        let chart = BarChart::default()
            .data(BarGroup::default().bars(&bars[index_start..=index_end]))
            .max(max_value.unwrap_or(60 * 24) as u64)
            .block(
                Block::bordered()
                    .title("Chart")
                    .border_type(BorderType::Rounded),
            )
            .bar_width(width)
            .bar_style(self.color)
            .label_style(self.color);
        frame.render_widget(chart, area);
    }

    fn slice_bars(&mut self, number_elements: usize, widths: &mut u16) -> (usize, usize) {
        let index_start = match self.mode {
            Mode::Day => &mut self.option_day.index_bar,
            Mode::App => &mut self.option_app.index_bar,
        };

        let mut width = *widths / (number_elements + 1) as u16;
        if width < 5 {
            width = 5;
        }

        let number_display = *widths / (width + 1);

        *widths = width;

        let mut index_end = *index_start + (number_display as usize) - 1;

        if index_end == number_elements {
            *index_start -= 1;
            index_end -= 1;
        }
        (*index_start, index_end)
    }

    fn display_arrow(frame: &mut Frame, area: Rect, arrow: &str) {
        let vertical = Layout::vertical([
            Constraint::Length(1),
            Constraint::Fill(1),
            Constraint::Length(1),
            Constraint::Fill(1),
            Constraint::Length(1),
        ]);
        let [arrow1, _, arrow2, _, arrow3] = vertical.areas(area);
        let arrow = Line::from(arrow).centered();
        frame.render_widget(&arrow, arrow1);
        frame.render_widget(&arrow, arrow2);
        frame.render_widget(&arrow, arrow3);
    }

    fn stats(&self, frame: &mut Frame, area: Rect) {
        let horizontal = Layout::horizontal([
            Constraint::Length(2),
            Constraint::Fill(1),
            Constraint::Length(2),
        ]);
        let [_, area, _] = horizontal.areas(area);

        let block = Block::bordered()
            .title_top("Stats")
            .border_type(BorderType::Rounded);
        let horizontal = Layout::horizontal([
            Constraint::Percentage(30),
            Constraint::Percentage(10),
            Constraint::Percentage(10),
            Constraint::Percentage(10),
            Constraint::Percentage(10),
            Constraint::Percentage(30),
        ]);
        let [_, max, min, sum, mean, _] = horizontal.areas(block.inner(area));

        let stat = Stat::new(&self.get_data());
        frame.render_widget(block, area);
        frame.render_widget(Line::from(format!(" Max : {} ", stat.max)).centered(), max);
        frame.render_widget(Line::from(format!(" Min : {} ", stat.min)).centered(), min);
        frame.render_widget(Line::from(format!(" Sum : {} ", stat.sum)).centered(), sum);
        frame.render_widget(
            Line::from(format!(" Mean : {} ", stat.mean)).centered(),
            mean,
        );
    }

    fn get_data(&self) -> Vec<TimeApp> {
        match &self.mode {
            Mode::Day => self.database.get_time_day(self.option_day.date).unwrap(),
            Mode::App => self
                .database
                .get_time_app(
                    &self.option_app.app,
                    self.option_app.date,
                    self.option_app.number,
                )
                .unwrap(),
        }
    }

    fn is_reverse(&self) -> bool {
        match self.mode {
            Mode::Day => self.option_day.reverse,
            Mode::App => self.option_app.reverse,
        }
    }

    fn valide_input(&mut self) {
        match self.mode {
            Mode::Day => {
                if self.show_input == TypeInput::Date {
                    self.option_day.date =
                        NaiveDate::parse_from_str(&self.input.value_and_reset(), "%Y-%m-%d")
                            .unwrap_or(self.option_day.date)
                }
            }
            Mode::App => match self.show_input {
                TypeInput::Date => {
                    self.option_app.date =
                        NaiveDate::parse_from_str(&self.input.value_and_reset(), "%Y-%m-%d")
                            .unwrap_or(self.option_app.date)
                }
                TypeInput::Number => {
                    self.option_app.number = self
                        .input
                        .value_and_reset()
                        .parse::<u16>()
                        .unwrap_or(self.option_app.number)
                }
                TypeInput::App => {
                    if self
                        .database
                        .contain_names(
                            &String::from_str(self.input.value()).unwrap_or(String::new()),
                        )
                        .unwrap_or(false)
                    {
                        self.option_app.app = self.input.value_and_reset()
                    }
                }
                _ => (),
            },
        }
        self.input.reset();
        self.show_input = TypeInput::None;
    }

    fn cancel_input(&mut self) {
        self.show_input = TypeInput::None;
        self.input.reset();
    }

    fn popup_area(area: Rect, percent_x: u16, percent_y: u16) -> Rect {
        let vertical = Layout::vertical([Constraint::Percentage(percent_y)]).flex(Flex::Center);
        let horizontal = Layout::horizontal([Constraint::Percentage(percent_x)]).flex(Flex::Center);
        let [area] = vertical.areas(area);
        let [area] = horizontal.areas(area);
        area
    }

    fn display_input(frame: &mut Frame, initial_value: &str) {
        let input = Paragraph::new(initial_value)
            .block(
                Block::bordered()
                    .title("Input")
                    .border_type(BorderType::Rounded),
            )
            .centered();
        let area = App::popup_area(frame.area(), 20, 10);
        frame.render_widget(Clear, area);
        frame.render_widget(input, area);
    }

    fn display_help(frame: &mut Frame) {
        let block = Block::bordered()
            .title("Help")
            .border_type(BorderType::Rounded);

        let vertical = Layout::vertical([Constraint::Length(15)]).flex(Flex::Center);
        let horizontal = Layout::horizontal([Constraint::Length(60)]).flex(Flex::Center);
        let [area] = vertical.areas(frame.area());
        let [area] = horizontal.areas(area);

        let horizontal =
            Layout::horizontal([Constraint::Percentage(25), Constraint::Percentage(75)]);
        let [key, desc] = horizontal.areas(block.inner(area));

        let text1: Vec<_> = vec![
            "<?>".into(),
            "<Q|Esc>".into(),
            "<M>".into(),
            "<A>".into(),
            "<D>".into(),
            "<N>".into(),
            "<R>".into(),
            "<Enter>".into(),
            "<T>".into(),
            "<K|Up>".into(),
            "<J|Down>".into(),
            "<L|Right>".into(),
            "<H|Left>".into(),
        ];
        let help1 = Paragraph::new(text1)
            .block(Block::new().borders(Borders::RIGHT))
            .centered();
        let text2: Vec<_> = vec![
            "Show help".into(),
            "Exit application, help, user input".into(),
            "Switch between day and application mode".into(),
            "Change the observed application".into(),
            "Change observed date".into(),
            "Change the number of days observed".into(),
            "Reverse graph display".into(),
            "Validate user input".into(),
            "Back to today's date".into(),
            "Increase date by one day".into(),
            "Reduce date by one day".into(),
            "Scroll the graph to the right".into(),
            "Scroll the graph to the left".into(),
        ];
        let help2 = Paragraph::new(text2);

        frame.render_widget(Clear, area);
        frame.render_widget(help1, key);
        frame.render_widget(help2, desc);
        frame.render_widget(block, area);
    }
}

#[derive(PartialEq)]
enum Mode {
    Day,
    App,
}

struct OptionDay {
    date: NaiveDate,
    reverse: bool,
    index_bar: usize,
}

impl OptionDay {
    fn new() -> OptionDay {
        OptionDay {
            date: Utc::now().date_naive(),
            reverse: false,
            index_bar: 0,
        }
    }

    fn render(&self, frame: &mut Frame, area: Rect) {
        let horizontal = Layout::horizontal([
            Constraint::Length(2),
            Constraint::Fill(1),
            Constraint::Length(2),
        ]);
        let [_, area, _] = horizontal.areas(area);

        let block = Block::bordered()
            .title_top("Options")
            .border_type(BorderType::Rounded);
        let horizontal = Layout::horizontal([
            Constraint::Fill(1),
            Constraint::Length(25),
            Constraint::Length(25),
            Constraint::Length(25),
            Constraint::Fill(1),
        ]);
        let [_, mode, date, is_reverse, _] = horizontal.areas(block.inner(area));

        frame.render_widget(block, area);
        frame.render_widget(Line::from(" <M>ode : Day ").centered(), mode);
        frame.render_widget(
            Line::from(format!(" <D>ate : {} {}", self.date.weekday(), self.date)).centered(),
            date,
        );
        frame.render_widget(
            Line::from(format!(" <R>everse : {} ", self.reverse)).centered(),
            is_reverse,
        );
    }

    fn switch_reverse(&mut self) {
        self.reverse = !self.reverse;
    }
}

struct OptionApp {
    app: String,
    date: NaiveDate,
    number: u16,
    reverse: bool,
    index_bar: usize,
}

impl OptionApp {
    fn new() -> OptionApp {
        OptionApp {
            app: String::from("pc"),
            date: Utc::now().date_naive(),
            number: 7,
            reverse: false,
            index_bar: 0,
        }
    }

    fn render(&self, frame: &mut Frame, area: Rect) {
        let horizontal = Layout::horizontal([
            Constraint::Length(2),
            Constraint::Fill(1),
            Constraint::Length(2),
        ]);
        let [_, area, _] = horizontal.areas(area);

        let block = Block::bordered()
            .title_top("Options")
            .border_type(BorderType::Rounded);
        let horizontal = Layout::horizontal([
            Constraint::Fill(1),
            Constraint::Length(25),
            Constraint::Length(25),
            Constraint::Length(25),
            Constraint::Length(25),
            Constraint::Length(25),
            Constraint::Fill(1),
        ]);
        let [_, mode, app, date, number, is_reverse, _] = horizontal.areas(block.inner(area));

        frame.render_widget(block, area);
        frame.render_widget(Line::from(" <M>ode : App ").centered(), mode);
        frame.render_widget(
            Line::from(format!(" <A>pp : {} ", self.app)).centered(),
            app,
        );
        frame.render_widget(
            Line::from(format!(" <D>ate : {} {}", self.date.weekday(), self.date)).centered(),
            date,
        );
        frame.render_widget(
            Line::from(format!(" <N>umber : {} ", self.number)).centered(),
            number,
        );
        frame.render_widget(
            Line::from(format!(" <R>everse : {} ", self.reverse)).centered(),
            is_reverse,
        );
    }

    fn switch_reverse(&mut self) {
        self.reverse = !self.reverse;
    }
}

#[derive(PartialEq)]
enum TypeInput {
    None,
    Date,
    App,
    Number,
}
