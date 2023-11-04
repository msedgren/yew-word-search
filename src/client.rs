use crate::generator::generate_puzzle;
use yew::prelude::*;
use wasm_bindgen::{JsCast, JsValue};
use web_sys::HtmlInputElement;
use gloo_console::log;

#[derive(Properties, PartialEq)]
pub struct PuzzleProps {
    pub puzzle: Vec<Vec<char>>,
}

#[function_component]
pub fn Puzzle(PuzzleProps { puzzle }: &PuzzleProps) -> Html {
    let rows = puzzle.iter().map(|row|
        html! {
            <Row row={ row.clone() } />
        }).collect::<Html>();

    html! {
        <div class="m-4 p-3">
            { rows }
        </div>
    }
}

#[derive(Properties, PartialEq)]
struct RowProps {
    row: Vec<char>,
}

#[function_component]
fn Row(RowProps { row }: &RowProps) -> Html {
    let columns = row.iter().map(|column| html! {
        <Column value={ column.clone() } />
    }).collect::<Html>();

    html! {
        <div class="mx-0 my-2">
            { columns }
        </div>
    }
}

#[derive(Properties, PartialEq)]
struct ColumnProps {
    value: char,
}

#[function_component]
fn Column(ColumnProps { value }: &ColumnProps) -> Html {
    html! {
        <span class="font-mono text-xl my-2 mx-3">
            { value }
        </span>
    }
}


#[derive(Properties, PartialEq)]
struct DimensionProps {
    name: String,
    label: String,
    on_entry: Callback<i16>,
}


#[function_component]
fn DimensionInput(DimensionProps { name, label, on_entry }: &DimensionProps) -> Html {
    let onchange = {
        let on_entry = on_entry.clone();
        Callback::from(move |e: InputEvent| {
            let target = e.target().expect("Event should have a target when dispatched");
            let input = target.unchecked_into::<HtmlInputElement>();
            let value = input.value().parse::<i16>();

            match value {
                Ok(ok) => on_entry.emit(ok),
                Err(err) => log!(JsValue::from(err.to_string())),
            }
        })
    };

    html! {
        <div class="grid grid-cols-2">
            <label class="font-bold" for={name.clone()}>{ label }</label>
            <input class="border shadow-md" type="number" min="1" max="100" oninput={onchange} />
        </div>
    }
}

#[function_component]
pub fn App() -> Html {
    let placed_words: UseStateHandle<Vec<String>> = use_state(|| Vec::new());
    let puzzle_state: UseStateHandle<Vec<Vec<char>>> = use_state(|| vec![]);
    let width: UseStateHandle<String> = use_state(|| "".to_string());
    let height: UseStateHandle<String> = use_state(|| "".to_string());
    let error: UseStateHandle<String> = use_state(|| "".to_string());

    let words = use_state(|| String::new());

    let onsubmit = {
        let words = words.clone();
        let puzzle_state = puzzle_state.clone();
        let placed_words = placed_words.clone();
        let width = width.clone();
        let height = height.clone();
        let error = error.clone();
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();

            let split_words = words.lines().collect::<Vec<&str>>();
            let parsed_width = width.parse::<i16>();
            let parsed_height = height.parse::<i16>();
            if parsed_height.is_err() || parsed_width.is_err() {
                error.set("Invalid width or height".to_string());
                puzzle_state.set(Vec::new());
                placed_words.set(Vec::new());
            } else {
                // let split_words_str = split_words.iter().map(|word| &word[..]).collect::<Vec<&str>>();
                let (puzzle, failed_words) = generate_puzzle(parsed_width.unwrap(), parsed_height.unwrap(), &split_words);
                puzzle_state.set(puzzle);
                placed_words.set(split_words.iter()
                    .filter(|word| !failed_words.contains(&&word[..]))
                    .map(|word| word.to_uppercase())
                    .collect());

                if !failed_words.is_empty() {
                    error.set("Could not place all words".to_string());
                } else {
                    error.set("".to_string());
                }
            }
        })
    };

    let on_height_change = {
        let height = height.clone();
        let error = error.clone();
        Callback::from(move |given: i16| {
            error.set("".to_string());
            height.set(given.to_string())
        })
    };

    let on_width_change = {
        let width = width.clone();
        let error = error.clone();
        Callback::from(move |given: i16| {
            error.set("".to_string());
            width.set(given.to_string())
        })
    };

    let on_words_change = {
        let words = words.clone();
        let error = error.clone();
        Callback::from(move |e: Event| {
            let target = e.target().expect("Event should have a target when dispatched");
            let input = target.unchecked_into::<HtmlInputElement>();

            words.set(input.value());
            error.set("".to_string());
        })
    };

    html! {
        <div class="container mx-auto">
            <div>
                <h1 class="my-5 font-bold text-3xl underline print:hidden">{ "Word Search puzzle Generator" }</h1>
            </div>
            if !(*error).is_empty() {
                <div class="md:w-1/5 bg-red-200 border border-red-400 text-red-700 m-3 px-3 py-3 rounded" role="alert">
                    <strong class="font-bold">{ "Error: " }</strong>
                    <span class="block sm:inline">{ (*error).clone() }</span>
                </div>
            }
            <form class="md:w-1/5 space-y-3 p-2 print:hidden" name="word_form" {onsubmit}>
                <div>
                    <DimensionInput name="width" label="Width" on_entry={on_width_change} />
                </div>
                <div>
                    <DimensionInput name="height" label="Height" on_entry={on_height_change} />
                </div>
                <div>
                    <div class="grid grid-cols-2">
                        <label class="font-bold" for="words">{ "Words: " }</label>
                        <textarea class="border shadow-md" id="words" name="words" rows="10" cols="50" value={(*words).clone()} onchange={on_words_change} />
                    </div>
                </div>
                <div class="py-5">
                    <button class="rounded-full p-2 font-bold bg-cyan-200 hover:bg-cyan-300 shadow-md">{ "Generate" }</button>
                </div>
            </form>
            if !(*placed_words).is_empty() {
                <div>
                    <h3 class="font-bold underline text-xl">{ "Words:" }</h3>
                    <div class="grid grid-cols-4">
                        { for (*placed_words).iter().map(|word| html! { <span>{ word }</span> }) }
                    </div>
                </div>
            }
            <Puzzle puzzle={(*puzzle_state).clone()} />
        </div>
    }
}
