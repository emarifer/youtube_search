use yew::{function_component, html, Html, Properties};

#[derive(Properties, PartialEq)]
pub struct VideoSectionProps {
    pub id: String,
    pub name: String,
}

#[function_component(VideoSection)]
pub fn video_section(props: &VideoSectionProps) -> Html {
    let yt_url = format!("https://www.youtube.com/embed/{}", props.id);
    // let yt_name = format!("{}", props.name);

    html! {
       <div class="mt-8 flex flex-col justify-center">
         <iframe class="mt-32 w-[650px] h-[365px] m-auto" src={yt_url}  />
         <h5 class="mt-8 text-2xl text-center">{&props.name}</h5>
       </div>
    }
}
