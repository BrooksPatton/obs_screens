use stylist::{yew::styled_component, Style};
use yew::{function_component, html, use_state, Html};
use yew_hooks::use_effect_once;

#[styled_component(App)]
fn component_app() -> Html {
    let my_style = css!(
        r#"
            height: 100vh;
            display: flex;
            justify-content: center;
            align-items: center;
            flex-direction: column;

            img {
                height: 2rem;
            }

            pre {
              overflow: hidden; /* Ensures the content is not revealed until the animation */
              border-right: .15em solid orange; /* The typwriter cursor */
              margin: 0 auto; /* Gives that scrolling effect as the typing happens */
              letter-spacing: .15em; /* Adjust as needed */
              animation: 
                typing 3.5s steps(40, end),
                blink-caret .75s step-end infinite;
            }

            /* The typing effect */
            @keyframes typing {
              from { width: 0 }
              to { width: 100% }
            }

            /* The typewriter cursor effect */
            @keyframes blink-caret {
              from, to { border-color: transparent }
              50% { border-color: orange; }
            }
        "#
    );

    let class = Style::new(my_style).unwrap();

    let code_to_print = use_state(|| Vec::new());
    {
        let code_to_print = code_to_print.clone();

        use_effect_once(move || {
            let code_to_print = code_to_print.clone();

            wasm_bindgen_futures::spawn_local(async move {
                let code = get_text().await;
                code_to_print.set(code);
            });

            || {}
        })
    }
    html! {
        <main {class}>
            <img src="/images/svg/logo-bb-blue.svg" alt="Brooks Builds Logo" />
            <h1>{"Starting Soon"}</h1>
            // {
            //     code_to_print.iter().map(|code| html!{ <pre>{code.clone()}</pre>}).collect::<Vec<Html>>()
            // }
        </main>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}

async fn get_text() -> Vec<String> {
    let response = gloo::net::http::Request::get(
        "https://raw.githubusercontent.com/brooks-builds/code_of_conduct/master/README.md",
    )
    .send()
    .await
    .unwrap();

    let text = response.text().await.unwrap();

    text.lines().map(ToOwned::to_owned).collect()
}
