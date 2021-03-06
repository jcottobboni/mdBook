extern crate handlebars;
extern crate rustc_serialize;

use std::path::Path;
use std::collections::BTreeMap;

use self::rustc_serialize::json;
use self::handlebars::{Handlebars, HelperDef, RenderError, RenderContext, Helper, Context};

// Handlebars helper to construct TOC
#[derive(Clone, Copy)]
pub struct RenderToc;

impl HelperDef for RenderToc {
  fn call(&self, c: &Context, _h: &Helper, _: &Handlebars, rc: &mut RenderContext) -> Result<(), RenderError> {

    // get value from context data
    // rc.get_path() is current json parent path, you should always use it like this
    // param is the key of value you want to display
    let chapters = c.navigate(rc.get_path(), "chapters");
    let current = c.navigate(rc.get_path(), "path").to_string().replace("\"", "");
    try!(rc.writer.write("<ul class=\"chapter\">".as_bytes()));

    // Decode json format
    let decoded: Vec<BTreeMap<String,String>> = json::decode(&chapters.to_string()).unwrap();

    let mut current_level = 1;

    for item in decoded {

        let level = item.get("section").expect("Error: section should be Some(_)").len() / 2;
        if level > current_level {
            try!(rc.writer.write("<li>".as_bytes()));
            try!(rc.writer.write("<ul class=\"section\">".as_bytes()));
            try!(rc.writer.write("<li>".as_bytes()));
        } else if level < current_level {
            while level < current_level {
                try!(rc.writer.write("</ul>".as_bytes()));
                try!(rc.writer.write("</li>".as_bytes()));
                current_level = current_level - 1;
            }
            try!(rc.writer.write("<li>".as_bytes()));
        }
        else {
            try!(rc.writer.write("<li>".as_bytes()));
        }

        // Link
        let path_exists = if let Some(path) = item.get("path") {
            if path.len() > 0 {
                try!(rc.writer.write("<a href=\"".as_bytes()));

                // Add link
                try!(rc.writer.write(
                    Path::new(
                        item.get("path")
                            .expect("Error: path should be Some(_)")
                        ).with_extension("html")
                        .to_str().unwrap().as_bytes()
                    ));

                try!(rc.writer.write("\"".as_bytes()));

                if path == &current {
                    try!(rc.writer.write(" class=\"active\"".as_bytes()));
                }

                try!(rc.writer.write(">".as_bytes()));
                true
            } else {
                false
            }
        }else {
            false
        };

        try!(rc.writer.write("<strong>".as_bytes()));
        try!(rc.writer.write(item.get("section").expect("Error: section should be Some(_)").as_bytes()));
        try!(rc.writer.write("</strong> ".as_bytes()));
        try!(rc.writer.write(item.get("name").expect("Error: name should be Some(_)").as_bytes()));

        if path_exists {
            try!(rc.writer.write("</a>".as_bytes()));
        }

        try!(rc.writer.write("</li>".as_bytes()));

        current_level = level;
    }

    try!(rc.writer.write("</ul>".as_bytes()));
    Ok(())
  }
}
