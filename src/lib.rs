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
    closures: Vec<Closure<dyn FnMut(InputEvent)>>,
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
        Sudoku { board, closures: vec![] }
    }

    pub fn render(&mut self, container: &Element) {
        container.set_inner_html(""); // Clear previous board
        self.closures.clear(); // Clear old closures

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
                    
                    // Unsafe block to get mutable access to self from a closure
                    unsafe {
                        (*board_ptr).board[index] = val;
                    }
                }) as Box<dyn FnMut(InputEvent)>);

                input_element.add_event_listener_with_callback("input", closure.as_ref().unchecked_ref()).unwrap();
                self.closures.push(closure);
            }
            board_container.append_child(&cell).unwrap();
        }
        container.append_child(&board_container).unwrap();
    }

    pub fn check_solution(&self) {
        let is_complete = !self.board.contains(&0);
        let message = if is_complete {
            // A full implementation would validate the solution here
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
    let doc = document();
    let body = doc.body().unwrap();

    // Create a main container for the game
    let game_container = doc.create_element("div")?;
    game_container.set_id("game-container");
    body.append_child(&game_container)?;

    let sudoku = Rc::new(RefCell::new(Sudoku::new()));
    sudoku.borrow_mut().render(&game_container);

    // --- Event listener for Check button ---
    let check_btn = doc.get_element_by_id("check-btn").unwrap().dyn_into::<HtmlButtonElement>()?;
    let sudoku_for_check = Rc::clone(&sudoku);
    let check_closure = Closure::wrap(Box::new(move |_: MouseEvent| {
        sudoku_for_check.borrow().check_solution();
    }) as Box<dyn FnMut(_)>);
    check_btn.add_event_listener_with_callback("click", check_closure.as_ref().unchecked_ref())?;
    check_closure.forget(); // Leaks the closure memory

    // --- Event listener for New Game button ---
    let new_game_btn = doc.get_element_by_id("new-game-btn").unwrap().dyn_into::<HtmlButtonElement>()?;
    let sudoku_for_new_game = Rc::clone(&sudoku);
    let new_game_closure = Closure::wrap(Box::new(move |_: MouseEvent| {
        let container = document().get_element_by_id("game-container").unwrap();
        sudoku_for_new_game.borrow_mut().new_game(&container);
    }) as Box<dyn FnMut(_)>);
    new_game_btn.add_event_listener_with_callback("click", new_game_closure.as_ref().unchecked_ref())?;
    new_game_closure.forget(); // Leaks the closure memory

    Ok(())
}
