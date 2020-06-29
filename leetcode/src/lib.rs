mod twosum;
mod longestsubstr;
mod reverse;
mod atoi;
mod palindrone;
mod roman;
mod duplicate;
mod kmp;
mod dp;
mod stack;
mod minstack;
mod maxarea;
mod edit;
mod ilist;
mod count_say;

extern {
    fn c_atoi(string: *const u8) -> i32;
    fn reverseString(s: *mut u8, size: i32);
    fn maxArea(height: *const i32, size: i32) -> i32;
    fn trap(height: *const i32, size: i32) -> i32;
    fn minDistance(word1: *const u8, word2: *const u8) -> i32;
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_two_sum() {
        let res = crate::twosum::two_sum([3, 2, 4].to_vec(), 6);
        assert_eq!([1, 2].to_vec(), res);
    }
    #[test]
    fn test_two_sum_hash() {
        let res = crate::twosum::two_sum_hash([3, 2, 4].to_vec(), 6);
        assert_eq!([1, 2].to_vec(), res);
    }

    #[test]
    fn test_longest_substr_a() {
        let res = crate::longestsubstr::length_of_longest_substring("abcdefd".to_string());
        assert_eq!(6, res);
    }
    #[test]
    fn test_longest_substr_b() {
        let res = crate::longestsubstr::length_of_longest_substring("abba".to_string());
        assert_eq!(2, res);
    }
    #[test]
    fn test_longest_common_prefix() {
        let input = vec!["mory".to_string(), "morning".to_string()];
        let res = crate::longestsubstr::longest_common_prefix(input);
        assert_eq!("mor".to_string(), res);
        let input = vec!["mory".to_string(), "morning".to_string(), String::new()];
        let res = crate::longestsubstr::longest_common_prefix(input);
        assert_eq!("".to_string(), res);
        let input = vec!["aa".to_string(), "a".to_string()];
        let res = crate::longestsubstr::longest_common_prefix(input);
        assert_eq!("a".to_string(), res);
    }  
    #[test]
    fn test_reverse_int() {
        let res = crate::reverse::reverse_int(123);
        assert_eq!(321, res);
        let res = crate::reverse::reverse_int(2147483647);
        assert_eq!(0, res);
        let res = crate::reverse::reverse_int(-123); 
        assert_eq!(-321, res);
        let res = crate::reverse::reverse_int(-2147483648); 
        assert_eq!(0, res);
    }
    #[test]
    fn test_reverse_string() {
        let mut input: Vec<char> = "hello".chars().collect();
        crate::reverse::reverse_string(&mut input); 
        let left: Vec<char> = "olleh".chars().collect();
        assert_eq!(left, input);
        let mut input: Vec<char> = "".chars().collect();
        crate::reverse::reverse_string(&mut input); 
        let left: Vec<char> = "".chars().collect();
        assert_eq!(left, input);
    }

    #[test]
    fn test_atoi() {
        let res = crate::atoi::atoi("123".to_string());
        assert_eq!(123, res);
        let res = crate::atoi::atoi("   123v".to_string());
        assert_eq!(123, res);
        let res = crate::atoi::atoi("fd123".to_string());
        assert_eq!(0, res);
        let res = crate::atoi::atoi("-123fdf".to_string());
        assert_eq!(-123, res);
        let res = crate::atoi::atoi("-2147483649fdf".to_string());
        assert_eq!(-2147483648, res);
        let res = crate::atoi::atoi("2147483649fdf".to_string());
        assert_eq!(2147483647, res);
        let res = crate::atoi::atoi("-91283472332".to_string());
        assert_eq!(-2147483648, res);
    }
    #[test]
    fn test_c_atoi() {
        unsafe {
            let res = crate::c_atoi(b"123\0".as_ptr());
            assert_eq!(123, res);
            let res = crate::c_atoi(b"   123v\0".as_ptr());
            assert_eq!(123, res);
            let res = crate::c_atoi(b"fd123\0".as_ptr());
            assert_eq!(0, res);
            let res = crate::c_atoi(b"-123fdf\0".as_ptr());
            assert_eq!(-123, res);
            let res = crate::c_atoi(b"-2147483649fdf\0".as_ptr());
            assert_eq!(-2147483648, res);
            let res = crate::c_atoi(b"2147483649fdf\0".as_ptr());
            assert_eq!(2147483647, res);
            let res = crate::c_atoi(b"-91283472332\0".as_ptr());
            assert_eq!(-2147483648, res);
        }
    }

    #[test]
    fn test_c_reverse_string() {
        unsafe {
            let mut strs = b"hello\n".to_owned();
            crate::reverseString(strs.as_mut_ptr(), 5);
            assert_eq!(b"olleh\n".to_owned(), strs);
        }
    }
    #[test]
    fn test_palindrone() {
        let res = crate::palindrone::is_palindrome(123);
        assert_eq!(false, res);
        let res = crate::palindrone::is_palindrome(121);
        assert_eq!(true, res);
        let res = crate::palindrone::is_palindrome(-121);
        assert_eq!(false, res);
    }

    #[test]
    fn test_roman_to_int() {
        let res = crate::roman::roman_to_int("III".to_string());
        assert_eq!(3, res);
        let res = crate::roman::roman_to_int("IVII".to_string());
        assert_eq!(6, res);
    }

    #[test]
    fn test_remove_duplicate() {
        let res = crate::duplicate::remove_duplicates(&mut [1, 1, 2, 2, 3, 3].to_vec());
        assert_eq!(3, res);
    }
    #[test]
    fn test_remove_element() {
        let res = crate::duplicate::remove_element(&mut [1, 1, 2, 2, 3, 3].to_vec(), 2);
        assert_eq!(4, res);
    }

    #[test]
    fn test_kmp_next() {
        let res = crate::kmp::compute_next(&"needle".to_string());
        let left: Vec<usize> = vec![0, 1, 1, 1, 1, 1];
        assert_eq!(left, res);
        let res = crate::kmp::compute_next(&"abcabcde".to_string());
        let left: Vec<usize> = vec![0, 1, 1, 1, 2, 3, 4, 1];
        assert_eq!(left, res);
        let res = crate::kmp::compute_next(&"".to_string());
        let left: Vec<usize> = vec![];
        assert_eq!(left, res);
    }
    fn test_kmp() {
        let res = crate::kmp::str_str(
            "hello".to_string(), "ll".to_string());
        assert_eq!(2, res);
        let res = crate::kmp::str_str(
            "a".to_string(), "".to_string());
        assert_eq!(0, res);
        let res = crate::kmp::str_str(
            "aississipa".to_string(), "issip".to_string());
        assert_eq!(4, res);
        let res = crate::kmp::str_str(
            "".to_string(), "".to_string());
        assert_eq!(0, res);
        let res = crate::kmp::str_str(
            "Jenny".to_string(), "Mory".to_string());
        assert_eq!(-1, res);
        let res = crate::kmp::str_str(
            "".to_string(), "Mory".to_string());
        assert_eq!(-1, res);
    }

    #[test]
    fn test_max_sub_array() {
        let mut input = [-3, -2, -1, 0, 1, 2].to_vec();
        assert_eq!(3, crate::dp::max_sub_array(input));
        let mut input = [-3].to_vec();
        assert_eq!(-3, crate::dp::max_sub_array(input));
    }

    #[test] 
    fn test_climb_stairs() {
        assert_eq!(3, crate::dp::climb_stairs(3));
    }

    #[test] 
    fn test_stock_once() {
        let profit = crate::dp::max_profit_once([7,1,5,3,6,4].to_vec());
        assert_eq!(5, profit);
    }

    #[test] 
    fn test_stock_infinite() {
        let profit = crate::dp::max_profit_infinite([7,1,5,3,6,4].to_vec());
        assert_eq!(7, profit);
    }

    #[test] 
    fn test_stock_twice() {
        let profit = crate::dp::max_profit_twice([3,3,5,0,0,3,1,4].to_vec());
        assert_eq!(6, profit);
        let profit = crate::dp::max_profit_twice([3,2,6,5,0,3].to_vec());
        assert_eq!(7, profit);
    }

    #[test]
    fn test_stock_k() {
        let profit = crate::dp::max_profit_k_memory_consume(2, [3,2,6,5,0,3].to_vec());
        assert_eq!(7, profit);
    }

    #[test]
    fn test_min_path_sum() {
        //  1  3  1     1  4  5
        //  1  5  1  => 2  7  6  
        //  4  2  1     6  8  7 
        let input = [ [1,3,1].to_vec(), [1,5,1].to_vec(), [4,2,1].to_vec() ].to_vec();
        let res = crate::dp::min_path_sum(input);
        assert_eq!(7, res);
    }

    #[test]
    fn test_is_valid() {
        let valid = crate::stack::is_valid("({{}})".to_string());
        assert_eq!(true, valid);
        let valid = crate::stack::is_valid("(]".to_string());
        assert_eq!(false, valid);
    }

    #[test]
    fn test_maxArea() {
        unsafe {
            let input = [1,8,6,2,5,4,8,3,7].to_vec();
            let res = crate::maxArea(input.as_ptr(), input.len() as i32);
            assert_eq!(49, res);
        }
    }
        #[test]
        fn test_max_area() {
            let input = [1,8,6,2,5,4,8,3,7].to_vec();
            let res = crate::maxarea::max_area(input);
            assert_eq!(49, res);
    }

    #[test]
    fn test_trap_water() {
        unsafe {
            //    let input = [0,1,0,2,1,0,1,3,2,1,2,1].to_vec();
            //    let res = crate::trap(input.as_ptr(), input.len() as i32);
            //    assert_eq!(6, res);
            let input = [4, 2, 3];
            let res = crate::trap(input.as_ptr(), input.len() as i32);
            assert_eq!(1, res);
        }
    }

    #[test]
    fn test_minstack() {
        let mut stack = crate::minstack::MinStack::new();
        stack.push(-2);
        stack.push(0);
        stack.push(-3);
        assert_eq!(-3, stack.get_min());
        stack.pop();
        stack.top();
        assert_eq!(-2, stack.get_min());
    }

    #[test]
    fn test_edit_distance() {
        let word1 = "horse".to_string(); 
        let word2 = "ros".to_string();
        let dist = crate::edit::min_distance(word1, word2);
        assert_eq!(dist, 3);
        let word1 = "".to_string();
        let word2 = "".to_string();
        let dist = crate::edit::min_distance(word1, word2);
        assert_eq!(dist, 0);
        let word1 = "a".to_string();
        let word2 = "a".to_string();
        let dist = crate::edit::min_distance(word1, word2);
        assert_eq!(dist, 0);
        let word1 = "a".to_string();
        let word2 = "ab".to_string();
        let dist = crate::edit::min_distance(word1, word2);
        assert_eq!(dist, 1);
        let word1 = "a".to_string();
        let word2 = "b".to_string();
        let dist = crate::edit::min_distance(word1, word2);
        assert_eq!(dist, 1);
    }

    #[test]
    fn test_edit_distance_iter() {
        // let word1 = "".to_string();
        // let word2 = "".to_string();
        // let dist = crate::edit::min_distance_iter(word1, word2);
        // assert_eq!(dist, 0);
        // let word1 = "a".to_string();
        // let word2 = "a".to_string();
        // let dist = crate::edit::min_distance_iter(word1, word2);
        // assert_eq!(dist, 0);
        // let word1 = "a".to_string();
        // let word2 = "ab".to_string();
        // let dist = crate::edit::min_distance_iter(word1, word2);
        // assert_eq!(dist, 1);
        // let word1 = "ose".to_string(); 
        // let word2 = "r".to_string();
        // let dist = crate::edit::min_distance_iter(word1, word2);
        // assert_eq!(dist, 3);
        let word1 = "a".to_string();
        let word2 = "b".to_string();
        let dist = crate::edit::min_distance_iter(word1, word2);
        assert_eq!(dist, 1);

    }

    #[test]
    fn test_edit_distance_c() {
        unsafe {
            let word1 = b"horse\0".as_ptr();
            let word2 = b"ros\0".as_ptr();
            let dist = crate::minDistance(word1, word2);
            assert_eq!(dist, 3);
            let word1 = b"\0".as_ptr();
            let word2 = b"\0".as_ptr();
            let dist = crate::minDistance(word1, word2);
            assert_eq!(dist, 0);
            let word1 = b"a\0".as_ptr();
            let word2 = b"a\0".as_ptr();
            let dist = crate::minDistance(word1, word2);
            assert_eq!(dist, 0);
            let word1 = b"a\0".as_ptr();
            let word2 = b"ab\0".as_ptr();
            let dist = crate::minDistance(word1, word2);
            assert_eq!(dist, 1);
        }
    }

    #[test]
    fn test_i_list() {
        let mut l1 = crate::ilist::ListNode::new(1);
        let mut l2 = None;
        let res = crate::ilist::merge_two_lists(Some(Box::new(l1)), l2);
        let res = res.unwrap();
        assert_eq!(res.val, 1);
    }

    #[test]
    fn test_count_and_say() {
        let res = crate::count_say::count_and_say(5);
        assert_eq!("111221".to_string(), res);
    }
}
