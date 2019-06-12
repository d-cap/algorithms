#[derive(Debug)]
pub struct TriangularArray<T: Default + Clone> {
    array: Vec<T>,
    capacity: usize,
}

impl<T> TriangularArray<T>
where
    T: Default + Clone,
{
    pub fn with_capacity(size: usize) -> Result<Self, String> {
        if size > 1 {
            let size = size * (size - 1) / 2;
            Ok(TriangularArray {
                array: vec![Default::default(); size],
                capacity: size,
            })
        } else {
            Err("invalid length".to_owned())
        }
    }

    pub fn get_value(&self, rows_index: usize, columns_index: usize) -> Option<&T> {
        self.array
            .get(self.calculate_index(rows_index, columns_index))
    }

    pub fn set_value(&mut self, rows_index: usize, columns_index: usize, data: T) {
        let index = self.calculate_index(rows_index, columns_index);
        self.array[index] = data;
    }

    #[inline]
    fn calculate_index(&self, rows_index: usize, columns_index: usize) -> usize {
        if rows_index == columns_index {
            panic!(
                "The row({}) and column({}) indexes cannot be the same",
                rows_index, columns_index
            );
        }
        if rows_index > self.capacity {
            panic!(
                "The row({}) must be smaller than the number of rows({})",
                rows_index, self.capacity
            );
        }
        if columns_index > self.capacity {
            panic!(
                "The column({}) must be smaller than the number of rows({})",
                columns_index, self.capacity
            );
        }
        if rows_index < columns_index {
            columns_index * (columns_index - 1) / 2 + rows_index
        } else {
            rows_index * (rows_index - 1) / 2 + columns_index
        }
    }
}

#[cfg(test)]
mod tests {
    use super::TriangularArray;

    #[test]
    fn should_not_work_with_empty_array() {
        let a = TriangularArray::<u32>::with_capacity(0);
        assert!(a.is_err());
    }

    #[test]
    fn should_not_work_with_size_one_array() {
        let a = TriangularArray::<u32>::with_capacity(1);
        assert!(a.is_err());
    }

    #[test]
    fn should_work_with_size_two_array() {
        let capacity = 2;
        let mut a = TriangularArray::<usize>::with_capacity(capacity).unwrap();
        for column in 0..capacity {
            if column != 1 {
                a.set_value(1, column, 1 + column);
            }
        }

        assert_eq!(1, *a.get_value(0, 1).unwrap());
    }

    #[test]
    fn should_work_with_size_four_array() {
        let capacity = 4;
        let mut a = TriangularArray::<usize>::with_capacity(capacity).unwrap();
        for row in 1..capacity {
            for column in 0..capacity {
                if row != column {
                    a.set_value(row, column, row + column);
                }
            }
        }

        assert_eq!(1, *a.get_value(0, 1).unwrap());
    }
}
