import voucher

print('dir(voucher):', dir(voucher))

print('voucher.demo():', voucher.demo())


def test_eq(title, left, right):
    result = left == right
    print('[test]', title, ':', '✅' if result else '❌')

if 1:
    tpl = voucher.test_ffi()
    test_eq('voucher.test_ffi',
        tpl, (42, False, None, True, False, b'\xa0\xb1\xc2\xd3\xe4\xf5', False))
    # print(tpl)

    bs_jada = voucher.get_voucher_jada()
    test_eq('voucher.get_voucher_jada', len(bs_jada), 328)
    # print(len(bs_jada), bs_jada, list(bs_jada))

    bs_f2 = voucher.get_voucher_F2_00_02()
    test_eq('voucher.get_voucher_F2_00_02', len(bs_f2), 771)
    # print(len(bs_f2), bs_f2)

    bs_pem_f2 = voucher.get_masa_pem_F2_00_02()
    test_eq('voucher.get_masa_pem_F2_00_02', len(bs_pem_f2), 684)
    # print(len(bs_pem_f2), bs_pem_f2)

    # TODO
    # test_eq('voucher.validate - jada', voucher.validate(bs_jada), True)
    # test_eq('voucher.validate - f2', voucher.validate(bs_f2, bs_pem_f2), True)
