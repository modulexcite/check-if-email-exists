// check-if-email-exists
// Copyright (C) 2018-2020 Amaury Martiny

// check-if-email-exists is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// check-if-email-exists is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with check-if-email-exists.  If not, see <http://www.gnu.org/licenses/>.

use crate::util::ser_with_display::ser_with_display;
use async_smtp::{error::Error as AsyncSmtpError, EmailAddress};
use serde::Serialize;
use std::str::FromStr;

/// Syntax information after parsing an email address
#[derive(Debug, PartialEq, Serialize)]
pub struct SyntaxDetails {
	/// The email address as a async_smtp EmailAddress
	pub address: EmailAddress,
	/// The domain name, after "@"
	pub domain: String,
	/// Does the email have a valid syntax?
	pub is_valid_syntax: bool,
	/// The username, before "@"
	pub username: String,
}

#[derive(Debug, Serialize)]
#[serde(tag = "type", content = "message")]
pub enum SyntaxError {
	#[serde(serialize_with = "ser_with_display")]
	SyntaxError(AsyncSmtpError),
}

impl PartialEq for SyntaxError {
	fn eq(&self, other: &Self) -> bool {
		match (self, other) {
			(
				SyntaxError::SyntaxError(AsyncSmtpError::InvalidEmailAddress),
				SyntaxError::SyntaxError(AsyncSmtpError::InvalidEmailAddress),
			) => true,
			_ => false,
		}
	}
}

/// From an `email_address` string, compute syntax information about it, such as
/// username and domain
pub fn address_syntax(email_address: &str) -> Result<SyntaxDetails, SyntaxError> {
	let email_address = match EmailAddress::from_str(email_address) {
		Ok(m) => m,
		Err(error) => return Err(SyntaxError::SyntaxError(error)),
	};

	let iter: &str = email_address.as_ref();
	let mut iter = iter.split('@');
	let username = iter
		.next()
		.expect("We checked above that email is valid. qed.")
		.into();
	let domain = iter
		.next()
		.expect("We checked above that email is valid. qed.")
		.into();

	let address_details = SyntaxDetails {
		address: email_address,
		domain,
		is_valid_syntax: true,
		username,
	};

	Ok(address_details)
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn should_return_error_for_invalid_email() {
		assert_eq!(
			address_syntax("foo"),
			Err(SyntaxError::SyntaxError(
				AsyncSmtpError::InvalidEmailAddress
			))
		);
	}

	#[test]
	fn should_return_error_for_invalid_email_with_at() {
		assert_eq!(
			address_syntax("foo@bar"),
			Err(SyntaxError::SyntaxError(
				AsyncSmtpError::InvalidEmailAddress
			))
		);
	}

	#[test]
	fn should_work_for_valid_email() {
		assert_eq!(
			address_syntax("foo@bar.com"),
			Ok(SyntaxDetails {
				address: EmailAddress::new("foo@bar.com".into()).unwrap(),
				domain: "bar.com".into(),
				is_valid_syntax: true,
				username: "foo".into(),
			})
		);
	}
}
