use super::{build_test, Felt};

#[test]
fn test_montgomery_repr() {
    let source = "
    use.std::math::secp256k1_base_field

    begin
        exec.secp256k1_base_field::to_mont
        exec.secp256k1_base_field::from_mont
    end";

    let mut num = [0u32; 8];
    for i in 0..4 {
        let a = rand_utils::rand_value::<u32>();
        let b = rand_utils::rand_value::<u32>();

        num[i] = a;
        num[i ^ 4] = b;
    }

    let mut stack = [0u64; 8];
    for i in 0..4 {
        stack[i] = num[i] as u64;
        stack[i ^ 4] = num[i ^ 4] as u64;
    }
    stack.reverse();

    let test = build_test!(source, &stack);
    let strace = test.get_last_stack_state();

    for i in 0..8 {
        assert_eq!(Felt::new(num[i] as u64), strace[i]);
    }
}

#[test]
fn test_mul() {
    let source = "
    use.std::math::secp256k1_base_field

    begin
        exec.secp256k1_base_field::mul
    end";

    let mut stack = [0u64; 16];
    for i in 0..8 {
        let a = rand_utils::rand_value::<u32>() as u64;
        let b = rand_utils::rand_value::<u32>() as u64;

        stack[i] = a;
        stack[i ^ 8] = b;
    }

    let mut a = [0u32; 8];
    let mut b = [0u32; 8];

    for i in 0..8 {
        a[i] = stack[i] as u32;
        b[i] = stack[i ^ 8] as u32;
    }

    let expected = u256xu256_mod_mult(&a, &b);

    stack.reverse();

    let test = build_test!(source, &stack);
    let strace = test.get_last_stack_state();

    for i in 0..8 {
        assert_eq!(Felt::new(expected[i] as u64), strace[i]);
    }
}

#[test]
fn test_add() {
    let source = "
    use.std::math::secp256k1_base_field

    begin
        exec.secp256k1_base_field::add
    end";

    let mut stack = [0u64; 16];
    for i in 0..8 {
        let a = rand_utils::rand_value::<u32>() as u64;
        let b = rand_utils::rand_value::<u32>() as u64;

        stack[i] = a;
        stack[i ^ 8] = b;
    }

    let mut a = [0u32; 8];
    let mut b = [0u32; 8];

    for i in 0..8 {
        a[i] = stack[i] as u32;
        b[i] = stack[i ^ 8] as u32;
    }

    let expected = u256_mod_add(a, b);

    stack.reverse();

    let test = build_test!(source, &stack);
    let strace = test.get_last_stack_state();

    for i in 0..8 {
        assert_eq!(Felt::new(expected[i] as u64), strace[i]);
    }
}

#[test]
#[allow(clippy::needless_range_loop)]
fn test_neg() {
    let source = "
    use.std::math::secp256k1_base_field

    begin
        exec.secp256k1_base_field::neg
    end";

    let mut stack = [0u64; 16];
    for i in 0..8 {
        stack[i] = rand_utils::rand_value::<u32>() as u64;
    }

    let mut a = [0u32; 8];
    for i in 0..8 {
        a[i] = stack[i] as u32;
    }

    let expected = u256_mod_neg(a);

    stack.reverse();

    let test = build_test!(source, &stack);
    let strace = test.get_last_stack_state();

    for i in 0..8 {
        assert_eq!(Felt::new(expected[i] as u64), strace[i]);
    }
}

#[test]
fn test_sub() {
    let source = "
    use.std::math::secp256k1_base_field

    begin
        exec.secp256k1_base_field::sub
    end";

    let mut stack = [0u64; 16];
    for i in 0..8 {
        let a = rand_utils::rand_value::<u32>() as u64;
        let b = rand_utils::rand_value::<u32>() as u64;

        stack[i] = a;
        stack[i ^ 8] = b;
    }

    let mut a = [0u32; 8];
    let mut b = [0u32; 8];

    for i in 0..8 {
        a[i] = stack[i] as u32;
        b[i] = stack[i ^ 8] as u32;
    }

    let expected = u256_mod_sub(a, b);

    stack.reverse();

    let test = build_test!(source, &stack);
    let strace = test.get_last_stack_state();

    for i in 0..8 {
        assert_eq!(Felt::new(expected[i] as u64), strace[i]);
    }
}

