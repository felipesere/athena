use search::Search;
use ansi::Ansi;
use tty::TTY;
use fake_tty::FakeIO;
use renderer::Renderer;
use text::Text;

pub struct Screen <'a> {
    pub ansi: Ansi<'a>,
    height: usize,
}

impl <'a> Screen <'a>{
    pub fn new() -> Screen<'a> {
        let ansi = Ansi { io: Box::new(TTY::new()) };
        let (_, height) = ansi.io.dimensions();
        Screen {
            ansi: ansi,
            height: height,
        }
    }

    pub fn fake() -> Screen<'a> {
        Screen {
            ansi: Ansi { io: Box::new(FakeIO::new()) },
            height: 20,
        }
    }

    pub fn handle_keystroke(&self, search: Search<'a>, input: &str) -> Search<'a> {
        match input {
           "\u{e}" => search.down(),
           "\u{10}" => search.up(),
           "\u{7f}" => search.backspace(),
           "\n" => search.done(),
            _ => search.append_to_search(input),
        }
    }

    pub fn print(&mut self, search: &Search) {
        let renderer = Renderer;
        let result = renderer.render(search);
        self.ansi.hide_cursor();

        let start_line = self.height - search.config.visible_limit - 2;

        for (idx, text) in result.iter().enumerate() {
            self.write(start_line + idx, text);
        };
        self.ansi.set_position(start_line - 1, renderer.header(search).len());
        self.ansi.show_cursor();
    }

    pub fn write(&mut self, line: usize, text: &Text) {
        self.ansi.blank_line(line);
        self.ansi.set_position(line, 0);

        match *text {
            Text::Normal(ref t) => self.ansi.print(t.as_slice()),
            Text::Highlight(ref t) => self.ansi.inverted(t.as_slice()),
            Text::Blank => self.ansi.print("".as_slice()),
        };
    }

    pub fn move_cursor_to_end(&mut self) {
        self.ansi.set_position(self.height - 1, 0);
    }
}

#[cfg(test)]
mod tests {
    use configuration::Configuration;
    use search::Search;
    use super::*;

    #[test]
    fn moves_the_selection_down_for_ctrl_n() {
        let input = input();
        let config = Configuration::from_inputs(&input, None, Some(10));
        let search = Search::blank(&config);
        let screen = Screen::fake();
        let result = screen.handle_keystroke(search, "\u{e}");
        assert_eq!(result.selection, Some("two".to_string()));
    }

    #[test]
    fn moves_the_selection_up_for_ctrl_p() {
        let input = input();
        let config = Configuration::from_inputs(&input, None, Some(10));
        let search = Search::blank(&config).down();
        let screen = Screen::fake();
        let result = screen.handle_keystroke(search, "\u{10}");
        assert_eq!(result.selection, Some("one".to_string()));
    }

    #[test]
    fn removes_the_last_character_for_delete() {
        let input = input();
        let config = Configuration::from_inputs(&input, None, Some(10));
        let search = Search::blank(&config).append_to_search("w").append_to_search("x");
        let screen = Screen::fake();
        let result = screen.handle_keystroke(search, "\u{7f}");
        assert_eq!(result.selection, Some("two".to_string()));
    }

    #[test]
    fn marks_a_search_as_done_for_enter() {
        let input = input();
        let config = Configuration::from_inputs(&input, None, Some(10));
        let search = Search::blank(&config);
        let screen = Screen::fake();
        let result = screen.handle_keystroke(search, "\n");
        assert!(result.is_done());
    }

    fn input() -> Vec<String> {
        vec!["one".to_string(), "two".to_string()]
    }
}
