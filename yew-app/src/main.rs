use yew::html::Scope;
use yew::prelude::*;
use yew_router::prelude::*;

mod pages;
use pages::counter::Counter;
use pages::home::Home;
use pages::video::Videos;

#[derive(Debug, Clone, Copy, PartialEq, Routable)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/counter")]
    Counter,
    #[at("/videos")]
    Videos,
    #[not_found]
    #[at("/404")]
    NotFound,
}

pub enum Msg {
    ToggleNavbar,
}

pub struct App {
    navbar_active: bool,
}
impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            navbar_active: false,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::ToggleNavbar => {
                self.navbar_active = !self.navbar_active;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <BrowserRouter>
                { self.view_nav(ctx.link()) }

                <main>
                    <Switch<Route> render={Switch::render(switch)} />
                </main>
                <footer class="footer">
                    <div class="content has-text-centered">
                        { "Powered by " }
                        <a href="https://yew.rs">{ "Yew" }</a>
                        { " using " }
                        <a href="https://bulma.io">{ "Bulma" }</a>
                        { " and images from " }
                        <a href="https://unsplash.com">{ "Unsplash" }</a>
                    </div>
                </footer>
            </BrowserRouter>
        }
    }
}
impl App {
    fn view_nav(&self, link: &Scope<Self>) -> Html {
        let Self { navbar_active, .. } = *self;

        let active_class = if !navbar_active { "is-active" } else { "" };

        html! {
            <nav class="navbar is-primary" role="navigation" aria-label="main navigation">
                <div class="navbar-brand">
                    <h1 class="navbar-item is-size-3">{ "Yew Blog" }</h1>

                    <button class={classes!("navbar-burger", "burger", active_class)}
                        aria-label="menu" aria-expanded="false"
                        onclick={link.callback(|_| Msg::ToggleNavbar)}
                    >
                        <span aria-hidden="true"></span>
                        <span aria-hidden="true"></span>
                        <span aria-hidden="true"></span>
                    </button>
                </div>
                <div class={classes!("navbar-menu", active_class)}>
                    <div class="navbar-start">
                        <Link<Route> classes={classes!("navbar-item")} to={Route::Home}>
                            { "Home" }
                        </Link<Route>>
                        <Link<Route> classes={classes!("navbar-item")} to={Route::Counter}>
                            { "Counter" }
                        </Link<Route>>

                        <div class="navbar-item has-dropdown is-hoverable">
                            <div class="navbar-link">
                                { "More" }
                            </div>
                            <div class="navbar-dropdown">
                                <Link<Route> classes={classes!("navbar-item")} to={Route::Videos}>
                                    { "See the videos" }
                                </Link<Route>>
                            </div>
                        </div>
                    </div>
                </div>
            </nav>
        }
    }
}

fn switch(routes: &Route) -> Html {
    match routes {
        Route::Home => {
            html! { <Home /> }
        }
        Route::Counter => {
            html! { <Counter /> }
        }
        Route::Videos => {
            html! { <Videos /> }
        }
        Route::NotFound => html! { <h1>{ "404" }</h1> },
    }
}

fn main() {
    // wasm_logger::init(wasm_logger::Config::new(log::Level::Trace));
    yew::start_app::<App>();
}

// #[derive(Clone, PartialEq, Deserialize)]
// struct Video {
//     id: usize,
//     title: String,
//     speaker: String,
//     url: String,
// }

// #[derive(Clone, Properties, PartialEq)]
// struct VideosListProps {
//     videos: Vec<Video>,
//     on_click: Callback<Video>,
// }

// #[function_component(VideosList)]
// fn videos_list(VideosListProps { videos, on_click }: &VideosListProps) -> Html {
//     let on_click = on_click.clone();
//     videos
//         .iter()
//         .map(|video| {
//             let on_video_select = {
//                 let on_click = on_click.clone();
//                 let video = video.clone();
//                 Callback::from(move |_| on_click.emit(video.clone()))
//             };

//             html! {
//                 <p onclick={on_video_select}>{format!("{}: {}", video.speaker, video.title)}</p>
//             }
//         })
//         .collect()
// }

// #[derive(Clone, Properties, PartialEq)]
// struct VideosDetailsProps {
//     video: Video,
// }

// #[function_component(VideoDetails)]
// fn video_details(VideosDetailsProps { video }: &VideosDetailsProps) -> Html {
//     html! {
//         <div>
//             <h3>{ video.title.clone() }</h3>
//             <img src="https://via.placeholder.com/640x360.png?text=Video+Player+Placeholder" alt="video thumbnail" />
//         </div>
//     }
// }

// #[function_component(App)]
// fn app() -> Html {
//     let fetched_videos = vec![
//         Video {
//             id: 1,
//             title: "Building and breaking things".to_string(),
//             speaker: "John Doe".to_string(),
//             url: "https://youtu.be/PsaFVLr8t4E".to_string(),
//         },
//         Video {
//             id: 2,
//             title: "The development process".to_string(),
//             speaker: "Jane Smith".to_string(),
//             url: "https://youtu.be/PsaFVLr8t4E".to_string(),
//         },
//         Video {
//             id: 3,
//             title: "The Web 7.0".to_string(),
//             speaker: "Matt Miller".to_string(),
//             url: "https://youtu.be/PsaFVLr8t4E".to_string(),
//         },
//         Video {
//             id: 4,
//             title: "Mouseless development".to_string(),
//             speaker: "Tom Jerry".to_string(),
//             url: "https://youtu.be/PsaFVLr8t4E".to_string(),
//         },
//     ];

//     // let videos = fetched_videos
//     //     .iter()
//     //     .map(|video| {
//     //         html! {
//     //             <p>{format!("{}: {}", video.speaker, video.title)}</p>
//     //         }
//     //     })
//     //     .collect::<Html>();

//     let videos = use_state(|| vec![]);
//     {
//         let videos = videos.clone();
//         use_effect_with_deps(
//             move |_| {
//                 let videos = videos.clone();
//                 // wasm_bindgen_futures::spawn_local(async move {
//                 //     let fetched_videos: Vec<Video> = Request::get("/tutorial/data.json")
//                 //         .send()
//                 //         .await
//                 //         .unwrap()
//                 //         .json()
//                 //         .await
//                 //         .unwrap();
//                 //     videos.set(fetched_videos);
//                 // });
//                 videos.set(fetched_videos);
//                 || ()
//             },
//             (),
//         );
//     }

//     let selected_video = use_state(|| None);

//     let on_video_select = {
//         let selected_video = selected_video.clone();
//         Callback::from(move |video: Video| selected_video.set(Some(video)))
//     };

//     let details = selected_video.as_ref().map(|video| {
//         html! {
//             <VideoDetails video={video.clone()} />
//         }
//     });

//     html! {
//         <>
//             <h1>{"Videos to watch"}</h1>
//             <div>
//                 <h3>
//                     // {videos}
//                     <VideosList videos={(*videos).clone()} on_click={on_video_select.clone()} />
//                     { for details }
//                 </h3>
//             </div>
//         </>
//     }
// }

// fn main() {
//     yew::start_app::<App>();
// }
