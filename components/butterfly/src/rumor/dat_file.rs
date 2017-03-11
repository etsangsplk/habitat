// Copyright (c) 2017 Chef Software Inc. and/or applicable contributors
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

use std::fs::{self, File};
use std::io::{self, Write};
use std::ops::Deref;
use std::path::{Path, PathBuf};

use protobuf::Message;

use error::{Result, Error};
use message;
use rumor::{Rumor, RumorStore};
use server::Server;

/// A versioned binary file containing rumors exchanged by the butterfly server which have
/// been periodically persisted to disk.
///
/// The contents of the DatFile can be used to rebuild and resume a butterfly server connection
/// if it has been destroyed or lost.
///
#[derive(Debug)]
pub struct DatFile(PathBuf);

/// Describes contents and structure of dat file.
///
/// The information in this header is used to enable IO seeking operations on a binary dat
/// file containing rumors exchanged by the butterfly server.
pub struct Header(message::dat::DatHeader);

impl Header {
    fn new(server: &Server) -> Self {
        let mut header = message::dat::DatHeader::new();
        header.set_services(server.service_store.len() as u64);
        header.set_service_config(server.service_config_store.len() as u64);
        header.set_service_files(server.service_file_store.len() as u64);
        header.set_elections(server.election_store.len() as u64);
        header.set_updates(server.update_store.len() as u64);
        Header(header)
    }
}

impl Deref for Header {
    type Target = message::dat::DatHeader;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DatFile {
    pub fn new<T: AsRef<Path>>(data_path: T) -> Self {
        DatFile(data_path.as_ref().join("rumors.dat"))
    }

    fn byte_len(bytes: &[u8]) -> [u8; 4] {
        let len = bytes.len();
        [((len >> 24) & 0xff) as u8,
         ((len >> 16) & 0xff) as u8,
         ((len >> 8) & 0xff) as u8,
         (len & 0xff) as u8]
    }

    fn write_header(file: &File, header: &Header) -> io::Result<usize> {
        let mut total = 0;
        // JW TODO: handle IO error
        let bytes = header.write_to_bytes().unwrap();
        total += file.write(&Self::byte_len(&bytes))?;
        total += file.write(&bytes)?;
        Ok(total)
    }

    fn write_rumor_store<T>(file: &File, store: &RumorStore<T>) -> io::Result<usize>
        where T: Rumor
    {
        let mut total = 0;
        for rumor in store.iter() {
            total += Self::write_rumor(file, rumor)?;
        }
        Ok(total)
    }

    fn write_rumor<T: Rumor>(file: &File, rumor: T) -> io::Result<usize> {
        let mut total = 0;
        // JW TODO: handle IO error
        let bytes = rumor.write_to_bytes().unwrap();
        total += file.write(&Self::byte_len(&bytes))?;
        total += file.write(&bytes)?;
        Ok(total)
    }

    pub fn init(&self) -> Result<()> {
        let data_path = self.0.parent().unwrap();
        if let Some(err) = fs::create_dir_all(data_path).err() {
            return Err(Error::BadDataPath(data_path.to_path_buf(), err));
        }
        // validate the dat file exists or create it or what?
        Ok(())
    }

    pub fn path(&self) -> &Path {
        &self.0
    }

    pub fn write(&self, server: &Server) -> Result<usize> {
        let mut total = 0;
        // JW TODO: handle IO error
        let mut file = File::open(self.0).unwrap();
        // JW TODO: handle IO error
        file.set_len(0);
        let header = Header::new(server);
        // JW TODO: handle IO error
        total += Self::write_header(&file, &header).unwrap();
        total += Self::write_rumor_store(&file, &server.service_store).unwrap();
        total += Self::write_rumor_store(&file, &server.service_config_store).unwrap();
        total += Self::write_rumor_store(&file, &server.service_file_store).unwrap();
        total += Self::write_rumor_store(&file, &server.election_store).unwrap();
        total += Self::write_rumor_store(&file, &server.update_store).unwrap();
        Ok(total)
    }
}
