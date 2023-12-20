mod stateful_list;

use anyhow::{anyhow, Result};
use stateful_list::StatefulList;

use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

use ratatui::{
    layout::Constraint,
    prelude::*,
    style::Modifier,
    widgets::{Block, Borders, List, ListItem, Padding, Paragraph},
    Frame,
};

use std::{io::stdout, time::Duration};

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
    config: String,
}

impl Ui {
    pub fn new(instance_set: InstanceSet, config: String) -> Result<Self> {
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

        Ok(Self { list, config })
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
                Self::ui(frame, &mut self.list, self.config.clone()).unwrap();
            })?;

            /* Handle Events */
            match self.read_events()? {
                BlazeUiEvent::Quit => {
                    self.list.state.select(None);
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
                BlazeUiEvent::Selected => {
                    disable_raw_mode()?;
                    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
                    break;
                }
            }
        }

        match self.list.state.selected() {
            Some(index) => Ok(self.list.items[index].1.clone()),
            None => Err(anyhow!("No instance selected")),
        }
    }

    fn ui(
        frame: &mut Frame,
        list: &mut StatefulList<(String, InstanceDetails)>,
        config: String,
    ) -> Result<()> {
        let slices = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(70), Constraint::Percentage(30)])
            .split(frame.size());

        let dashboard_slices = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(slices[1]);

        /* render keybindings */
        let keybindings_list_items = vec![
            ListItem::new("j/Down: Next item"),
            ListItem::new("k/Up: Previous item"),
            ListItem::new("Enter: Select item"),
            ListItem::new("q/Esc: Quit"),
        ];

        let keybindings_list = List::new(keybindings_list_items).block(
            Block::default()
                .borders(Borders::ALL)
                .bg(Color::Black)
                .fg(Color::White)
                .padding(Padding::new(4, 1, 1, 1))
                .title("Keybindings"),
        );

        frame.render_widget(keybindings_list, dashboard_slices[0]);

        /* render raw config */
        let config = Paragraph::new(config).block(
            Block::default()
                .borders(Borders::ALL)
                .bg(Color::Black)
                .fg(Color::White)
                .title("Config")
                .padding(Padding::new(4, 1, 1, 1)),
        );
        frame.render_widget(config, dashboard_slices[1]);

        /* render instances list */
        let prepared_items: Vec<ListItem> = list
            .items
            .iter()
            .map(|(dsp_name, _item)| {
                ListItem::new(dsp_name.clone()).style(
                    Style::default()
                        .fg(ratatui::style::Color::White)
                        .bg(ratatui::style::Color::Black),
                )
            })
            .collect();

        let prepared_list = List::new(prepared_items)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .bg(Color::Black)
                    .padding(Padding::new(4, 4, 1, 1))
                    .title("Instances"),
            )
            .highlight_style(
                Style::default()
                    .bg(ratatui::style::Color::Green)
                    .fg(ratatui::style::Color::Black)
                    .add_modifier(Modifier::BOLD),
            )
            .highlight_symbol(">>= ");

        frame.render_stateful_widget(prepared_list, slices[0], &mut list.state);

        Ok(())
    }

    fn read_events(&self) -> Result<BlazeUiEvent> {
        match event::poll(Duration::from_millis(250))? {
            true => match event::read()? {
                Event::Key(event) => match event.kind {
                    KeyEventKind::Press => match event.code {
                        KeyCode::Char('q') | KeyCode::Esc => Ok(BlazeUiEvent::Quit),
                        KeyCode::Char('j') | KeyCode::Down => Ok(BlazeUiEvent::ListNext),
                        KeyCode::Char('k') | KeyCode::Up => Ok(BlazeUiEvent::ListPrevious),
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
