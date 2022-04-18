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

#![warn(clippy::all)]

use sqlparser::dialect::MySqlDialect;
use sqlparser::parser::Parser;
fn main() {
    let dialect = MySqlDialect {};
    let statements = Parser::parse_sql(&dialect, "SELECT * FROM USERS u join things t using ( foobar ) where u.col1 = t.col2 and t.col3 = 'XYZZY' and XYZZY in (1111111, 2222222, 3333333)").unwrap();
    println!();
    println!("Unwrapped AST Structure");
    println!("AST: {:?}", statements);
    println!("=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-");
    println!("AST TO RAW SQL QUERY");
    println!("{}", statements[0]);
}
