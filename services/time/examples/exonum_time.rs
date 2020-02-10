// Copyright 2020 The Vega Team
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//   http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! This example shows how to run the Vega node with the time service available.
//!
//! Though the time service artifact will be available within node, it should be
//! deployed and instantiated before it will become available for interaction.
//!
//! For details on deploy & init process, see the [runtime docs].
//!
//! [runtime docs]: https://docs.rs/vega/latest/vega/runtime/index.html

use vega_cli::NodeBuilder;
use vega_time::TimeServiceFactory;

fn main() -> Result<(), failure::Error> {
    vega::helpers::init_logger().unwrap();
    NodeBuilder::new()
        .with_service(TimeServiceFactory::default())
        .run()
}
