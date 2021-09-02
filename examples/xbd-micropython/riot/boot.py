import voucher

print('dir(voucher):', dir(voucher))

print('voucher.demo():', voucher.demo())


def test_eq(title, left, right):
    result = left == right
    print('[test]', title, ':', '✅' if result else '❌')

if 1:
    tpl = voucher.test_ffi()
    test_eq('voucher.test_ffi', tpl, (42, False, None, True, False, b'\xa0\xb1\xc2\xd3\xe4\xf5', False))
    # print(tpl)

    bs = voucher.get_voucher_jada()
    test_eq('voucher.get_voucher_jada', len(bs), 328)
    # print(len(bs), bs, list(bs))

    bs = voucher.get_voucher_F2_00_02()
    test_eq('voucher.get_voucher_F2_00_02', len(bs), 771)
    # print(len(bs), bs)

    bs = voucher.get_masa_pem_F2_00_02()
    test_eq('voucher.get_masa_pem_F2_00_02', len(bs), 684)
    # print(len(bs), bs)
