// Copyright 2021 Parity Technologies (UK) Ltd.
// This file is part of polkadot-rewards.

// polkadot-rewards is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// polkadot-rewards is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with polkadot-rewards.  If not, see <http://www.gnu.org/licenses/>.

//! Wrapper around calls to Subscans API

use crate::{
    cli::{App, Network},
    primitives::{ApiResponse, List, Price, Record, Reward},
};
use anyhow::Error;
use argh::FromArgs;

const POLKADOT_ENDPOINT: &str = "https://polkadot.subscan.io/api/";
const KUSAMA_ENDPOINT: &str = "https://kusama.subscan.io/api/";

fn get_endpoint(network: &Network) -> ureq::Request {
    match network {
        Network::Polkadot => ureq::post(POLKADOT_ENDPOINT),
        Network::Kusama => ureq::post(KUSAMA_ENDPOINT),
    }
}

// TODO: Rate limit these requests so we don't end up trying to DoS subscan.

pub struct Api<'a> {
    app: &'a App,
}

impl<'a> Api<'a> {
    pub fn price(&self, time: usize) -> Result<Price, Error> {
        let req = get_endpoint(&self.app.network);

        let mut buf: Vec<u8> = Vec::with_capacity(32);
        let _ = json::object! { "time": time }.write(&mut buf)?;

        // we don't use ureq `into_json` because we're cool and use `miniserde` 😎
        let price = req
            .set("Content-Type", "application/json")
            .send(buf.as_slice())?
            .into_string()?;
        let price: ApiResponse<Price> = miniserde::json::from_str(&price)?;
        Ok(price.consume())
    }

    pub fn rewards(&self, address: String, page: usize) -> Result<List<Reward>, Error> {
        let req = get_endpoint(&self.app.network);

        let mut buf: Vec<u8> = Vec::with_capacity(128);
        let _ = json::object! {
            "address": self.app.address.as_str(),
            "page": page,
            "row": 10 // this is how many items the api will return. Not sure why 'row' was chosen, but it kindof makes sense i guess 🤷
        }
        .write(&mut buf)?;

        let rewards = req
            .set("Content-Type", "application/json")
            .send(buf.as_slice())?
            .into_string()?;
        let rewards: ApiResponse<List<Reward>> = miniserde::json::from_str(&rewards)?;
        Ok(rewards.consume())
    }
}
