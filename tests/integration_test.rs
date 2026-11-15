use sigmaos_betteroffice_spreadsheets::{Workbook, Sheet, Cell, CellRef, CellValue};

#[test]
fn workbook_integration_aaa() {
    let mut w = Workbook::new("My Workbook");
    w.add_sheet(Sheet::new("Sheet1"));
    assert_eq!(w.sheet_count(), 1);
}
