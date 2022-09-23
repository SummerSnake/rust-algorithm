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

        println!("{:?}", arr);
    }

    /**
     * @desc 归并排序
     */
    fn merge<T>(arr: &mut [T], divider: usize)
    where
        T: PartialOrd + std::fmt::Debug + Clone + Copy,
    {
        let divider = divider.min(arr.len());
        let left = arr[..divider].to_vec();
        let right = arr[divider..].to_vec();

        let mut l = 0;
        let mut r = 0;

        for item in arr.iter_mut() {
            if r == right.len() || (l < left.len() && left[l] < right[r]) {
                *item = left[l];
                l += 1;
            } else {
                *item = right[r];
                r += 1;
            }
        }
    }

    pub fn merge_sort<T>(arr: &mut [T])
    where
        T: PartialOrd + std::fmt::Debug + Clone + Copy,
    {
        let mut step = 2;

        while step < arr.len() {
            arr.chunks_mut(step)
                .for_each(|sub| Sort::merge(sub, step / 2));

            step *= 2;
        }

        arr.chunks_mut(step)
            .for_each(|sub| Sort::merge(sub, step / 2));

        println!("{:?}", arr);
    }

    /**
     * @desc 快速排序
     */
    fn partition<T: PartialOrd>(arr: &mut [T], left: usize, right: usize) -> usize {
        let pivot = right; // 基数的起始位置
        let mut i = left;
        let mut j = right;

        loop {
            while arr[i] < arr[pivot] {
                i += 1;
            }
            while arr[j] > arr[pivot] {
                j -= 1;
            }

            if i >= j {
                break;
            } else {
                arr.swap(i, j);
                i += 1;
                j -= 1;
            }
        }

        arr.swap(i, pivot);
        i
    }

    fn quick_sort_range<T: PartialOrd>(arr: &mut [T], left: usize, right: usize) {
        if left < right {
            let partition = Sort::partition(arr, left, right);

            Sort::quick_sort_range(arr, left, partition - 1);
            Sort::quick_sort_range(arr, partition + 1, right);
        }
    }

    pub fn quick_sort<T: PartialOrd + std::fmt::Debug>(arr: &mut [T]) {
        let len = arr.len();

        if len > 1 {
            Sort::quick_sort_range(arr, 0, len - 1);
        }

        println!("{:?}", arr);
    }
}
