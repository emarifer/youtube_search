use chrono::{Datelike, Utc};
use yew::{function_component, html, use_state, Callback, Html, UseStateHandle};

mod components;
mod env;
mod youtube;

use crate::components::{video_controls::VideoControls, video_section::VideoSection};

fn main() {
    yew::Renderer::<App>::new().render();
}

// Este es nuestro modelo de datos
#[derive(Clone)]
struct Video {
    id: String,
    name: String,
}

#[function_component(App)]
fn app() -> Html {
    let video: UseStateHandle<Option<Video>> = use_state(|| None);

    let on_search = {
        let video = video.clone();
        Callback::from(move |text_to_search: String| {
            let video = video.clone();
            wasm_bindgen_futures::spawn_local(async move {
                match youtube::search_youtube(text_to_search).await {
                    Ok(video_item) => video.set(Some(Video {
                        id: video_item.id.video_id,
                        name: video_item.snippet.title,
                    })),
                    Err(e) => web_sys::console::log_1(&e.to_string().into()),
                }
            });
        })
    };

    let video_section = match (*video).clone() {
        Some(video) => html! {
          <VideoSection name={video.name} id={video.id} />
        },
        None => html! {},
    };

    html! {
        <main class="flex flex-col items-center">
          <VideoControls onsearch={on_search} />

          {video_section}

          <footer class="mt-20 flex items-center gap-4">
            <div class="hover:shadow-md ease-in duration-300 shadow-slate-600 dark:shadow-slate-50 rounded-full">
              <a href="https://github.com/emarifer?tab=repositories" target="_blank" >
                <img src="img/profile.png" alt="Profile" class="w-36 lg:w-24" />
              </a>
            </div>
            <a
              class="text-2xl lg:text-lg hover:text-sky-500 ease-in duration-300"
              href="https://github.com/emarifer/youtube_search"
              target="_blank"
            >
              {format!("MIT Licensed | Copyright © {} Enrique Marín", Utc::now().year())}
            </a>
          </footer>
        </main>
    }
}

/*
 * https://yew.rs/
 * https://www.youtube.com/watch?v=t6f0UOTwX4A
 * https://github.com/juliooa/youtube_search
 * https://github.com/jakobwesthoff/lexica_inkplate_server/tree/main/frontend
 * https://trunkrs.dev/
 * https://tailwindcss.com/docs/installation
 *
 * https://stackoverflow.com/questions/70308340/how-to-add-an-image-in-yew
 * https://www.youtube.com/watch?v=Wrze9x2EIgU
 * https://trunkrs.dev/assets/#images-other-resources
 * https://stackoverflow.com/questions/64197107/how-to-modify-svg-icon-colors-with-tailwind
 * https://jakearchibald.github.io/svgomg/
 * https://webdesign.tutsplus.com/es/tutorials/svg-viewport-and-viewbox-for-beginners--cms-30844
 * https://www.includehelp.com/rust/print-the-current-year-using-year-method.aspx
 */
