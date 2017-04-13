// Copyright 2015-2017 Parity Technologies (UK) Ltd.
// This file is part of Parity.

// Parity is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Parity is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Parity.  If not, see <http://www.gnu.org/licenses/>.

//! SecretStore-specific rpc interface.

use jsonrpc_core::Error;

use v1::types::{H160, Bytes};

build_rpc_trait! {
	/// Parity-specific rpc interface.
	pub trait SecretStore {
		/// Encrypt data with key, received from secret store.
		/// Arguments: `account`, `key`, `data`.
		#[rpc(name = "secretstore_encrypt")]
		fn encrypt(&self, H160, Bytes, Bytes) -> Result<Bytes, Error>;

		/// Decrypt data with key, received from secret store.
		/// Arguments: `account`, `key`, `data`.
		#[rpc(name = "secretstore_decrypt")]
		fn decrypt(&self, H160, Bytes, Bytes) -> Result<Bytes, Error>;

		/// Decrypt data with shadow key, received from secret store.
		/// Arguments: `account`, `key`, `data`.
		#[rpc(name = "secretstore_shadowDecrypt")]
		fn shadow_decrypt(&self, H160, Bytes, Bytes) -> Result<Bytes, Error>;
	}
}
