import voucher

print('dir(voucher):', dir(voucher))

print('voucher.demo():', voucher.demo())


def test_assert_eq(title, left, right, diag=True):
    result = left == right
    print('[test]', title, ':', '✅' if result else '❌')
    if diag and not result:
        print('test failed with')
        print('  left:', left)
        print('  right:', right)

def test_assert(title, condition):
    test_assert_eq(title, condition, True, diag=False)

if 1:  # test misc.
    test_assert_eq('bytes from list', bytes([1,2,3]), b'\x01\x02\x03')
    test_assert_eq('list from bytes', list(b'\x01\x02\x03'), [1,2,3])

if 1:  # test `voucher` module
    tpl = voucher.test_ffi()
    test_assert_eq('voucher.test_ffi',
        tpl, (42, False, None, True, False, b'\xa0\xb1\xc2\xd3\xe4\xf5', False))
    # print(tpl)

    bs_jada = voucher.get_voucher_jada()
    test_assert_eq('voucher.get_voucher_jada', len(bs_jada), 328)
    # print(len(bs_jada), bs_jada, list(bs_jada))

    bs_f2 = voucher.get_voucher_F2_00_02()
    test_assert_eq('voucher.get_voucher_F2_00_02', len(bs_f2), 771)
    # print(len(bs_f2), bs_f2)

    bs_pem_f2 = voucher.get_masa_pem_F2_00_02()
    test_assert_eq('voucher.get_masa_pem_F2_00_02', len(bs_pem_f2), 684)
    # print(len(bs_pem_f2), bs_pem_f2)

    test_assert('voucher.validate - jada', voucher.validate(bs_jada))
    test_assert('voucher.validate - F2_00_02', voucher.validate(bs_f2, bs_pem_f2))

    # The default `MP_RIOT_HEAPSIZE ?= 8192U` set in Makefile is not enough
    try:
        n = len(list(bs_jada))
    except MemoryError:
        n = -1
    test_assert_eq('no MemoryError for simple ops', n, 328)
