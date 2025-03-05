
use dioxus::prelude::*;
use gloo_net::http::Request;

// use nanorand::Rng;
const BUTTON_STYLE:Asset=asset!("/assets/button.css");


fn main() {

    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    let  mut  id=use_signal(|| "".to_string());
    let  resource=use_resource(move ||{
        let _id=id();
        async move{
            let response=match Request::get("https://linux.do/latest.rss")
                .send().await {
                Ok(response) => response.text().await.unwrap_or_else(|_| "解析失败".to_string()),
                Err(_) => "请求失败".to_string(),
            };
            let re = regex::Regex::new(r#"<dc:creator><!\[CDATA\[(.*?)\]\]></dc:creator>"#).unwrap();
            let mut ids: Vec<String> = re.captures_iter(&response)
                    .map(|cap| cap[1].to_string()).collect();
            if ids.is_empty(){
                ids.push("NULL".to_string());
            }
            ;

            let result=ids.get(fastrand::usize(..ids.len())).unwrap();
            result.to_string()       
        }}
        
    );

    rsx! {
        link { rel:"stylesheet", href: BUTTON_STYLE }
        textarea { id:"text1",placeholder:"ID:",value:id}
        button {
            id:"button1", 
            onclick: move|_| 
                match &*resource.read(){
                    Some(res)=>{ id.set(res.to_string())},
                    None=>{id.set("ERROR".to_string())}
                }
            ,
            "Random"
        }

    }
}




