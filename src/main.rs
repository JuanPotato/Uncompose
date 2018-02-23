/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

#![feature(plugin)]
#![plugin(phf_macros)]

extern crate gtk;
extern crate phf;

use gtk::prelude::*;
use gtk::{Dialog, Entry};

use std::char;
use std::process::{Command, Stdio};

use std::io::Write;

mod unicode;
use unicode::UNICODE;

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

fn get_name(c: u32) -> &'static str {
    match UNICODE.get(&c) {
        Some(s) => s,
        None => match c {
            0x3400...0x4DB5 => "CJK Ideograph Extension A",
            0x4E00...0x9FD5 => "CJK Ideograph",
            0xAC00...0xD7A3 => "Hangul Syllable",
            0xD800...0xDB7F => "Non Private Use High Surrogate",
            0xDB80...0xDBFF => "Private Use High Surrogate",
            0xDC00...0xDFFF => "Low Surrogate",
            0xE000...0xF8FF => "Private Use",
            0x17000...0x187EC => "Tangut Ideograph",
            0x20000...0x2A6D6 => "CJK Ideograph Extension B",
            0x2A700...0x2B734 => "CJK Ideograph Extension C",
            0x2B740...0x2B81D => "CJK Ideograph Extension D",
            0x2B820...0x2CEA1 => "CJK Ideograph Extension E",
            0x2CEB0...0x2EBE0 => "CJK Ideograph Extension F",
            0xF0000...0xFFFFD => "Plane 15 Private Use",
            0x100000...0x10FFFD => "Plane 16 Private Use",
            _ => "UNKNOWN CHARACTER",
        },
    }
}
