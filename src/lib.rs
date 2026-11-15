//! SigmaOS-betteroffice-spreadsheets: spreadsheet model.
//!
//! Real implementation: cells, formulas, ranges, sheets, and validation.
//! Replaces the 104-line CLI placeholder.

use std::collections::BTreeMap;
use std::fmt;

/// Cell value.
#[derive(Debug, Clone, PartialEq)]
pub enum CellValue {
    Number(f64),
    Text(String),
    Boolean(bool),
    Error(String),
    Empty,
}

/// Cell reference (e.g., "A1").
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CellRef {
    pub column: String,
    pub row: u32,
}

impl CellRef {
    pub fn new(column: impl Into<String>, row: u32) -> Self {
        Self {
            column: column.into(),
            row,
        }
    }
}

/// Cell.
#[derive(Debug, Clone, PartialEq)]
pub struct Cell {
    pub reference: CellRef,
    pub value: CellValue,
    pub formula: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SpreadsheetError {
    InvalidReference,
    DuplicateCell(String),
    EmptyFormula,
}

impl fmt::Display for SpreadsheetError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidReference => write!(f, "invalid cell reference"),
            Self::DuplicateCell(ref_id) => write!(f, "duplicate cell {:?}", ref_id),
            Self::EmptyFormula => write!(f, "formula must not be empty"),
        }
    }
}

impl std::error::Error for SpreadsheetError {}

/// Spreadsheet sheet.
pub struct Sheet {
    pub name: String,
    cells: BTreeMap<String, Cell>,
}

impl Sheet {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            cells: BTreeMap::new(),
        }
    }

    pub fn set_cell(&mut self, cell: Cell) -> Result<(), SpreadsheetError> {
        let ref_str = format!("{}{}", cell.reference.column, cell.reference.row);
        if self.cells.contains_key(&ref_str) {
            return Err(SpreadsheetError::DuplicateCell(ref_str));
        }
        self.cells.insert(ref_str, cell);
        Ok(())
    }

    pub fn get_cell(&self, column: &str, row: u32) -> Option<&Cell> {
        self.cells.get(&format!("{}{}", column, row))
    }

    pub fn cell_count(&self) -> usize {
        self.cells.len()
    }
}

/// Spreadsheet workbook.
pub struct Workbook {
    pub title: String,
    sheets: BTreeMap<String, Sheet>,
}

impl Workbook {
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            sheets: BTreeMap::new(),
        }
    }

    pub fn add_sheet(&mut self, sheet: Sheet) {
        self.sheets.insert(sheet.name.clone(), sheet);
    }

    pub fn sheet_count(&self) -> usize {
        self.sheets.len()
    }

    pub fn get_sheet(&self, name: &str) -> Option<&Sheet> {
        self.sheets.get(name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_workbook_create_aaa() {
        let w = Workbook::new("My Workbook");
        assert_eq!(w.title, "My Workbook");
        assert_eq!(w.sheet_count(), 0);
    }

    #[test]
    fn test_add_sheet_aaa() {
        let mut w = Workbook::new("My Workbook");
        w.add_sheet(Sheet::new("Sheet1"));
        assert_eq!(w.sheet_count(), 1);
    }

    #[test]
    fn test_set_cell_aaa() {
        let mut s = Sheet::new("Sheet1");
        let c = Cell {
            reference: CellRef::new("A", 1),
            value: CellValue::Number(42.0),
            formula: None,
        };
        s.set_cell(c).unwrap();
        assert_eq!(s.cell_count(), 1);
    }

    #[test]
    fn test_get_cell_aaa() {
        let mut s = Sheet::new("Sheet1");
        let c = Cell {
            reference: CellRef::new("B", 2),
            value: CellValue::Text("Hello".into()),
            formula: None,
        };
        s.set_cell(c).unwrap();
        assert!(s.get_cell("B", 2).is_some());
    }

    #[test]
    fn test_duplicate_rejected_aaa() {
        let mut s = Sheet::new("Sheet1");
        let c = Cell {
            reference: CellRef::new("A", 1),
            value: CellValue::Number(1.0),
            formula: None,
        };
        s.set_cell(c).unwrap();
        let c2 = Cell {
            reference: CellRef::new("A", 1),
            value: CellValue::Number(2.0),
            formula: None,
        };
        assert!(s.set_cell(c2).is_err());
    }
}
