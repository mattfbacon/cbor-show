#![deny(
	absolute_paths_not_starting_with_crate,
	future_incompatible,
	keyword_idents,
	macro_use_extern_crate,
	meta_variable_misuse,
	missing_abi,
	missing_copy_implementations,
	non_ascii_idents,
	nonstandard_style,
	noop_method_call,
	pointer_structural_match,
	private_in_public,
	rust_2018_idioms,
	unused_qualifications
)]
#![warn(clippy::pedantic)]
#![forbid(unsafe_code)]

use ciborium::value::Value;

fn main() {
	let reader = std::io::stdin().lock();
	let value = ciborium::de::from_reader(reader).expect("invalid input");
	print(&value);
}

fn print(value: &Value) {
	helper(value, 0);
	println!();
}

fn indent(level: usize) {
	for _ in 0..level {
		print!("  ");
	}
}

fn helper(value: &Value, level: usize) {
	match value {
		Value::Integer(integer) => print!("int({})", i128::from(*integer)),
		Value::Bytes(bytes) => {
			if let Ok(utf8) = std::str::from_utf8(bytes) {
				print!("bytes(s{utf8:?})");
			} else {
				print!("bytes(h{bytes:?})");
			}
		}
		Value::Float(float) => print!("float({float})"),
		Value::Text(text) => print!("text({text:?})"),
		Value::Bool(v) => print!("bool({v:?})"),
		Value::Null => print!("null"),
		Value::Tag(tag, inner) => {
			println!("tag {tag} (");

			indent(level + 1);
			helper(inner, level + 1);
			println!();

			indent(level);
			print!(")");
		}
		Value::Array(values) => {
			println!("array [");
			for value in values {
				indent(level + 1);
				helper(value, level + 1);
				println!(",");
			}

			indent(level);
			print!("]");
		}
		Value::Map(pairs) => {
			println!("map {{");

			for (key, value) in pairs {
				indent(level + 1);
				helper(key, level + 1);
				print!(" = ");
				helper(value, level + 1);
				println!(",");
			}

			indent(level);
			print!("}}");
		}
		_ => print!("unknown"),
	}
}
