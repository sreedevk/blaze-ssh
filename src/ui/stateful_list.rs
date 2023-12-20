use ratatui::widgets::ListState;

#[derive(Debug, Clone)]
pub struct StatefulList<T> {
    pub state: ListState,
    pub items: Vec<T>,
}

impl<T> StatefulList<T> {
    pub fn with_items(items: Vec<T>) -> Self {
        let mut list_state = ListState::default();
        list_state.select(Some(0));

        Self {
            state: list_state,
            items,
        }
    }

    pub fn next(&mut self) {
        let next_index = match self.state.selected() {
            Some(i) => match i {
                i if i >= self.items.len() - 1 => 0,
                _ => i + 1,
            },
            None => 0,
        };

        self.state.select(Some(next_index));
    }

    pub fn previous(&mut self) {
        let prev_index = match self.state.selected() {
            Some(i) => match i {
                0 => self.items.len() - 1,
                _ => i - 1,
            },
            None => 0,
        };
        self.state.select(Some(prev_index));
    }
}
