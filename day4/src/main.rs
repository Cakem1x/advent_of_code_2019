fn main() {
    let mut pw_count = 0;
    for password_int in 264360..746326 {
        let pw = password_from_int(&password_int);
        if does_password_satisfy_conditions(&pw) {
            pw_count+=1;
            println!("{:?} satisfies all conditions. Found {} in total which do that.", pw, pw_count);
        }
    }
}

type Password = [u32; 6];

fn password_from_int(pw_int: &u32) -> Password {
    let mut pw = [0,0,0,0,0,0];
    for n in 0..6 {
        let divisor = 10_u32.pow(5-n as u32);
        pw[n] = pw_int/divisor % 10;
    }
    return pw;
}

fn password_from_string(s: &str) -> Password {
    let mut chars = s.chars();
    return [
        chars
            .next()
            .expect("password string too short")
            .to_digit(10)
            .expect("invalid first digit"),
        chars
            .next()
            .expect("password string too short")
            .to_digit(10)
            .expect("invalid second digit"),
        chars
            .next()
            .expect("password string too short")
            .to_digit(10)
            .expect("invalid third digit"),
        chars
            .next()
            .expect("password string too short")
            .to_digit(10)
            .expect("invalid fourth digit"),
        chars
            .next()
            .expect("password string too short")
            .to_digit(10)
            .expect("invalid fifth digit"),
        chars
            .next()
            .expect("password string too short")
            .to_digit(10)
            .expect("invalid sixth digit"),
    ];
}

fn does_password_satisfy_conditions(pw: &Password) -> bool {
    return has_password_adjacent_double_digit(&pw) && !does_password_decrease(&pw);
}

fn has_password_adjacent_double_digit(pw: &Password) -> bool {
    for (i_prev, digit) in pw[1..].iter().enumerate() {
        let i_next = i_prev + 2;
        if digit == &pw[i_prev] {
            // part 2 check: no additional adjacent digit of the same type
            if (i_prev == 0 || digit != &pw[i_prev - 1]) && (i_next >= pw.len() || digit != &pw[i_next])
            {
                return true;
            }
        }
    }
    return false;
}

fn does_password_decrease(pw: &Password) -> bool {
    for (i_prev, digit_current) in pw[1..].iter().enumerate() {
        let digit_prev = &pw[i_prev];
        if digit_current < digit_prev {
            return true;
        }
    }
    return false;
}

#[test]
fn test_password_decrease() {
    let pw = password_from_string("111111");
    assert_eq!(does_password_decrease(&pw), false);
    let pw = password_from_string("223450");
    assert_eq!(does_password_decrease(&pw), true);
    let pw = password_from_string("123789");
    assert_eq!(does_password_decrease(&pw), false);
}

#[test]
fn test_password_has_adjacent_double_digit() {
    let pw = password_from_string("112233");
    assert_eq!(has_password_adjacent_double_digit(&pw), true);
    let pw = password_from_string("123444");
    assert_eq!(has_password_adjacent_double_digit(&pw), false);
    let pw = password_from_string("111122");
    assert_eq!(has_password_adjacent_double_digit(&pw), true);
}

#[test]
#[should_panic]
fn test_password_from_string_panics_for_letters() {
    let pw_string = "1a3592";
    password_from_string(&pw_string);
}

#[test]
#[should_panic]
fn test_password_from_string_panics_if_too_short() {
    let pw_string = "1352";
    password_from_string(&pw_string);
}

#[test]
#[should_panic]
fn test_password_from_string_panics_for_negative_numbers() {
    let pw_string = "1-3592";
    password_from_string(&pw_string);
}

#[test]
fn test_password_from_int() {
    let pw = 123592;
    assert_eq!(
        password_from_int(&pw),
        [1, 2, 3, 5, 9, 2]
    );
}

#[test]
fn test_password_from_string() {
    let pw_string = "123592";
    assert_eq!(
        password_from_string(&pw_string),
        [1, 2, 3, 5, 9, 2]
    );
}
