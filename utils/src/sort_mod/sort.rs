pub struct Sort {}

impl Sort {
    /**
     * @desc 冒泡排序
     */
    pub fn bubble_sort<T: PartialOrd + std::fmt::Debug>(arr: &mut [T]) {
        let len = arr.len();

        for i in 0..len {
            for j in 0..len - i - 1 {
                if arr[j] > arr[j + 1] {
                    arr.swap(j, j + 1);
                }
            }
        }

        println!("{:?}", arr);
    }

    /**
     * @desc 选择排序
     */
    pub fn selection_sort<T: PartialOrd + std::fmt::Debug>(arr: &mut [T]) {
        let len = arr.len();
        let mut min_index;

        for i in 0..len - 1 {
            min_index = i;

            for j in i + 1..len {
                if arr[j] < arr[min_index] {
                    min_index = j;
                }
            }

            arr.swap(i, min_index)
        }

        println!("{:?}", arr);
    }

    /**
     * @desc 插入排序
     */
    pub fn insertion_sort<T: PartialOrd + std::fmt::Debug>(arr: &mut [T]) {
        let len = arr.len();

        for i in 0..len {
            let mut j = i;

            while j > 0 {
                if arr[j] < arr[j - 1] {
                    arr.swap(j, j - 1);
                }

                j -= 1;
            }
        }

        println!("{:?}", arr);
    }

    /**
     * @desc 希尔排序
     */
    pub fn shell_sort<T: PartialOrd + std::fmt::Debug>(arr: &mut [T]) {
        let len = arr.len();
        let mut gap = len / 2;

        while gap > 0 {
            for i in gap..len {
                let mut j = i;

                while j > 0 && j >= gap {
                    if arr[j - gap] > arr[j] {
                        arr.swap(j, j - gap);
                    }

                    j -= gap;
                }
            }

            gap /= 2;
        }
    }
}
