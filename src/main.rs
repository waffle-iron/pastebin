#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate rand;

mod paste_id;

use paste_id::PasteID;
use rocket::Data;
use std::io;
use std::path::Path;
use std::fs::File;

#[get("/")]
fn index() -> &'static str {
    "
    USAGE

      POST /

          accepts raw data in the body of the request and responds with a URL of
          a page containing the body's content

      GET /<id>

          retrieves the content for the paste with id `<id>`
    "
}

#[post("/", data = "<paste>")]
fn upload(paste: Data) -> io::Result<String> {
    let id = PasteID::new(3);
    let filename = format!("upload/{id}", id = id);
    let url = format!("{host}/{id}\n", host = "http://localhost:8000", id = id);

    // Write the paste out to the file and return the URL.
    paste.stream_to_file(Path::new(&filename))?;
    Ok(url)
}

#[get("/<id>")]
fn retrieve(id: PasteID) -> Option<File> {
    let filename = format!("upload/{id}", id = id);
    File::open(&filename).ok()
}

fn main() {
    rocket::ignite().mount("/", routes![index, upload, retrieve]).launch()
}

// Suggestions:
// Add a web form to the index where users can manually input new pastes. Accept the form at POST /. Use format and/or rank to specify which of the two POST / routes should be called.
// Support deletion of pastes by adding a new DELETE /<id> route. Use PasteID to validate <id>.
// Limit the upload to a maximum size. If the upload exceeds that size, return a 206 partial status code. Otherwise, return a 201 created status code.
// Set the Content-Type of the return value in upload and retrieve to text/plain.
// Return a unique “key” after each upload and require that the key is present and matches when doing deletion. Use one of Rocket’s core traits to do the key validation.
// Add a PUT /<id> route that allows a user with the key for <id> to replace the existing paste, if any.
// Add a new route, GET /<id>/<lang> that syntax highlights the paste with ID <id> for language <lang>. If <lang> is not a known language, do no highlighting. Possibly validate <lang> with FromParam.
// Use the testing module to write unit tests for your pastebin.
// Dispatch a thread before launching Rocket in main that periodically cleans up idling old pastes in upload/.
