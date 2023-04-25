use actix_files::Files; // new line
use std::net::TcpListener;
use actix_web::{dev::Server, web, App, HttpResponse, HttpServer, middleware};
use tera::Tera;
pub mod handlers; // new line
#[macro_use]
extern crate lazy_static;
lazy_static! {
	pub static ref TEMPLATES: Tera = {
	    let mut tera = match Tera::new("templates/**/*.html") {
			Ok(t) => t,
			Err(e) => {
				println!("Parsing error(s): {}", e);
				::std::process::exit(1);
			}
		};
		tera.autoescape_on(vec![".html", ".sql"]);
		tera
	};
}
pub fn start_blog(listener: TcpListener) -> Result<Server, std::io::Error> {
	let srv = HttpServer::new(move || {
	    App::new()
		   .app_data(web::Data::new(TEMPLATES.clone()))
		   .wrap(middleware::Logger::default()) // enable logger
           .service(Files::new("/static", "static/").use_last_modified(true)) // new line
		   .route("/health", web::get().to(HttpResponse::Ok))
           .service(handlers::index) // new line
           .service(handlers::post)
	})
	.listen(listener)?
	.run();
	
	Ok(srv)
}