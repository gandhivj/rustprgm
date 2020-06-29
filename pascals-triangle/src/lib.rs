pub struct PascalsTriangle(u32);

impl PascalsTriangle {
    pub fn new(row_count: u32) -> Self {
        Self(row_count)
    }

    pub fn rows(&self) -> Vec<Vec<u32>> {
        let mut result = vec![];
        let rc = self.0;

        let mut prev_row:     Vec<u32> = vec![];
        let mut prev_row_ext: Vec<u32> = vec![];
        let mut new_row:      Vec<u32> = vec![];
        for _i in 0..rc {
            prev_row = new_row.clone();

            if prev_row.is_empty() {
                prev_row.push(1);
                new_row.push(1);
            } else {
                prev_row_ext = prev_row.clone();
                prev_row_ext.insert(0, 0);
                prev_row_ext.push(0);

                new_row.clear();
                for j in 0..=prev_row.len() {
                    new_row.push(prev_row_ext[j] + prev_row_ext[j+1]);
                }

            }

            result.push(new_row.clone());
        }

        result
    }
}
