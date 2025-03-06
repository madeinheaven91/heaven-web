use yew::Properties;
use yew::{function_component, html, Html}; 

use crate::models::Tag;

#[function_component(TagList)]
pub fn tag_list(TagListProps { tags }: &TagListProps) -> Html {
    if tags.is_empty() {
        html! {
            <p class="text-secondary">{ "Нет тегов..." }</p>
        }
    } else {
        tags.iter()
            .map(|tag| {
                let style = format!(
                    "text-decoration: none; background-color: #{}; color: #{}",
                    &tag.background_color, &tag.foreground_color
                );
                html! {
                    <a class="badge mr-1" style={style}>
                        { &tag.name }
                    </a>
                }
            })
            .collect()
    }
}

#[derive(Properties, PartialEq)]
pub struct TagListProps {
    pub tags: Vec<Tag>,
}
