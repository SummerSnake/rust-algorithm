pub struct Sort {}

impl Sort {
    /**
     * @desc 冒泡排序
     */
    pub fn bubble_sort<T: PartialOrd + Clone + std::fmt::Debug>(arr: &mut [T]) {
        let len = arr.len();
        let mut i = 0;
        let mut j;

        while i < len {
            j = i + 1;

            while j < len {
                if arr[i] > arr[j] {
                    arr.swap(i, j);
                }

                j += 1;
            }

            i += 1;
        }

        println!("{:?}", arr);
    }
}
