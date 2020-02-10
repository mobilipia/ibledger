// Copyright 2014 Tyler Neely
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
//

use vega_rocksdb::{Options, DB};
use tempdir::TempDir;

#[test]
fn test_set_num_levels() {
    let temp_dir = TempDir::new("_rust_rocksdb_test_set_num_levels").unwrap();
    let mut opts = Options::default();
    opts.create_if_missing(true);
    opts.set_num_levels(2);
    let db = DB::open(&opts, temp_dir.path()).unwrap();
    drop(db);
}
