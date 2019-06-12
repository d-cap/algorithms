#[derive(Default, Clone)]
pub struct SparseSingleLinkedList<T: Default + Clone> {
    sparse_list: Vec<Vec<T>>,
}

pub struct SparseRow<T: Default + Clone> {
    index: usize,
    next: SparseRow<T>,
    values: SparseNodeValue<T>
}

pub struct SparseNodeValue<T: Default + Clone> {
    index: usize,
    next: SparseNodeValue<T>,
    value: T,
}

impl<T> SparseSingleLinkedList<T>
where
    T: Default + Clone,
{
    pub fn new(size: usize) -> Self {
        SparseSingleLinkedList {
            sparse_list: vec![Vec::new(); size],
        }
    }

    pub fn insert(&mut self, row_index, usize, column_index: usize)

    pub fn get_value(&self, row_index: usize, column_index: usize) -> Option<&T> {
        for i in 0..self.sparse_list.len() {
            for j in 0..self.sparse_list[i].len() {
                if row_index == i && column_index == j {
                    return Some(&self.sparse_list[i][j]);
                }
            }
        }
        None
    }
}

#[derive(Default, Clone)]
pub struct SparseMatrixLinkedList<T> {
    sparse_rows: Vec<Vec<T>>,
    sparse_columns: Vec<Vec<T>>,
}

impl<T> SparseMatrixLinkedList<T> {
    pub fn new() -> Self {
        SparseMatrixLinkedList {
            sparse_rows: Vec::new(),
            sparse_columns: Vec::new(),
        }
    }
}
