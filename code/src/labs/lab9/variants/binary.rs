
use crate::libs::list::double_linked_list::List;

use super::super::logger::Logger;

pub fn binary_search<T: Ord + PartialEq>(
    list: &mut List<T>,
    elemet: T,
    logger: &mut Logger,
) -> Option<usize> {
    logger.start();
    unsafe {
        let mut mid = list.head;
        let mut left = list.head;

        let mut l = 0;
        let mut r = list.len() as i64 - 1;

        while l <= r {
            let m = (l + r) / 2;
            mid = left;
            for _ in 0..(m - l) {
                mid = (*mid).next;
            }

            let e = &mut (*mid).value;
            logger.log_compare();
            if elemet.eq(e) {
                logger.end();
                return Some(m as usize);
            } else if elemet.gt(e) {
                logger.log_compare();
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
