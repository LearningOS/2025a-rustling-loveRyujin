/*
	sort
	This problem requires you to implement a sorting algorithm
	you can use bubble sorting, insertion sorting, heap sorting, etc.
*/

fn sort<T>(array: &mut [T])
where
    T: PartialOrd,
{
    if array.len() <= 1 {
        return;
    }
    quicksort(array, 0, array.len() - 1);
}

fn quicksort<T>(array: &mut [T], low: usize, high: usize)
where
    T: PartialOrd,
{
    if low < high {
        let pivot_index = partition(array, low, high);
        
        // 递归排序左半部分
        if pivot_index > 0 {
            quicksort(array, low, pivot_index - 1);
        }
        
        // 递归排序右半部分
        if pivot_index + 1 < array.len() {
            quicksort(array, pivot_index + 1, high);
        }
    }
}

fn partition<T>(array: &mut [T], low: usize, high: usize) -> usize
where
    T: PartialOrd,
{
    // 选择最后一个元素作为基准
    let pivot_index = high;
    let mut i = low;
    
    for j in low..high {
        // 如果当前元素小于或等于基准元素
        if array[j] <= array[pivot_index] {
            array.swap(i, j);
            i += 1;
        }
    }
    
    // 将基准元素放到正确位置
    array.swap(i, pivot_index);
    i
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sort_1() {
        let mut vec = vec![37, 73, 57, 75, 91, 19, 46, 64];
        sort(&mut vec);
        assert_eq!(vec, vec![19, 37, 46, 57, 64, 73, 75, 91]);
    }
	#[test]
    fn test_sort_2() {
        let mut vec = vec![1];
        sort(&mut vec);
        assert_eq!(vec, vec![1]);
    }
	#[test]
    fn test_sort_3() {
        let mut vec = vec![99, 88, 77, 66, 55, 44, 33, 22, 11];
        sort(&mut vec);
        assert_eq!(vec, vec![11, 22, 33, 44, 55, 66, 77, 88, 99]);
    }
}