use crate::{error::Result, Error, ErrorValue};

use super::{context::Context, expression::Expression};

#[derive(Debug)]
pub enum Value {
	U32(u32),
	String(String),
}

impl Value {
	/// Representation of a u32-kind unknown value, used as the default value for
	/// unspecified parameters. It is treated as an always-successful condition.
	pub const UNKNOWN: u32 = u32::MAX;
}

impl TryFrom<Value> for u32 {
	type Error = Error;

	fn try_from(value: Value) -> Result<Self, Self::Error> {
		match value {
			Value::U32(value) => Ok(value),
			Value::String(_) => Err(Error::Invalid(
				ErrorValue::Other("SeString".into()),
				"cannot resolve string value to u32".into(),
			)),
		}
	}
}

impl TryFrom<Value> for String {
	type Error = Error;

	fn try_from(value: Value) -> Result<Self, Self::Error> {
		match value {
			Value::String(value) => Ok(value),
			Value::U32(_) => Err(Error::Invalid(
				ErrorValue::Other("SeString".into()),
				"cannot resolve u32 value to string".into(),
			)),
		}
	}
}

pub trait ArgumentExt {
	fn resolve<T>(&self, context: &mut Context) -> Result<T>
	where
		T: FromArguments;
}

impl ArgumentExt for &[Expression] {
	fn resolve<T>(&self, context: &mut Context) -> Result<T>
	where
		T: FromArguments,
	{
		T::resolve(self, context)
	}
}

pub trait FromArguments: Sized {
	fn resolve(arguments: &[Expression], context: &mut Context) -> Result<Self>;
}

macro_rules! tuple_impl {
	($arg:ident $(, $args:ident)*) => {
		#[allow(non_camel_case_types)]
		impl<$arg: TryFrom<Value, Error = Error>, $($args: TryFrom<Value, Error = Error>),*> FromArguments for ($arg, $($args),*) {
			fn resolve(arguments: &[Expression], context: &mut Context) -> Result<Self> {
				let iter = &mut arguments.iter();
				Ok((
					resolve_argument::<$arg>(iter, context)?,
					$(resolve_argument::<$args>(iter, context)?),*
				))
			}
		}
	};

	() => {};
}

tuple_impl!(arg1, arg2, arg3);

fn resolve_argument<'a, T>(
	iter: &mut impl Iterator<Item = &'a Expression>,
	context: &mut Context,
) -> Result<T>
where
	T: TryFrom<Value, Error = Error>,
{
	let expression = iter.next().ok_or_else(|| {
		Error::Invalid(
			ErrorValue::Other("SeString".into()),
			"insufficient arguments".into(),
		)
	})?;
	expression.resolve(context)
}
