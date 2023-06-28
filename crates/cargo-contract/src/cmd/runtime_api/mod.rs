// Copyright 2018-2020 Parity Technologies (UK) Ltd.
// This file is part of cargo-contract.
//
// cargo-contract is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// cargo-contract is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with cargo-contract.  If not, see <http://www.gnu.org/licenses/>.

#![allow(clippy::too_many_arguments)]

#[subxt::subxt(
    runtime_metadata_path = "src/cmd/runtime_api/academy_pow.scale",
    substitute_type(
        path = "sp_weights::weight_v2::Weight",
        with = "::subxt::utils::Static<::sp_weights::Weight>"
    ),
    // This was just me grasping at a potential solution to the compile errors.
    // substitute_type(
    //     path = "contract_transcode::AccountId20",
    //     with = "::account::AccountId20"
    // )
)]
pub mod api {}
