use anyhow::{Context, Result};
use chrono::{DateTime, NaiveDateTime, Utc};
use rocksdb::{
    ColumnFamily, ColumnFamilyDescriptor, DBIteratorWithThreadMode, Direction, Options, DB,
};
use serde::de::DeserializeOwned;
use std::{marker::PhantomData, path::Path, sync::Arc, time::Duration};
use tokio::time;
use tracing::{error, info};

const COLUMN_FAMILY_NAMES: [&str; 1] = ["column"];
const ONE_HOUR: u64 = 60 * 60;

pub struct Data {}

pub struct DbOptions {
    max_open_files: i32,
    max_mb_of_level_base: u64,
}

impl Default for DbOptions {
    fn default() -> Self {
        Self {
            max_open_files: 8000,
            max_mb_of_level_base: 512,
        }
    }
}

impl DbOptions {
    #[must_use]
    pub fn new(max_open_files: i32, max_mb_of_level_base: u64) -> Self {
        DbOptions {
            max_open_files,
            max_mb_of_level_base,
        }
    }
}

#[derive(Clone)]
pub struct Store {
    db: Arc<DB>,
}

impl Store {
    /// Opens the Store at the given path.
    ///
    /// # Errors
    ///
    /// Returns an error if a database open fails.
    pub fn open(path: &Path, db_options: &DbOptions) -> Result<Store> {
        let (db_opts, cf_opts) = rocksdb_options(db_options);
        let mut cfs_name: Vec<&str> = Vec::with_capacity(COLUMN_FAMILY_NAMES.len());
        cfs_name.extend(COLUMN_FAMILY_NAMES);

        let cfs = cfs_name
            .into_iter()
            .map(|name| ColumnFamilyDescriptor::new(name, cf_opts.clone()));

        let db = DB::open_cf_descriptors(&db_opts, path, cfs).context("cannot open store")?;
        Ok(Store { db: Arc::new(db) })
    }

    pub fn event(&self) -> Result<EventStore<Data>> {
        let cf = self
            .db
            .cf_handle("column")
            .context("cannot access column family")?;
        Ok(EventStore::new(&self.db, cf))
    }
}

pub struct EventStore<'db, T> {
    db: &'db DB,
    cf: &'db ColumnFamily,
    phantom: PhantomData<T>,
}

// RocksDB must manage thread safety for `ColumnFamily`.
// See rust-rocksdb/rust-rocksdb#407.
unsafe impl<'db, T> Send for EventStore<'db, T> {}

impl<'db, T> EventStore<'db, T> {
    fn new(db: &'db DB, cf: &'db ColumnFamily) -> EventStore<'db, T> {
        EventStore {
            db,
            cf,
            phantom: PhantomData,
        }
    }

    pub fn append(&self, key: &[u8], raw_event: &[u8]) -> Result<()> {
        self.db.put_cf(self.cf, key, raw_event)?;
        Ok(())
    }

    pub fn flush(&self) -> Result<()> {
        self.db.flush_wal(true)?;
        Ok(())
    }
}

impl<'db, T: DeserializeOwned> EventStore<'db, T> {
    pub fn iter_forward(&self, from: &[u8]) -> Iter<'db, T> {
        Iter::new(self.db.iterator_cf(
            self.cf,
            rocksdb::IteratorMode::From(from, Direction::Forward),
        ))
    }
}

pub type KeyValue<T> = (Box<[u8]>, T);

pub struct Iter<'d, T> {
    inner: DBIteratorWithThreadMode<'d, DB>,
    phantom: PhantomData<T>,
}

impl<'d, T> Iter<'d, T> {
    pub fn new(inner: DBIteratorWithThreadMode<'d, DB>) -> Self {
        Self {
            inner,
            phantom: PhantomData,
        }
    }
}

impl<'d, T> Iterator for Iter<'d, T>
where
    T: DeserializeOwned,
{
    type Item = anyhow::Result<KeyValue<T>>;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().map(|item| match item {
            Ok((key, value)) => bincode::deserialize::<T>(&value)
                .map(|value| (key, value))
                .map_err(Into::into),
            Err(e) => Err(e.into()),
        })
    }
}

fn rocksdb_options(db_options: &DbOptions) -> (Options, Options) {
    let max_bytes = db_options.max_mb_of_level_base * 1024 * 1024;
    let mut db_opts = Options::default();
    db_opts.create_if_missing(true);
    db_opts.create_missing_column_families(true);
    db_opts.set_max_open_files(db_options.max_open_files);
    db_opts.set_keep_log_file_num(10);
    db_opts.set_stats_dump_period_sec(3600);
    db_opts.set_max_total_wal_size(max_bytes);
    db_opts.set_manual_wal_flush(true);

    let mut cf_opts = Options::default();
    cf_opts.set_write_buffer_size((max_bytes / 4).try_into().expect("u64 to usize"));
    cf_opts.set_max_bytes_for_level_base(max_bytes);
    cf_opts.set_target_file_size_base(max_bytes / 10);
    cf_opts.set_target_file_size_multiplier(10);
    cf_opts.set_compression_type(rocksdb::DBCompressionType::Lz4);
    cf_opts.set_bottommost_compression_type(rocksdb::DBCompressionType::Zstd);
    cf_opts.set_bottommost_zstd_max_train_bytes(0, true);

    (db_opts, cf_opts)
}

/// Every hour, delete all data older than a certain amount of time (the delete threshold is taken from the settings).
///
/// # Errors
///
/// Returns an error if a database open fails.
pub async fn retain_periodically(retention_period: Duration, store: Store) -> Result<()> {
    let mut itv = time::interval(time::Duration::from_secs(ONE_HOUR));
    let retention_duration = i64::try_from(retention_period.as_nanos())?;
    let from = DateTime::<Utc>::from_utc(
        NaiveDateTime::from_timestamp_opt(61, 0).expect("valid time"),
        Utc,
    )
    .timestamp_nanos()
    .to_be_bytes();

    loop {
        itv.tick().await;
        let event_store = store.event()?;
        let standard_duration = Utc::now().timestamp_nanos() - retention_duration;
        let to = standard_duration.to_be_bytes();
        match event_store.db.delete_range_cf(event_store.cf, from, to) {
            Ok(_) => {
                info!("event db deleted");
                event_store
                    .db
                    .delete_file_in_range_cf(event_store.cf, from, to)?;
            }
            Err(e) => {
                error!("delete all error: {e}");
            }
        }
    }
}
