#[cfg(test)]
use mockall::{mock, predicate::*};


#[cfg(test)]
mod tests {
    #[test]
    fn exploration() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn another() {
        panic!("Make this test fail");
    }

    #[test]
    fn test_print_hud(str: lv, i8: hp, u16: ex, u16: ne) {
        let hud: &'static str = "╔═══________═════════════_╗\n\r\
                                 ╔                         ╗\n\r\
                                 ║ LV:{} HP:{} EX:{} NE:{} ║\n\r\
                                 ╚══________═╧════════════_╝";

    }
}
