use configuration::Configuration;
use std::slice::SliceExt;
use std::cmp::Ordering;
use score::Score;

struct Search {
    config: Configuration,
    current: uint,
    query: String,
    selection: String,
    result: Vec<String>,
}

impl Search {
    fn blank(config: Configuration) -> Search {
        let query = config.initial_search.clone();

        Search::new(config, query, 0)
    }

    fn new(config: Configuration, query: String, index: uint) -> Search {
        let result = Search::filter(query.as_slice(), &config.choices.clone());

        Search { config: config,
                 current: index,
                 query: query,
                 selection: result[index].to_string(),
                 result: result }
    }

    fn new_for_index(self, index: uint) -> Search {
        Search::new(self.config, self.query, index)
    }

    fn filter(query: &str, choices: &Vec<String>) -> Vec<String> {
        let mut filtered = choices.iter().filter_map( |choice| {
            let quality = Score::score(choice.as_slice(), query);
            if quality > 0.0 {
                Some((quality, choice.to_string()))
            } else {
                None
            }
        }).collect::<Vec<(f32, String)>>();

        filtered.sort_by( |&(quality_a, _), &(quality_b, _)| {
            quality_a.partial_cmp(&quality_b).unwrap_or(Ordering::Equal).reverse()
        });

        filtered.iter().map( |&(_, ref choice)| choice.to_string() ).collect::<Vec<String>>()
    }


    fn down(self) -> Search {
        let next_index = self.next_index();
        self.new_for_index(next_index)
    }

    fn up(self) -> Search {
        let next_index = self.prev_index();
        self.new_for_index(next_index)
    }


    fn append_to_search(self, input: String) -> Search {
        let mut new_query = self.query;
        new_query.push_str(input.as_slice());
        Search::new(self.config, new_query, self.current)
    }

    fn next_index(&self) -> uint {
        let next_index = self.current + 1;

        if next_index >= self.config.visible_limit { 0 } else { next_index }
    }

    fn prev_index(&self) -> uint {
        if self.current == 0 { self.config.visible_limit - 1 } else  { self.current - 1 }
    }
}

#[cfg(test)]

#[test]
fn it_selects_the_first_choice_by_default() {
    let input =  one_two_three();

    let config = Configuration::from_inputs(input, None, None);
    let search = Search::blank(config);

    assert_eq!(search.selection, "one");
}

fn one_two_three() -> Vec<String> {
    vec!["one".to_string(),
         "two".to_string(),
         "three".to_string()]
}

#[test]
fn it_selets_the_second_when_down_is_called() {
    let input =  one_two_three();

    let config = Configuration::from_inputs(input, None, None);
    let search = Search::blank(config);

    assert_eq!(search.down().selection, "two");
}

#[test]
fn it_loop_around_when_reaching_end_of_list() {
    let input =  one_two_three();

    let config = Configuration::from_inputs(input, None, None);
    let search = Search::blank(config);

    assert_eq!(search.down().down().down().down().selection, "two");
}

#[test]
fn it_loop_around_when_reaching_top_of_list() {
    let input =  one_two_three();

    let config = Configuration::from_inputs(input, None, None);
    let search = Search::blank(config);

    assert_eq!(search.up().up().selection, "two");
}

#[test]
fn it_loop_around_when_reaching_visible_limit() {
    let input =  one_two_three();

    let config = Configuration::from_inputs(input, None, Some(2));
    let search = Search::blank(config);

    assert_eq!(search.down().down().down().selection, "two");
}

#[test]
fn it_moves_down_the_filtered_search_results() {
    let input =  one_two_three();

    let config = Configuration::from_inputs(input, None, None);
    let search = Search::blank(config);
    assert_eq!(search.append_to_search("t".to_string()).down().selection, "three");
}

#[test]
fn it_moves_down_the_filtered_search_results_twice() {
    let input =  one_two_three();

    let config = Configuration::from_inputs(input, None, None);
    let search = Search::blank(config);
    assert_eq!(search.append_to_search("t".to_string())
               .append_to_search("w".to_string()).selection, "two");
}