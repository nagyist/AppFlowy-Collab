use std::thread::sleep;
use std::time::{Duration, Instant};

use crate::core::collab_plugin::EncodedCollab;
use yrs::updates::encoder::Encode;
use yrs::{Doc, ReadTxn, StateVector, Transact, Transaction, TransactionMut};

use crate::core::origin::CollabOrigin;
use crate::error::CollabError;

/// TransactionRetry is a wrapper of Transaction and TransactionMut.
/// It will retry to get a transaction if fail to require the transaction.
/// The default timeout is `2` seconds and the default retry interval is `50` milliseconds.
/// Most of the time, it will get the transaction in the first try.
pub struct TransactionRetry<'a> {
  timeout: Duration,
  doc: &'a Doc,
  start: Instant,
  retry_interval: Duration,
}

impl<'a> TransactionRetry<'a> {
  pub fn new(doc: &'a Doc) -> Self {
    Self {
      timeout: Duration::from_secs(2),
      retry_interval: Duration::from_millis(50),
      doc,
      start: Instant::now(),
    }
  }

  pub fn get_read_txn(&mut self) -> Transaction<'a> {
    while self.start.elapsed() < self.timeout {
      match self.doc.try_transact() {
        Ok(txn) => {
          return txn;
        },
        Err(_e) => {
          sleep(self.retry_interval);
        },
      }
    }
    tracing::warn!("[Txn]: acquire read txn timeout");
    self.doc.transact()
  }

  pub fn try_get_write_txn(&mut self) -> Result<TransactionMut<'a>, CollabError> {
    while self.start.elapsed() < self.timeout {
      match self.doc.try_transact_mut() {
        Ok(txn) => {
          return Ok(txn);
        },
        Err(_e) => {
          sleep(self.retry_interval);
        },
      }
    }
    tracing::warn!("[Txn]: acquire write txn timeout");
    Err(CollabError::AcquiredWriteTxnFail)
  }

  pub fn get_write_txn_with(&mut self, origin: CollabOrigin) -> TransactionMut<'a> {
    while self.start.elapsed() < self.timeout {
      match self.doc.try_transact_mut_with(origin.clone()) {
        Ok(txn) => {
          return txn;
        },
        Err(_e) => {
          sleep(self.retry_interval);
        },
      }
    }
    tracing::warn!("[Txn]: acquire write txn timeout");
    self.doc.transact_mut_with(origin)
  }

  pub fn try_get_write_txn_with(
    &mut self,
    origin: CollabOrigin,
  ) -> Result<TransactionMut<'a>, CollabError> {
    while self.start.elapsed() < self.timeout {
      match self.doc.try_transact_mut_with(origin.clone()) {
        Ok(txn) => {
          return Ok(txn);
        },
        Err(_e) => {
          sleep(self.retry_interval);
        },
      }
    }
    tracing::warn!("[Txn]: acquire write txn timeout");
    Err(CollabError::AcquiredWriteTxnFail)
  }
}

pub trait DocTransactionExtension: Send + Sync {
  fn doc_transaction(&self) -> Transaction;
  fn doc_transaction_mut(&self) -> TransactionMut;

  fn get_encoded_collab_v1(&self) -> EncodedCollab {
    let txn = self.doc_transaction();
    EncodedCollab::new_v1(
      txn.state_vector().encode_v1(),
      txn.encode_state_as_update_v1(&StateVector::default()),
    )
  }

  fn get_encoded_collab_v2(&self) -> EncodedCollab {
    let txn = self.doc_transaction();
    EncodedCollab::new_v2(
      txn.state_vector().encode_v2(),
      txn.encode_state_as_update_v2(&StateVector::default()),
    )
  }
}

impl DocTransactionExtension for Doc {
  fn doc_transaction(&self) -> Transaction {
    self.transact()
  }
  fn doc_transaction_mut(&self) -> TransactionMut {
    self.transact_mut()
  }
}
