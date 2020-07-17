#[macro_export]
macro_rules! linked_list {
    () => (
        UnsafeLinkedList::new()
    );
    ($($x:expr),+ $(,)?) => {
        {
            let mut list = UnsafeLinkedList::new();

            $(
                list.push_front($x);
            )*

            list
        }
    };
    ($elem:expr; $n:expr) => (
        {
            let mut list = UnsafeLinkedList::new();

            for _ in 0..$n {
                list.push_front($elem);
            }

            list
        }
    );
}

#[cfg(test)]
mod test {
    use super::super::unsafe_linked_list::UnsafeLinkedList;

    #[test]
    fn empty_linked_list() {
        let mut list = linked_list![];

        assert!(list.is_empty());
        assert_eq!(list.pop_front(), None);

        list.push_front(5);
        assert_eq!(list.len(), 1);
        assert_eq!(list.pop_front(), Some(5));
    }

    #[test]
    fn single_linked_list() {
        let mut list = linked_list![0];

        assert_eq!(list.len(), 1);
        assert_eq!(list.pop_front(), Some(0));
        assert_eq!(list.pop_front(), None);
    }

    #[test]
    fn multiple_linked_list() {
        let mut list = linked_list![0, 1, 2,];

        assert_eq!(list.len(), 3);
        assert_eq!(list.pop_front(), Some(2));
        assert_eq!(list.pop_front(), Some(1));
        assert_eq!(list.pop_front(), Some(0));
        assert_eq!(list.pop_front(), None);
    }

    #[test]
    fn repeating_value() {
        let list = linked_list![10; 100];

        assert_eq!(list.len(), 100);
        assert_eq!(list.into_iter().sum::<i32>(), 1000);
    }
}
