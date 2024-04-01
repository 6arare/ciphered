#![allow(clippy::wildcard_imports, clippy::enum_glob_use)]

use std::io::stdout;
use KeyCode::*;
use color_eyre::{config::HookBuilder, Result};
use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::{prelude::*, style::palette::tailwind, widgets::*};
use strum::{Display, EnumIter, FromRepr, IntoEnumIterator};

#[derive(Default)]
struct App {
    state: AppState,
    selected_tab: SelectedTab,
    tab_state:TabState,
}

#[derive(Default, Clone, Copy, PartialEq, Eq)]
enum AppState {
    #[default]
    Running,
    Quitting,
}

#[derive(Default, Clone, Copy, PartialEq, Eq)]
enum TabState {
    #[default]
    Selected,
    NotSelected,
}

#[derive(Default, Clone, Copy, Display, FromRepr, EnumIter)]
enum SelectedTab {
    #[default]
    #[strum(to_string = "base64")]
    tab1,
    #[strum(to_string = "Hex")]
    Tab2,
    #[strum(to_string = "XOR")]
    Tab3,
    #[strum(to_string = "MD5")]
    Tab4,
    #[strum(to_string = "tab5")]
    Tab5,
    #[strum(to_string = "tab6")]
    Tab6,
}

pub fn init_ui() -> Result<()> {
    init_error_hooks()?;
    let mut terminal = init_terminal()?;
    App::default().run(&mut terminal)?;
    restore_terminal()?;
    Ok(())
}

impl App {
    fn run(&mut self, terminal: &mut Terminal<impl Backend>) -> Result<()> {
        while self.state == AppState::Running {
            self.draw(terminal)?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn draw(&self, terminal: &mut Terminal<impl Backend>) -> Result<()> {
        terminal.draw(|frame| frame.render_widget(self, frame.size()))?;
        Ok(())
    }

    fn handle_events(&mut self) -> std::io::Result<()> {
        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press {
                match key.code {
                    Char('l') | Right => self.next_tab(),
                    Char('h') | Left => self.previous_tab(),
                    Char('q') | Esc => self.quit(),
                    Char('k') | Up => self.unselect_tab(),
                    Char('j') | Down => self.select_tab(),
                    _ => {}
                }
            }
        }
        Ok(())
    }

    pub fn next_tab(&mut self) {
        self.selected_tab = self.selected_tab.next();
    }

    pub fn previous_tab(&mut self) {
        self.selected_tab = self.selected_tab.previous();
    }

    pub fn quit(&mut self) {
        self.state = AppState::Quitting;
    }
    pub fn select_tab(&mut self){
        self.tab_state=TabState::Selected;
    }
    pub fn unselect_tab(&mut self){
        self.tab_state=TabState::NotSelected;
    }
}

impl SelectedTab {
    /// Get the previous tab, if there is no previous tab return the current tab.
    fn previous(self) -> Self {
        let current_index: usize = self as usize;
        let previous_index = current_index.saturating_sub(1);
        Self::from_repr(previous_index).unwrap_or(self)
    }

    /// Get the next tab, if there is no next tab return the current tab.
    fn next(self) -> Self {
        let current_index = self as usize;
        let next_index = current_index.saturating_add(1);
        Self::from_repr(next_index).unwrap_or(self)
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        use Constraint::*;
        let vertical = Layout::vertical([Length(1), Min(0), Length(1)]);
        let [header_area, inner_area, footer_area] = vertical.areas(area);

        let horizontal = Layout::horizontal([Min(0), Length(20)]);
        let [tabs_area, title_area] = horizontal.areas(header_area);

        render_title(title_area, buf);
        self.render_tabs(tabs_area, buf);
        self.selected_tab.render(inner_area, buf);
        render_footer(footer_area, buf);
    }
}

impl App {
    fn render_tabs(&self, area: Rect, buf: &mut Buffer) {
        let titles = SelectedTab::iter().map(SelectedTab::title);
        let highlight_style = (Color::default(), self.selected_tab.palette().c700);
        let selected_tab_index = self.selected_tab as usize;
        Tabs::new(titles)
            .highlight_style(highlight_style)
            .select(selected_tab_index)
            .padding("", "")
            .divider(" ")
            .render(area, buf);
    }
}

fn render_title(area: Rect, buf: &mut Buffer) {
    "Ciphered".bold().render(area, buf);
}

fn render_footer(area: Rect, buf: &mut Buffer) {
    Line::raw("◄ ► to change tab | Press q to quit")
        .centered()
        .render(area, buf);
}

impl Widget for SelectedTab {
    fn render(self, area: Rect, buf: &mut Buffer) {
        // in a real app these might be separate widgets
        match self {
            Self::tab1 => self.render_tab0(area, buf),
            Self::Tab2 => self.render_tab1(area, buf),
            Self::Tab3 => self.render_tab2(area, buf),
            Self::Tab4 => self.render_tab3(area, buf),
            Self::Tab5 => self.render_tab3(area, buf),
            Self::Tab6 => self.render_tab3(area, buf),
        }
    }
}
impl SelectedTab {
    /// Return tab's name as a styled `Line`
    fn title(self) -> Line<'static> {
        format!("  {self}  ")
        .fg(tailwind::SLATE.c200)
            .bg(self.palette().c900)
            .into()
        }
        
        fn render_tab0(self, area: Rect, buf: &mut Buffer) {
            Paragraph::new("base 64")
            .block(self.block())
            .render(area, buf);
    }

    fn render_tab1(self, area: Rect, buf: &mut Buffer) {
        Paragraph::new("Welcome to the Ratatui tabs example!")
        .block(self.block())
            .render(area, buf);
    }

    fn render_tab2(self, area: Rect, buf: &mut Buffer) {
        Paragraph::new("Look! I'm different than others!")
        .block(self.block())
        .render(area, buf);
}

fn render_tab3(self, area: Rect, buf: &mut Buffer) {
    Paragraph::new("I know, these are some basic changes. But I think you got the main idea.")
    .block(self.block())
    .render(area, buf);
}

/// A block surrounding the tab's content
fn block(self) -> Block<'static> {
    Block::default()
    .borders(Borders::ALL)
    .border_set(symbols::border::PROPORTIONAL_TALL)
    .padding(Padding::horizontal(1))
    .border_style(self.palette().c700)
}

const fn palette(self) -> tailwind::Palette {
    match self {
        Self::tab1 => tailwind::BLUE,
        Self::Tab2 => tailwind::EMERALD,
        Self::Tab3 => tailwind::INDIGO,
        Self::Tab4 => tailwind::RED,
        Self::Tab5 => tailwind::RED,
        Self::Tab6 => tailwind::RED,
    }
}
}

fn init_error_hooks() -> color_eyre::Result<()> {
    let (panic, error) = HookBuilder::default().into_hooks();
    let panic = panic.into_panic_hook();
    let error = error.into_eyre_hook();
    color_eyre::eyre::set_hook(Box::new(move |e| {
        let _ = restore_terminal();
        error(e)
    }))?;
    std::panic::set_hook(Box::new(move |info| {
        let _ = restore_terminal();
        panic(info);
    }));
    Ok(())
}

fn init_terminal() -> color_eyre::Result<Terminal<impl Backend>> {
    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout());
    let terminal = Terminal::new(backend)?;
    Ok(terminal)
}

fn restore_terminal() -> color_eyre::Result<()> {
    disable_raw_mode()?;
    stdout().execute(LeaveAlternateScreen)?;
    Ok(())
}

