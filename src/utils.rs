use std::collections::VecDeque;


pub fn peek_take_while<T>(iter: &mut VecDeque<T>,check: fn(&T) -> bool )-> VecDeque<T> {
    let mut ret_vec = VecDeque::new();
    loop {
        let Some(item) = iter.front() else {
            break;
        };
        if check(item) {
            break;
        }
        let Some(item) = iter.pop_front() else {
            break;
        };
        ret_vec.push_back(item);
    }
    ret_vec
}

#[cfg(test)]
mod test {
    use std::collections::VecDeque;

    use crate::utils::peek_take_while;

    #[test]
    fn _peek_take_while(){
        let mut deque = VecDeque::from(vec![1, 2, 3, 4, 5]);
        let result = peek_take_while(&mut deque, |&x| x == 3);
        assert_eq!(result, VecDeque::from(vec![1, 2]));
        assert_eq!(deque, VecDeque::from(vec![3, 4, 5]));

        let mut deque : VecDeque<usize>= VecDeque::new();
        let result = peek_take_while(&mut deque, |&x| x == 3);
        assert_eq!(result, VecDeque::new());
        assert_eq!(deque, VecDeque::new());

        let mut deque = VecDeque::from(vec![1, 2, 3, 4, 5]);
        let result = peek_take_while(&mut deque, |&x| x == 10);
        assert_eq!(result, VecDeque::from(vec![1, 2, 3, 4, 5]));
        assert_eq!(deque, VecDeque::new());
    }

}