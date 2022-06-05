
#[derive(Clone, Copy)]
pub enum CellType {
    Ref(usize),
    Val(i32),
}

#[derive(Debug)]
pub enum Operation {
    Val,
    Add,
    Sub,
    Mult,
}

pub struct CellValue {
    pub kind: CellType,
    cache: Option<i32>,
}

impl CellValue {
    pub fn new(kind: CellType) -> CellValue {
        CellValue { kind, cache: None }
    }

    pub fn value(&self) -> CellType {
        match self.cache {
            Some(val) => CellType::Val(val),
            None => self.kind,
        }
    }
}

pub struct Sheet {
    cells: Vec<Cell>,
}

impl Sheet {
    pub fn new() -> Sheet {
        Sheet { cells: vec![] }
    }

    pub fn push(&mut self, cell: Cell) {
        self.cells.push(cell);
    }

    fn cell_value(&mut self, index: usize, arg: usize) -> i32 {
        match &mut self.cells[index].args[arg] {
            Some(value) => match value.value() {
                CellType::Ref(reference) => {
                    let val = self.calculate_cell(reference);
                    val
                }
                CellType::Val(val) => val,
            },
            None => 0,
        }
    }

    pub fn calculate_cell(&mut self, index: usize) -> i32 {
        let mut args: [i32; 2] = [0, 0];

        for arg in 0..args.len() {
            args[arg] = self.cell_value(index, arg);
            if let Some(cell) = self.cells[index].args[arg].as_mut() {
                cell.cache = Some(args[arg])
            }
        }

        match self.cells[index].operation {
            Operation::Val => args[0],
            Operation::Add => args[0] + args[1],
            Operation::Sub => args[0] - args[1],
            Operation::Mult => args[0] * args[1],
        }
    }

}

pub struct Cell {
    pub operation: Operation,
    pub args: [Option<CellValue>; 2],
}

impl Cell {}
