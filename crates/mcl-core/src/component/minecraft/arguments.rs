use super::{rules, version};
use crate::component::Argument;

#[must_use]
pub fn convert(arguments: Vec<version::Argument>) -> Vec<Argument> {
	let mut active_arguments = Vec::new();
	for argument in arguments {
		match argument {
			version::Argument::Basic(value) => {
				active_arguments.push(value);
			},
			version::Argument::Conditional { rules, value } => {
				if rules::check(&rules) {
					match value {
						version::ArgumentValue::Single(value) => active_arguments.push(value),
						version::ArgumentValue::Multiple(values) => {
							values
								.into_iter()
								.for_each(|value| active_arguments.push(value));
						},
					}
				}
			},
		}
	}

	let mut out = Vec::new();
	let mut iter = active_arguments.into_iter().peekable();
	while let Some(argument) = iter.next() {
		if let Some(first_char) = argument.chars().next() {
			if let '$' = first_char {
				out.push(Argument::Single(argument));
			} else {
				if let Some((argument, value)) = argument.split_once('=') {
					out.push(Argument::Eq(argument.to_owned(), value.to_owned()));
					continue;
				} else if let Some(next_argument) = iter.peek() {
					if let Some('$') = next_argument.chars().next() {
						out.push(Argument::Pair(argument, iter.next().unwrap()));
						continue;
					}
				}
				out.push(Argument::Single(argument));
			}
		}
	}

	out
}
