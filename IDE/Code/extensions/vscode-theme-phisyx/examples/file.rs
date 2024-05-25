/*
 * Un commentaire.
 */
// Single line
/* Multi
 * Line
 */
/**
 * Documentation
 */
/// Documentation

pub mod my_module;

mod my_module2 {
    mod my_module3;
}

use std::fmt;

use my_crate::my_module::MyItem;

pub use crate::my_module::*;
use self::my_module::{ MyItem1, MyItem2 };
use super::my_module::{ MyItem1, MyItem2 };

"Hello";
"hello {world:#?}";

1;
2_000;
0x12;
0b1;
0e2;
0.1;

+ - / * | & "" '' () {} [] ^ ? ! : ; - _ $ % , @ # 

async fn 

expr.await

impl X for Y {}

'root: loop {
    'inner: loop {
        if x {
            continue 'root;
        }

        if y {
            break: 'inner;
        }
    }

    if z {
        break;
    }
} 

while true {} 

for x in 0.. {}

use item;
use item::x;

mod item;
mod item {}

let ident
let mut ident
const IDENT

pub
pub(crate)
pub(super)
pub(self)
Self

// ---

let x1 = 1;
let x2 = export1 != export2;
let x3 = 0 > 1 && 0 < 1;

const CONSTANT: char = 'a';

println!();
my_crate::println!();
println();

struct Struct<'a>
{
    field: &'a str,
    time: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug)]
#[derive(serde::Deserialize, serde::Serialize)]
enum Enum
{
	#[serde(skip_serializing)]
    Variant(
		/// Doc
        // Comment
        /*
        Comment
        */
		Type,
	),
}

impl Enum
{
    pub fn to_char(&self) -> char
	{
		match self {
			| Self::Variant(..) => 'v',
		}
	}

    pub fn set(&mut self, new: Self)
	{
        *self = new;
	}

    async fn fetch(&self, url: impl AsRef<str>) -> impl ToString
    {
        let req = get(url).await;
        return req.text().await;
    }
}

#[rustfmt::skip]
pub trait Trait
    : Constraints
    + std::fmt::Debug
{
    type Type: TraitInterface;

	type OwnedID: ToString + Clone;
    type RefID<'a>
		: ?Sized + 'a
		+ ToOwned<Owned = Self::OwnedID>
        ;

    fn set_type(
		&mut self,
		ty: <Self::Type as TraitInterface>::AnotherType,
	);
}


pub fn validate<'de, D>(de: D) -> Result<String, D::Error>
where
	D: serde::Deserializer<'de>,
{
	use serde::Deserialize;
	let s: String = String::deserialize(de)?;

	if s.is_empty() || s.len() > 30 && s.len() < 1024 {
		return Err(serde::de::Error::custom(format!("Invalid")));
	}

	Ok(s)
}

#[macro_export]
macro_rules! macro_name {
	(
		$(
		$(#[$doc_struct:meta])*
		struct $command:ident {
			$(
				$(#[$doc_field:meta])*
				$field:ident : $ty:ty,
			)*
		}
		)*
	) => {
        
$crate::paste::paste! { $(

	#[derive(Debug)]
	#[derive(serde::Serialize, serde::Deserialize)]
	$(#[$doc_struct])*
	pub struct [ <$command:camel Form> ]
	{
		$(
			$(#[$doc_field])*
			pub $field: $ty,
		)*
	}

)* }
    }
}