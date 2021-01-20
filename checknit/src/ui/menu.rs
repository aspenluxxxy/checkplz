/*
	checknit, an init script + simple UI for  checkplz

	Copyright (C) 2021  aspen

	This file is part of checknit.

	checknit is free software: you can redistribute it and/or modify
	it under the terms of the GNU General Public License as published by
	the Free Software Foundation, either version 2 of the License, or
	(at your option) any later version.

	checknit is distributed in the hope that it will be useful,
	but WITHOUT ANY WARRANTY; without even the implied warranty of
	MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
	GNU General Public License for more details.

	You should have received a copy of the GNU General Public License
	along with checknit.  If not, see <http://www.gnu.org/licenses/>.
*/

use cursive::{
	align::HAlign,
	traits::*,
	views::{Dialog, SelectView},
	Cursive,
};
use std::sync::atomic::{AtomicU8, Ordering};

static QUIT_ACTION: AtomicU8 = AtomicU8::new(SelectedOption::Shutdown as u8);

#[repr(u8)]
#[derive(Clone, Copy)]
pub enum SelectedOption {
	Checkra1n = 1,
	OdysseyRa1n,
	Shell,
	Shutdown,
	Reboot,
}

pub fn main_menu() -> SelectedOption {
	let mut siv = cursive::default();

	let mut select = SelectView::new().h_align(HAlign::Center).autojump();

	select.add_item("jailbreak with checkra1n", SelectedOption::Checkra1n);
	select.add_item("bootstrap odysseyra1n", SelectedOption::OdysseyRa1n);
	select.add_item("shell (advanced)", SelectedOption::Shell);
	select.add_item("shutdown", SelectedOption::Shutdown);
	select.add_item("reboot", SelectedOption::Reboot);
	select.set_on_submit(select_option);

	siv.add_layer(Dialog::around(select.scrollable()).title("checknit menu"));

	siv.run();
	std::mem::drop(siv);

	// Can I do this better? Very much, yes.
	// Do I need to use transmute? Nope.
	// But I'm lazy and QUIT_ACTION will never be invalid, so whatever.
	unsafe { std::mem::transmute(QUIT_ACTION.load(Ordering::SeqCst)) }
}

pub fn select_option(siv: &mut Cursive, opt: &SelectedOption) {
	QUIT_ACTION.store(*opt as u8, Ordering::SeqCst);
	siv.quit();
}
