//! Explicit shell-owned budget shared by every reader in one readiness request.
use std::{cell::Cell, io, rc::Rc};

pub(crate) const MAX_REPORT_BYTES: u64 = 64 * 1024 * 1024;
pub(crate) const MAX_REPORT_ENTRIES: usize = 64 * 1024;

#[derive(Clone)]
pub(crate) struct ObservationBudget {
    bytes: Rc<Cell<u64>>,
    entries: Rc<Cell<usize>>,
}

impl ObservationBudget {
    pub(crate) fn new() -> Self {
        Self::with_limits(MAX_REPORT_BYTES, MAX_REPORT_ENTRIES)
    }

    pub(crate) fn with_limits(bytes: u64, entries: usize) -> Self {
        Self {
            bytes: Rc::new(Cell::new(bytes)),
            entries: Rc::new(Cell::new(entries)),
        }
    }

    pub(crate) fn read_limit(&self, local_limit: u64) -> u64 {
        local_limit.min(self.bytes.get())
    }

    pub(crate) fn charge_bytes(&self, bytes: u64) -> io::Result<()> {
        let remaining = self
            .bytes
            .get()
            .checked_sub(bytes)
            .ok_or_else(budget_error)?;
        self.bytes.set(remaining);
        Ok(())
    }

    pub(crate) fn charge_entry(&self) -> io::Result<()> {
        let remaining = self.entries.get().checked_sub(1).ok_or_else(budget_error)?;
        self.entries.set(remaining);
        Ok(())
    }
}

fn budget_error() -> io::Error {
    io::Error::new(
        io::ErrorKind::InvalidData,
        "readiness report aggregate I/O budget exceeded",
    )
}
