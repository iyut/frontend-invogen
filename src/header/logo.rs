extern crate base64;

use std::collections::HashMap;

use base64::encode;
use gloo::file::callbacks::FileReader;
use gloo::file::File;
use gloo::storage::LocalStorage;
use gloo_storage::Storage;
use web_sys::{ DragEvent, Event, FileList, HtmlInputElement };
use yew::html::TargetCast;
use yew::{ html, Callback, Component, Context, Html };

struct FileDetails {
	name: String,
	file_type: String,
	data: Vec<u8>,
}

pub enum Msg {
	Loaded( String, String, Vec<u8> ),
	Files( Vec<File> ),
}

pub struct Logo {
	readers: HashMap<String, FileReader>,
	files: Vec<FileDetails>,
}

impl Component for Logo {
	type Message = Msg;
	type Properties = ();

	fn create( _ctx: &Context<Self> ) -> Self {
		Self {
			readers: HashMap::default(),
			files: Vec::default(),
		}
	}

	fn update( &mut self, ctx: &Context<Self>, msg: Self::Message ) -> bool {
		match msg {
			Msg::Loaded( file_name, file_type, data ) => {

				self.files.push( FileDetails {
					data: data.clone(),
					file_type,
					name: file_name.clone(),
				} );

				LocalStorage::set( String::from( "testthis" ), encode( data ) ).ok();

				self.readers.remove( &file_name );
				
				return true;
			}

			Msg::Files( files ) => {
				for file in files.into_iter() {
					let file_name = file.name();
					let file_type = file.raw_mime_type();

					let task = {
						let link = ctx.link().clone();
						let file_name = file_name.clone();

						gloo::file::callbacks::read_as_bytes(&file, move |res| {
							link.send_message( Msg::Loaded(
								file_name,
								file_type,
								res.expect( "failed to read file" ),
							))
						})
					};

					self.readers.insert( file_name, task );
				}

				return true;
			}
		}
	}

	fn view( &self, ctx: &Context<Self> ) -> Html {

		let div_ondrop = ctx.link().callback( |event: DragEvent| {
			event.prevent_default();
			let files = event.data_transfer().unwrap().files();
			
			return Self::upload_files( files );
		} );

		let div_ondragover = Callback::from( |event: DragEvent| {
			event.prevent_default();
		} );

		let div_ondragenter = Callback::from( |event: DragEvent| {
			event.prevent_default();
		} );

		let input_file_onchange = ctx.link().callback(move |e: Event| {
			let input: HtmlInputElement = e.target_unchecked_into();
			
			return Self::upload_files( input.files() );
		});

		html! {
			<div id="wrapper">
				<p id="title">{ String::from( "Upload Your Files To The Cloud" ) }</p>
				<label for="file-upload">
				<div id="drop-container" ondrop={div_ondrop} ondragover={div_ondragover} ondragenter={div_ondragenter} >
					<i class="fa fa-cloud-upload"></i>
					<p>{"Drop your images here or click to select"}</p>
				</div>
				</label>
				<input id="file-upload" type="file" accept="image/*,video/*" multiple={true} onchange={input_file_onchange} />
				<div id="preview-area">
					{ for self.files.iter().map( Self::view_file ) }
				</div>
			</div>
		}
	}
}

impl Logo {
	fn view_file( file: &FileDetails ) -> Html {

		let yew: String = LocalStorage::get("testthis").unwrap_or_default();
		html! {
			<div class="preview-tile">
				<p class="preview-name">{ format!( "{}", file.name ) }</p>
				<div class="preview-media">
					if file.file_type.contains( "image" ) {
						<img src={  format!( "data:{};base64,{}", file.file_type, yew ) } />
					} else if file.file_type.contains( "video" ) {
						<video controls={true}>
							<source src={ format!( "data:{},base64,{}", file.file_type, yew ) } type={file.file_type.clone() } />
						</video>
					}
				</div>
			</div>
		}
	}

	fn upload_files( files: Option<FileList> ) -> Msg {
		let mut result = Vec::new();

		if let Some( files ) = files {
			let files = js_sys::try_iter( &files )
				.unwrap()
				.unwrap()
				.map( |v| web_sys::File::from( v.unwrap() ) )
				.map( File::from );

			result.extend( files );
		}

		Msg::Files( result )
	}
}