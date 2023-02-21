use yew::prelude::*;
use crate::header::simple::Simple;
use crate::header::logo::Logo;

pub mod header;

#[function_component(DocumentLogo)]
fn document_logo() -> Html {
	html! {
		<div class={ classes!( "document_logo_container" ) }>
			<Logo />
		</div>
	}
}

#[function_component(DocumentHeader)]
fn document_header() -> Html {
	html! {
		<header class={ classes!( "document_header" ) }>
			<DocumentLogo />
			
			<div class={ classes!( "document_title_container" ) }>
			</div>
			<div class={ classes!( "document_number_container" ) }>
			</div>
		</header>
	}
}

#[function_component(App)]
fn app() -> Html {
    html! {
        <div>
            <DocumentHeader />
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}