use yew::prelude::*;

pub struct Simple {
	word: String,
}

pub enum Msg {
	FirstWord,
	SecondWord,
}

impl Component for Simple {
	type Properties = ();
	type Message = Msg;

	fn create( _ctx: &Context<Self> ) -> Self {
		Self {
			word: String::from( "test this" ),
		}
	}

	fn update( &mut self, _ctx: &Context<Self>, msg: Self::Message ) -> bool {
		match msg {
			Msg::FirstWord => {
				self.word = String::from( "first word" );
				true
			},
			Msg::SecondWord => {
				self.word = String::from( "second word" );
				true
			}
		}
	}

	fn view( &self, ctx: &Context<Self> ) -> Html {
		let word = &self.word;

		html! {
			<div>
				<div>{ word }</div>
				<div>
					<button onclick={ctx.link().callback( |_| Msg::FirstWord ) }>{ String::from( "click this" ) }</button>
					<input type="file" id="file-input" onchange={ctx.link().callback( |_| Msg::SecondWord )} />
				</div>
			</div>
		}
	}
}