use wasm_bindgen::{JsCast, UnwrapThrowExt};
use web_sys::{Event, HtmlInputElement, InputEvent};
use yew::{
    function_component, html, use_effect, use_node_ref, use_state, Callback, Html, Properties,
};

static ICON_PATH: &str = "M536 654q-27 1-51.5 1.5t-43.5.5h-41q-71 0-133-2-53-2-104.5-5.5T88 \
                          639q-26-7-45-26t-26-45q-6-23-9.5-56T2 449q-2-36-2-73t2-73q2-30 5.5-63t9.5-56q7-26 \
                          26-45t45-26q23-6 74.5-9.5T267 98q62-2 133-2t133 2q53 2 104.5 5.5T712 113q26 7 45 \
                          26t26 45q6 23 9.5 56t5.5 63q2 36 2 73v17q-19-8-39-12.5t-41-4.5q-83 0-141.5 58.5T520 \
                          576q0 21 4 40.5t12 37.5zM320 496l208-120-208-120Zm360 200v-80h-80v-80h80v-80h80v80h80v80h-80v80z";

#[derive(Properties, PartialEq)]
pub struct VideoControlsProps {
    pub onsearch: Callback<String>,
}

#[function_component(VideoControls)]
pub fn video_controls(props: &VideoControlsProps) -> Html {
    let input_ref = use_node_ref();

    let text_to_search = use_state(|| String::new());

    let handle_input = {
        let text_to_search = text_to_search.clone();

        Callback::from(move |input_event: InputEvent| {
            // Obtener el texto desde el InputEvent
            let text = get_value_from_input_event(input_event);
            // web_sys::console::log_1(&text.into());
            text_to_search.set(text);
        })
    };

    let on_search_pressed = {
        let onsearch = props.onsearch.clone();
        let input_ref = input_ref.clone();
        let text_to_search = text_to_search.clone();

        Callback::from(move |_| {
            onsearch.emit(text_to_search.to_string());
            // Limpamos el input
            input_ref
                .cast::<web_sys::HtmlInputElement>()
                .unwrap()
                .set_value("");
        })
    };

    let toggle_theme_color = Callback::from(|_| {
        let local_storage = web_sys::window().unwrap().local_storage().unwrap().unwrap();

        let window = web_sys::window().expect("no global `window` exists");
        let document = window.document().expect("should have a document on window");

        match local_storage.get_item("theme").unwrap() {
            // if set via local storage previously
            Some(value) => {
                // let str_value = value.clone();
                if &value[..] == "light" {
                    // web_sys::console::log_1(&str_value.into());

                    document.document_element().unwrap().set_class_name("dark");
                    local_storage
                        .set_item("theme", "dark")
                        .expect("could not set a value in localStorage");
                } else {
                    document.document_element().unwrap().set_class_name("");
                    local_storage
                        .set_item("theme", "light")
                        .expect("could not set a value in localStorage");
                }
            }
            // if NOT set via local storage previously
            None => {
                let value = document.document_element().unwrap().class_name();
                // web_sys::console::log_1(&value[..].into());
                if &value[..] == "dark" {
                    document.document_element().unwrap().set_class_name("light");
                    local_storage
                        .set_item("theme", "light")
                        .expect("could not set a value in localStorage");
                } else {
                    document.document_element().unwrap().set_class_name("dark");
                    local_storage
                        .set_item("theme", "dark")
                        .expect("could not set a value in localStorage");
                }
            }
        }
    });

    {
        let input_ref = input_ref.clone();
        use_effect(move || {
            if let Some(input) = input_ref.cast::<web_sys::HtmlInputElement>() {
                input.focus().unwrap();
            }
        });
    }

    html! (
       <>
         <h1 class="text-6xl text-center mt-24 lg:text-2xl font-kanit">
           {"Buscador por palabras clave de Youtube"}
         </h1>

         /* BOTÓN PARA DARK MODE */
         <button
           class="absolute top-7 right-7 lg:top-4 lg:right-4"
           onclick={toggle_theme_color}
         >
           <i
             class="bi bi-moon-stars-fill text-sky-600 hover:text-sky-400 text-4xl lg:text-2xl"
           ></i>
         </button>

         <svg
           class="fill-red-500 w-40 mt-8"
           viewBox="0 96 840 600"
         >
           <path d="M279.664 221.876h252.122v279.627H279.664z" style="fill:#fff"/>
           <path d={ICON_PATH} />
         </svg>

         <input
           class="rounded-md px-2 py-4 w-4/5 bg-slate-200 dark:bg-black mt-20 focus:outline-none focus:ring focus:ring-blue-400 font-ubuntu
           text-3xl lg:text-lg max-w-[60%]"
           type="text"
           oninput={handle_input}
           placeholder="Ingresa una palabra…"
           ref={input_ref}
         />

         <button
           class="bg-sky-600 hover:bg-sky-400 mt-10 w-4/5 text-slate-50 rounded-md px-2 py-4 font-ubuntu text-3xl lg:text-lg max-w-[60%]"
           onclick={on_search_pressed}
         >
           {"Buscar"}
         </button>
       </>
    )
}

fn get_value_from_input_event(e: InputEvent) -> String {
    let event: Event = e.dyn_into().unwrap_throw();
    let event_target = event.target().unwrap_throw();
    let target: HtmlInputElement = event_target.dyn_into().unwrap_throw();
    target.value()
}