#[test]
fn test_add_then_sub() {
    let source_add = "
    use.std::math::secp256k1_base_field

    begin
        exec.secp256k1_base_field::add
    end";

    let source_sub = "
    use.std::math::secp256k1_base_field

    begin
        exec.secp256k1_base_field::sub
    end";

    let mut stack = [0u64; 16];
    for i in 0..8 {
        let a = rand_utils::rand_value::<u32>() as u64;
        let b = rand_utils::rand_value::<u32>() as u64;

        stack[i] = a;
        stack[i ^ 8] = b;
    }

    let mut a = [0u32; 8];
    let mut b = [0u32; 8];

    // randomly generate a, b --- two secp256k1 field elements
    for i in 0..8 {
        a[i] = stack[i] as u32;
        b[i] = stack[i ^ 8] as u32;
    }

    // compute c = a + b
    let c = {
        stack.reverse();

        let c = u256_mod_add(a, b);

        let test = build_test!(source_add, &stack);
        let strace = test.get_last_stack_state();

        for i in 0..8 {
            assert_eq!(Felt::new(c[i] as u64), strace[i]);
        }

        c
    };

    for i in 0..8 {
        stack[i] = c[i] as u64;
        stack[i ^ 8] = a[i] as u64;
    }

    // compute d = c - a
    let d = {
        stack.reverse();

        let d = u256_mod_sub(c, a);

        let test = build_test!(source_sub, &stack);
        let strace = test.get_last_stack_state();

        for i in 0..8 {
            assert_eq!(Felt::new(d[i] as u64), strace[i]);
        }

        d
    };

    // check b == d | (d = c - a) & (c = a + b)
    for i in 0..8 {
        assert_eq!(b[i] ^ d[i], 0);
    }
}

fn mac(a: u32, b: u32, c: u32, carry: u32) -> (u32, u32) {
    let tmp = a as u64 + (b as u64 * c as u64) + carry as u64;
    ((tmp >> 32) as u32, tmp as u32)
}

fn adc(a: u32, b: u32, carry: u32) -> (u32, u32) {
    let tmp = a as u64 + b as u64 + carry as u64;
    ((tmp >> 32) as u32, tmp as u32)
}

fn sbb(a: u32, b: u32, borrow: u32) -> (u32, u32) {
    let tmp = (a as u64).wrapping_sub(b as u64 + (borrow >> 31) as u64);
    ((tmp >> 32) as u32, tmp as u32)
}

fn u256xu32(a: &mut [u32], b: u32, c: &[u32]) {
    assert_eq!(a.len(), 9);
    assert_eq!(c.len(), 8);

    let mut carry: u32;

    let v = mac(a[0], b, c[0], 0);
    carry = v.0;
    a[0] = v.1;

    let v = mac(a[1], b, c[1], carry);
    carry = v.0;
    a[1] = v.1;

    let v = mac(a[2], b, c[2], carry);
    carry = v.0;
    a[2] = v.1;

    let v = mac(a[3], b, c[3], carry);
    carry = v.0;
    a[3] = v.1;

    let v = mac(a[4], b, c[4], carry);
    carry = v.0;
    a[4] = v.1;

    let v = mac(a[5], b, c[5], carry);
    carry = v.0;
    a[5] = v.1;

    let v = mac(a[6], b, c[6], carry);
    carry = v.0;
    a[6] = v.1;

    let v = mac(a[7], b, c[7], carry);
    a[8] = v.0;
    a[7] = v.1;
}

fn u288_reduce(c: &[u32], pc: u32) -> [u32; 9] {
    assert_eq!(c.len(), 9);

    let prime: [u32; 8] = [
        4294966319, 4294967294, 4294967295, 4294967295, 4294967295, 4294967295, 4294967295,
        4294967295,
    ];
    let mu: u32 = 3525653809;

    let q: u32 = mu.wrapping_mul(c[0]);
    let mut carry: u32;
    let mut d = [0u32; 9];

    let v = mac(c[0], q, prime[0], 0);
    carry = v.0;

    let v = mac(c[1], q, prime[1], carry);
    carry = v.0;
    d[0] = v.1;

    let v = mac(c[2], q, prime[2], carry);
    carry = v.0;
    d[1] = v.1;

    let v = mac(c[3], q, prime[3], carry);
    carry = v.0;
    d[2] = v.1;

    let v = mac(c[4], q, prime[4], carry);
    carry = v.0;
    d[3] = v.1;

    let v = mac(c[5], q, prime[5], carry);
    carry = v.0;
    d[4] = v.1;

    let v = mac(c[6], q, prime[6], carry);
    carry = v.0;
    d[5] = v.1;

    let v = mac(c[7], q, prime[7], carry);
    carry = v.0;
    d[6] = v.1;

    let v = adc(c[8], pc, carry);
    d[8] = v.0;
    d[7] = v.1;

    d
}

