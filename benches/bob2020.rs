// Copyright 2018-2020, Wayfair GmbH
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use criterion::criterion_main;

mod borrow_mut;
mod mem_burn;
mod mpsc_bcast;
mod static_borrow;
mod unchecked_get;

criterion_main! {
    borrow_mut::benches,
    mem_burn::benches,
    static_borrow::benches,
    unchecked_get::benches,
    mpsc_bcast::benches,
}
