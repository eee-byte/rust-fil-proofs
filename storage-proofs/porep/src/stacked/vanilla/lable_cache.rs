use std::convert::TryInto;
use std::marker::PhantomData;

use std::path::{Path, PathBuf};

use anyhow::{bail, ensure, Context};
use byteorder::{ByteOrder, LittleEndian};
use rayon::prelude::*;
use std::io::{Read, Write};
use log::info;
use sha2raw::Sha256;
use storage_proofs_core::{

    drgraph::BASE_DEGREE,
    drgraph::{BucketGraph, Graph},
    error::Result,
    hasher::Hasher,
    parameter_cache::{ensure_parent, ParameterSetMetadata},
    settings,
    util::NODE_SIZE,
};
use std::fs::{OpenOptions, File};
use memmap::{Mmap, MmapMut, MmapOptions};

/// The expansion degree used for Stacked Graphs.
pub const EXP_DEGREE: usize = 8;

pub(crate) const DEGREE: usize = BASE_DEGREE + EXP_DEGREE;

/// u32 = 4 bytes
const NODE_BYTES: usize = 4;


// StackedGraph will hold two different (but related) `ParentCache`,
#[derive(Debug)]
pub struct LableCache {
    /// Disk path for the cache.
    path: PathBuf,
    /// The total number of cache entries.
    mammp_size: usize,
    cache: memmap::MmapMut,
    offset: u64
}

#[inline]
fn prefetch(parents: &[u32], data: &[u8]) {
    for parent in parents {
        let start = *parent as usize * NODE_SIZE;
        let end = start + NODE_SIZE;

        prefetch!(data[start..end].as_ptr() as *const i8);
    }
}

#[inline]
fn read_node<'a>(i: usize, parents: &[u32], data: &'a [u8]) -> &'a [u8] {
    let start = parents[i] as usize * NODE_SIZE;
    let end = start + NODE_SIZE;
    &data[start..end]
}

fn lable_cache_dir_name() -> String {
    settings::SETTINGS
        .lock()
        .expect("parent_cache settings lock failure")
        .lable_cache
        .clone()
}

impl LableCache{
    //Generates a new cache and stores it on disk.
    pub fn generate(path: PathBuf, len: u64) -> Result<Self> {
        info!("lable cache: generating {}", path.display());
        let mut file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(&path)
            .with_context(|| format!("could not generate path={:?}", path.display()))?;
        file.set_len(len)?;
        let mut mmap_data = unsafe { MmapMut::map_mut(&file)? };
        info!("lable cache: generated");

        Ok(LableCache {
            path: path,
            mammp_size: len as usize,
            cache: mmap_data,
            offset: 0
        })
    }

    pub fn shfit(&mut self, offset: u64) {
        self.offset = offset;
    }
    // pub fn read(&self, cache_parents: &[u32], exp_data: &mut [&[u8]]) {
    //
    //     //assert!(node >= self.offset, "node not in cache");
    //     let mut data = [0u8; NODE_SIZE];
    //     let mut node_data = [0u8; EXP_DEGREE];
    //     for (n, parents) in cache_parents {
    //         data.clone_from_slice(&self.cache[parents..parents+NODE_SIZE]);
    //         node_data[n] = &data
    //     }
    //     exp_data.copy_from_slice(&node_data)
    // }

    pub fn update_cache_file(&mut self, data: &[u8]) -> Result<()> {
        (&mut self.cache[self.offset as usize..]).write_all(&data)?;
        self.offset += data.len() as u64;
        self.cache.flush()?;

        Ok(())
    }
}
