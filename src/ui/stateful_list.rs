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

    /* TODO: Implement Ctrl+D & Ctrl+U for scrolling */

    /* pub fn leap_forward(&mut self) {
        let next_index = match self.state.selected() {
            Some(i) => match i {
                i if i >= self.items.len() - 7 => self.items.len() - 1,
                _ => i + 6,
            },
            None => 0,
        };

        self.state.select(Some(next_index));
    }

    pub fn leap_backward(&mut self) {
        let prev_index = match self.state.selected() {
            Some(i) => match i {
                0..=6 => 0,
                _ => i - 6,
            },
            None => 0,
        };
        self.state.select(Some(prev_index));
    } */

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

    pub fn last(&mut self) {
        self.state.select(Some(self.items.len() - 1));
    }

    pub fn first(&mut self) {
        self.state.select(Some(0));
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
