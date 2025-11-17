use std::rc::Rc;
use std::cell::RefCell;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{window, Document, Element, HtmlButtonElement, HtmlInputElement, InputEvent, MouseEvent};

// A helper to get the document object
fn document() -> Document {
    window().unwrap().document().unwrap()
}

#[wasm_bindgen]
pub struct Sudoku {
    board: [u8; 81],
    cells: Vec<Element>, // To store cell elements for highlighting
    closures: Vec<Closure<dyn FnMut(InputEvent)>>,
    keypad_closures: Vec<Closure<dyn FnMut(MouseEvent)>>,
}

#[wasm_bindgen]
impl Sudoku {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Sudoku {
        let board = [
            5, 3, 0, 0, 7, 0, 0, 0, 0, 6, 0, 0, 1, 9, 5, 0, 0, 0, 0, 9, 8, 0, 0, 0, 0, 6, 0,
            8, 0, 0, 0, 6, 0, 0, 0, 3, 4, 0, 0, 8, 0, 3, 0, 0, 1, 7, 0, 0, 0, 2, 0, 0, 0, 6,
            0, 6, 0, 0, 0, 0, 2, 8, 0, 0, 0, 0, 4, 1, 9, 0, 0, 5, 0, 0, 0, 0, 8, 0, 0, 7, 9,
        ];
        Sudoku { board, cells: vec![], closures: vec![], keypad_closures: vec![] }
    }

    pub fn render(&mut self, container: &Element) {
        container.set_inner_html(""); // Clear previous board
        self.closures.clear();
        self.cells.clear();

        let board_container = document().create_element("div").unwrap();
        board_container.set_attribute("class", "sudoku-board").unwrap();

        for i in 0..81 {
            let val = self.board[i];
            let cell: Element;

            if val != 0 {
                cell = document().create_element("div").unwrap();
                cell.set_text_content(Some(&val.to_string()));
                cell.set_attribute("class", "sudoku-cell given").unwrap();
            } else {
                cell = document().create_element("input").unwrap();
                let input_element = cell.clone().dyn_into::<HtmlInputElement>().unwrap();
                input_element.set_type("text");
                input_element.set_max_length(1);
                input_element.set_attribute("class", "sudoku-cell").unwrap();
                input_element.set_attribute("data-index", &i.to_string()).unwrap();

                let board_ptr = self as *mut Sudoku;
                let closure = Closure::wrap(Box::new(move |event: InputEvent| {
                    let target = event.target().unwrap();
                    let input = target.dyn_into::<HtmlInputElement>().unwrap();
                    let val_str = input.value();
                    let index = input.get_attribute("data-index").unwrap().parse::<usize>().unwrap();
                    let val = val_str.trim().parse::<u8>().unwrap_or(0);
                    
                    unsafe {
                        (*board_ptr).board[index] = val;
                    }
                }) as Box<dyn FnMut(InputEvent)>);

                input_element.add_event_listener_with_callback("input", closure.as_ref().unchecked_ref()).unwrap();
                self.closures.push(closure);
            }
            self.cells.push(cell.clone());
            board_container.append_child(&cell).unwrap();
        }
        container.append_child(&board_container).unwrap();
        self.render_keypad();
    }

    fn render_keypad(&mut self) {
        self.keypad_closures.clear();
        let keypad_container = document().get_element_by_id("keypad-container").unwrap();
        keypad_container.set_inner_html("");

        let sudoku_ptr = self as *mut Sudoku;

        for i in 1..=10 { // 1-9 for numbers, 10 for clear
            let num = if i > 9 { 0 } else { i };
            let btn = document().create_element("button").unwrap().dyn_into::<HtmlButtonElement>().unwrap();
            btn.set_attribute("class", "keypad-btn").unwrap();
            btn.set_text_content(Some(if num == 0 { "C" } else { &num.to_string() }));

            let closure = Closure::wrap(Box::new(move |_: MouseEvent| {
                unsafe {
                    (*sudoku_ptr).highlight_number(num);
                }
            }) as Box<dyn FnMut(_)>);

            btn.add_event_listener_with_callback("click", closure.as_ref().unchecked_ref()).unwrap();
            self.keypad_closures.push(closure);
            keypad_container.append_child(&btn).unwrap();
        }
    }

    fn highlight_number(&self, num_to_highlight: u8) {
        for (i, cell_element) in self.cells.iter().enumerate() {
            cell_element.class_list().remove_1("highlight").unwrap();

            if num_to_highlight > 0 {
                let cell_value = if let Ok(input) = cell_element.clone().dyn_into::<HtmlInputElement>() {
                    input.value().parse::<u8>().unwrap_or(0)
                } else {
                    cell_element.text_content().unwrap_or_default().parse::<u8>().unwrap_or(0)
                };

                if cell_value == num_to_highlight {
                    cell_element.class_list().add_1("highlight").unwrap();
                }
            }
        }
    }

    pub fn check_solution(&self) {
        let is_complete = !self.board.contains(&0);
        let message = if is_complete {
            "Congratulations! Puzzle complete!"
        } else {
            "The puzzle is not finished yet."
        };
        window().unwrap().alert_with_message(message).unwrap();
    }

    pub fn new_game(&mut self, container: &Element) {
        self.board = Sudoku::new().board;
        self.render(container);
    }
}

#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();

    let doc = document();
    let game_container = doc.get_element_by_id("game-container").unwrap();

    let sudoku = Rc::new(RefCell::new(Sudoku::new()));
    sudoku.borrow_mut().render(&game_container);

    // --- Event listener for Check button ---
    let check_btn = doc.get_element_by_id("check-btn").unwrap().dyn_into::<HtmlButtonElement>()?;
    let sudoku_for_check = Rc::clone(&sudoku);
    let check_closure = Closure::wrap(Box::new(move |_: MouseEvent| {
        sudoku_for_check.borrow().check_solution();
    }) as Box<dyn FnMut(_)>);
    check_btn.add_event_listener_with_callback("click", check_closure.as_ref().unchecked_ref())?;
    check_closure.forget();

    // --- Event listener for New Game button ---
    let new_game_btn = doc.get_element_by_id("new-game-btn").unwrap().dyn_into::<HtmlButtonElement>()?;
    let sudoku_for_new_game = Rc::clone(&sudoku);
    let new_game_closure = Closure::wrap(Box::new(move |_: MouseEvent| {
        let container = document().get_element_by_id("game-container").unwrap();
        sudoku_for_new_game.borrow_mut().new_game(&container);
    }) as Box<dyn FnMut(_)>);
    new_game_btn.add_event_listener_with_callback("click", new_game_closure.as_ref().unchecked_ref())?;
    new_game_closure.forget();

    Ok(())
}