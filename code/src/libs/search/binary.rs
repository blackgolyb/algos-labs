use crate::libs::list::double_linked_list::List;

use super::logger::Logger;

pub fn binary_search<T: Ord + PartialEq>(
    list: &mut List<T>,
    element: T,
    logger: &mut Logger,
) -> Option<usize> {
    logger.start();
    unsafe {
        let mut left = list.head;

        let mut l = 0;
        let mut r = list.len() as i64 - 1;

        while l <= r {
            let m = (l + r) / 2;
            let mut mid = left;
            for _ in 0..(m - l) {
                logger.log_shift();
                mid = (*mid).next;
            }

            let e = &mut (*mid).value;
            logger.log_compare();
            if element.eq(e) {
                logger.end();
                return Some(m as usize);
            } else if element.gt(e) {
                logger.log_compare();
                logger.log_shift();
                l = m + 1;
                left = (*mid).next;
            } else {
                logger.log_compare();
                r = m - 1;
            }
        }
    }
    logger.end();
    None
}
