use crate::libs::list::double_linked_list::List;

use super::logger::Logger;

pub fn linear_search<T: PartialEq>(
    list: &mut List<T>,
    elemet: T,
    logger: &mut Logger,
) -> Option<usize> {
    logger.start();

    let mut i: usize = 0;
    unsafe {
        let mut node = list.head;
        while !node.is_null() {
            let e = &mut (*node).value;
            logger.log_compare();
            if elemet.eq(e) {
                logger.end();
                return Some(i);
            }
            logger.log_shift();
            node = (*node).next;
            i += 1;
        }
    }
    logger.end();
    None
}
