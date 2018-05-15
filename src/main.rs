/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

extern crate charname;
extern crate gtk;

use gtk::prelude::*;
use gtk::{Dialog, Entry};

use std::char;
use std::process::{Command, Stdio};

use std::io::Write;

use charname::get_name;


fn main() {
    gtk::init().unwrap();

    let dialog = Dialog::new();
    dialog.set_decorated(false);

    let content = dialog.get_content_area();
    let input = Entry::new();
    input.set_max_length(8);

    input.connect_key_press_event(|input, event_key| {
        if event_key.get_keyval() == 65293 {
            if let Some(text) = input.get_text() {
                if let Some(c) = get_char(&text) {
                    copy_clipboard(c);
                    gtk::main_quit();
                }
            }
        }

        Inhibit(false)
    });

    let status = gtk::Label::new("Please enter a codepoint\nex. 1F954");

    content.pack_start(&input, false, false, 0);
    content.pack_start(&status, false, false, 0);

    input.connect_changed(move |input| {
        if let Some(text) = input.get_text() {
            status.set_text(get_status(&text));
        };
    });

    dialog.show_all();

    dialog.connect_delete_event(|dialog, event| {
        gtk::main_quit();
        println!("Dialog: {:?}", dialog);
        println!("Event: {:?}", event);
        println!("End");
        Inhibit(false)
    });

    gtk::main();
}

fn copy_clipboard(c: char) {
    let mut cmd = Command::new("xclip")
            .arg("-selection")
            .arg("c")
            .stdin(Stdio::piped())
            .spawn()
            .expect("Can't find or open xclip");

    let s = c.to_string();

    let stdin = cmd.stdin.as_mut().expect("Failed to open stdin");
    stdin.write_all(s.as_bytes()).expect("Failed to write to stdin");
}

fn get_char(code: &str) -> Option<char> {
    let num = u32::from_str_radix(code, 16).ok()?;
    char::from_u32(num)
}

fn get_status(code: &str) -> &'static str {
    match u32::from_str_radix(code, 16) {
        Ok(n) => get_name(n),
        Err(_) => "Invalid hex code",
    }
}

