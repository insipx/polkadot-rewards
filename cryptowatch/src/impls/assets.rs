use crate::types::*;
use std::borrow::Cow;
impl<'a> Asset<'a> {
	pub fn into_owned(self) -> Asset<'static> {
		let Asset { id, sid, symbol, name, fiat, route } = self;
		Asset {
			id,
			sid: Cow::Owned(sid.into_owned()),
			symbol: Cow::Owned(symbol.into_owned()),
			name: Cow::Owned(name.into_owned()),
			fiat,
			route,
		}
	}
}
