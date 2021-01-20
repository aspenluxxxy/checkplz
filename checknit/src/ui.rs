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

pub(crate) mod checkrain;
pub(crate) mod menu;
pub(crate) mod odysseyrain;
pub(crate) mod power;
pub(crate) mod shell;

use menu::SelectedOption;

pub fn ui() {
	match menu::main_menu() {
		SelectedOption::Checkra1n => checkrain::checkra1n(),
		SelectedOption::OdysseyRa1n => odysseyrain::odysseyra1n(),
		SelectedOption::Shell => shell::shell(),
		SelectedOption::Shutdown => power::power(power::LINUX_REBOOT_CMD_POWER_OFF),
		SelectedOption::Reboot => power::power(power::LINUX_REBOOT_CMD_RESTART),
	}
}
