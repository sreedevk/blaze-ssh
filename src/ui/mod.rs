mod stateful_list;

use stateful_list::StatefulList;

use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

use ratatui::{
    prelude::{CrosstermBackend, Style, Terminal},
    style::Modifier,
    widgets::{Block, Borders, List, ListItem},
    Frame,
};

use std::{
    io::{stdout, Result},
    time::Duration,
};

use crate::instance_details::{InstanceDetails, InstanceSet};

#[derive(Debug, Clone)]
enum BlazeUiEvent {
    Quit,
    Noop,
    ListNext,
    ListPrevious,
    Selected,
}

#[derive(Debug, Clone)]
pub struct Ui {
    list: StatefulList<(String, InstanceDetails)>,
}

impl Ui {
    pub fn new(instance_set: InstanceSet) -> Result<Self> {
        let list_elements = instance_set
            .instances
            .iter()
            .map(|instance| {
                (
                    instance.display_name().unwrap_or_default(),
                    instance.clone(),
                )
            })
            .collect::<Vec<_>>();

        let list = StatefulList::with_items(list_elements);

        Ok(Self { list })
    }

    pub fn run(&mut self) -> Result<InstanceDetails> {
        /* terminal setup */
        enable_raw_mode()?;
        let mut stdout = stdout();
        execute!(stdout, EnterAlternateScreen)?;
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;

        /* create app and run it */
        loop {
            /* Render UI */
            terminal.draw(|frame| {
                Self::ui(frame, &mut self.list).unwrap();
            })?;

            /* Handle Events */
            match self.read_events()? {
                BlazeUiEvent::Quit | BlazeUiEvent::Selected => {
                    disable_raw_mode()?;
                    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
                    break;
                }
                BlazeUiEvent::Noop => {
                    continue;
                }
                BlazeUiEvent::ListNext => {
                    self.list.next();
                }
                BlazeUiEvent::ListPrevious => {
                    self.list.previous();
                }
            }
        }

        Ok(self.list.items[self.list.state.selected().unwrap()]
            .1
            .clone())
    }

    fn ui(frame: &mut Frame, list: &mut StatefulList<(String, InstanceDetails)>) -> Result<()> {
        let prepared_items: Vec<ListItem> = list
            .items
            .iter()
            .map(|(dsp_name, _item)| {
                ListItem::new(dsp_name.clone()).style(
                    Style::default()
                        .fg(ratatui::style::Color::Black)
                        .bg(ratatui::style::Color::White),
                )
            })
            .collect();

        let prepared_list = List::new(prepared_items)
            .block(Block::default().borders(Borders::ALL).title("Instances"))
            .highlight_style(
                Style::default()
                    .bg(ratatui::style::Color::LightBlue)
                    .add_modifier(Modifier::BOLD),
            )
            .highlight_symbol(">> ");

        frame.render_stateful_widget(prepared_list, frame.size(), &mut list.state);

        Ok(())
    }

    fn read_events(&self) -> Result<BlazeUiEvent> {
        match event::poll(Duration::from_millis(250))? {
            true => match event::read()? {
                Event::Key(event) => match event.kind {
                    KeyEventKind::Press => match event.code {
                        KeyCode::Char('q') | KeyCode::Esc => Ok(BlazeUiEvent::Quit),
                        KeyCode::Char('j') | KeyCode::Up => Ok(BlazeUiEvent::ListNext),
                        KeyCode::Char('k') | KeyCode::Down => Ok(BlazeUiEvent::ListPrevious),
                        KeyCode::Enter => Ok(BlazeUiEvent::Selected),
                        _ => Ok(BlazeUiEvent::Noop),
                    },
                    _ => Ok(BlazeUiEvent::Noop),
                },
                _ => Ok(BlazeUiEvent::Noop),
            },
            false => Ok(BlazeUiEvent::Noop),
        }
    }
}