/// See `mont_mult` routine in https://gist.github.com/itzmeanjan/d4853347dfdfa853993f5ea059824de6
fn u256xu256_mod_mult(a: &[u32], b: &[u32]) -> [u32; 8] {
    assert_eq!(a.len(), 8);
    assert_eq!(a.len(), b.len());

    let mut c = [0u32; 16];
    let mut pc = 0u32;

    u256xu32(&mut c[0..9], b[0], a);

    let d = u288_reduce(&c[0..9], pc);
    pc = d[8];
    c[1..9].copy_from_slice(&d[0..8]);

    u256xu32(&mut c[1..10], b[1], a);

    let d = u288_reduce(&c[1..10], pc);
    pc = d[8];
    c[2..10].copy_from_slice(&d[0..8]);

    u256xu32(&mut c[2..11], b[2], a);

    let d = u288_reduce(&c[2..11], pc);
    pc = d[8];
    c[3..11].copy_from_slice(&d[0..8]);

    u256xu32(&mut c[3..12], b[3], a);

    let d = u288_reduce(&c[3..12], pc);
    pc = d[8];
    c[4..12].copy_from_slice(&d[0..8]);

    u256xu32(&mut c[4..13], b[4], a);

    let d = u288_reduce(&c[4..13], pc);
    pc = d[8];
    c[5..13].copy_from_slice(&d[0..8]);

    u256xu32(&mut c[5..14], b[5], a);

    let d = u288_reduce(&c[5..14], pc);
    pc = d[8];
    c[6..14].copy_from_slice(&d[0..8]);

    u256xu32(&mut c[6..15], b[6], a);

    let d = u288_reduce(&c[6..15], pc);
    pc = d[8];
    c[7..15].copy_from_slice(&d[0..8]);

    u256xu32(&mut c[7..16], b[7], a);

    let d = u288_reduce(&c[7..16], pc);
    pc = d[8];
    c[8..16].copy_from_slice(&d[0..8]);

    c[8] += pc * 977;
    c[9] += pc;

    c[8..16].try_into().expect("incorrect length")
}

/// See https://gist.github.com/itzmeanjan/d4853347dfdfa853993f5ea059824de6#file-test_montgomery_arithmetic-py-L236-L256
fn u256_mod_add(a: [u32; 8], b: [u32; 8]) -> [u32; 8] {
    let mut c = [0u32; 8];
    let mut carry = 0u32;

    let v = adc(a[0], b[0], carry);
    carry = v.0;
    c[0] = v.1;

    let v = adc(a[1], b[1], carry);
    carry = v.0;
    c[1] = v.1;

    let v = adc(a[2], b[2], carry);
    carry = v.0;
    c[2] = v.1;

    let v = adc(a[3], b[3], carry);
    carry = v.0;
    c[3] = v.1;

    let v = adc(a[4], b[4], carry);
    carry = v.0;
    c[4] = v.1;

    let v = adc(a[5], b[5], carry);
    carry = v.0;
    c[5] = v.1;

    let v = adc(a[6], b[6], carry);
    carry = v.0;
    c[6] = v.1;

    let v = adc(a[7], b[7], carry);
    carry = v.0;
    c[7] = v.1;

    c[0] += carry * 977;
    c[1] += carry;

    c
}

/// See https://github.com/itzmeanjan/secp256k1/blob/ec3652afe8ed72b29b0e39273a876a898316fb9a/field.py#L77-L95
fn u256_mod_neg(a: [u32; 8]) -> [u32; 8] {
    let mut b = [0u32; 8];
    let mut borrow = 0u32;

    let prime = [
        4294966319, 4294967294, 4294967295, 4294967295, 4294967295, 4294967295, 4294967295,
        4294967295,
    ];

    let v = sbb(prime[0], a[0], borrow);
    borrow = v.0;
    b[0] = v.1;

    let v = sbb(prime[1], a[1], borrow);
    borrow = v.0;
    b[1] = v.1;

    let v = sbb(prime[2], a[2], borrow);
    borrow = v.0;
    b[2] = v.1;

    let v = sbb(prime[3], a[3], borrow);
    borrow = v.0;
    b[3] = v.1;

    let v = sbb(prime[4], a[4], borrow);
    borrow = v.0;
    b[4] = v.1;

    let v = sbb(prime[5], a[5], borrow);
    borrow = v.0;
    b[5] = v.1;

    let v = sbb(prime[6], a[6], borrow);
    borrow = v.0;
    b[6] = v.1;

    let v = sbb(prime[7], a[7], borrow);
    b[7] = v.1;

    b
}

/// See https://github.com/itzmeanjan/secp256k1/blob/ec3652afe8ed72b29b0e39273a876a898316fb9a/field.py#L97-L101
fn u256_mod_sub(a: [u32; 8], b: [u32; 8]) -> [u32; 8] {
    u256_mod_add(a, u256_mod_neg(b))
}