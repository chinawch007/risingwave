// Copyright 2022 Singularity Data
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

mod top_n_bottom_n_state;
mod top_n_state;
pub use top_n_bottom_n_state::ManagedTopNBottomNState;
pub use top_n_state::ManagedTopNState;

pub mod variants {
    pub const TOP_N_MIN: usize = 0;
    pub const TOP_N_MAX: usize = 1;
}
