#[allow(unused_imports)]
use color_eyre::{config::HookBuilder,Result};
use std::{clone, default, io::{self,stdout}};
use crossterm::{
    event::{self,Event,KeyCode,KeyEventKind},
terminal::{disable_raw_mode,enable_raw_mode,EnterAlternateScreen,LeaveAlternateScreen},
ExecutableCommand};
use ratatui::{prelude::*,style::palette::tailwind,widgets::*};
use strum::{EnumIter,Display,FromRepr,IntoEnumIterator};
#[derive(Default,PartialEq,Eq,Clone,Copy)]
enum Appstate{
    #[default]
    Running,
    Quitting,
}
#[derive(Default)]
struct App{
    state:Appstate,
    selected_tab:SelectedTab,
}

#[derive(Default,Clone, Copy,PartialEq, Eq,EnumIter)]
enum SelectedTab {
    #[default]
    #[strum(to_string="Tab 1")]
    Tab1,
    #[strum(to_string="Tab 2")]
    Tab2,
    #[strum(to_string="Tab 3")]
    Tab3,
    #[strum(to_string="Tab 4")]
    Tab4,
} 


fn main()-> Result<()> {
    init_error_hooks()?;
    let mut terminal=init_terminal()?;
    App::default().run(&mut terminal)?;
    restore_terminal()?;
    ok(())
}
impl App{
    fn run (&mut self , terminal:&mut Terminal<impl Backend>) -> Result<()>{
        while self.state==Appstate::Running{
            self.draw(terminal)?;
            self.handle_events()?;
        }
        ok(())
    }

    fn draw(&self,terminal:&mut Terminal<impl Backend>)->Result<()>{
        terminal.draw(|frame|frame.render_widget(self,frame.size()))?;
        ok(())
    }
    fn handle_events(&mut self)->std::io::Result<()>{
        todo!();
    }
    pub fn next_tab(&mut self){
        self.selected_tab=self.selected_tab.next();
    }

    pub fn previous_tab(&mut self){
    self.selected_tab=self.selected_tab.previous();
    }
    pub fn quit(&mut self){
        self.state=Appstate::Quitting;
    }

}
impl SelectedTab{
    fn next(self)->Self{
        todo!();
    }
    fn previous(self)->Self{
        todo!();
    }
}
