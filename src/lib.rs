/**
 * Copyright (c) 2016 Alex Maslakov, <http://gildedhonour.com>, <http://alexmaslakov.com>
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 * http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 *
 * For questions and comments about this product, please see the project page at:
 *
 * https://github.com/GildedHonour/atarashii_imap
 *
 */

use std::net::TcpStream;
use std::result;

pub enum Error {
  Append,
  Search,
  Fetch,
  Store,
  Copy,
  UidCommand
}

pub type Result<T> = Result<T, Error>;

fn execute_cmd()

pub fn select_cmd()
pub fn examine_cmd()
pub fn create_cmd()
pub fn delete_cmd()
pub fn rename_cmd()
pub fn subscribe_cmd()
pub fn unsubscribe_cmd()
pub fn list_cmd()
pub fn lsub_cmd()
pub fn status_cmd()
pub fn append_cmd()
pub fn expunge_cmd()
pub fn search_cmd()
pub fn fetch_cmd()
pub fn copy_cmd()
pub fn store_cmd()
pub fn uid_cmd()
pub fn check_cmd()
pub fn close_cmd()

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
