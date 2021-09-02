/*
 * This file is part of the MicroPython project, http://micropython.org/
 *
 * The MIT License (MIT)
 *
 * Copyright (c) 2017 Linaro Limited
 * Copyright (c) 2021 ANIMA Minerva toolkit
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in
 * all copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
 * THE SOFTWARE.
 */

#include "py/mpconfig.h"
#if MICROPY_PY_VOUCHER

#include "py/runtime.h"

#include "stdio.h"
#include "string.h"
#include "voucher.h"

STATIC mp_obj_t mod_demo(void) {
    printf("[xbd-micropython/riot/custom/modvoucher.c] mod_demo(): ^^\n");

    return mp_const_false;
}
STATIC MP_DEFINE_CONST_FUN_OBJ_0(mod_demo_obj, mod_demo);

STATIC mp_obj_t mod_test_ffi(void) {
    printf("mod_test_ffi(): ^^\n");

    { // basic stuff
        int input = 4;
        int output = vch_square(input);
        printf("input: %d output: %d\n", input, output);
    }

    // return misc types

    uint8_t mac[6] = { 0xA0, 0xB1, 0xC2, 0xD3, 0xE4, 0xF5 };
    uint8_t mac2[6] = { 0x00, 0xB1, 0xC2, 0xD3, 0xE4, 0xF5 };

#define SZ 7
    mp_obj_t tuple[SZ] = {
        MP_OBJ_NEW_SMALL_INT(42), // 42
        mp_obj_new_bool(1 == 0), // False
        mp_const_none, // None
        mp_const_true, // True
        mp_const_false, // False
        mp_obj_new_bytes(mac, sizeof(mac)), // b'\xa0\xb1\xc2\xd3\xe4\xf5'
        mp_obj_new_bool(memcmp(mac2, mac, sizeof(mac)) == 0), // False
    };
    printf("sizeof(tuple): %d\n", sizeof(tuple));

    return mp_obj_new_tuple(SZ, tuple);
}
STATIC MP_DEFINE_CONST_FUN_OBJ_0(mod_test_ffi_obj, mod_test_ffi);

STATIC mp_obj_t mod_get_voucher_jada(void) {
    uint8_t *data;
    size_t sz;

    sz = vch_get_voucher_jada(&data);
    return mp_obj_new_bytes(data, sz);
}
STATIC MP_DEFINE_CONST_FUN_OBJ_0(mod_get_voucher_jada_obj, mod_get_voucher_jada);

STATIC mp_obj_t mod_get_voucher_F2_00_02(void) {
    uint8_t *data;
    size_t sz;

    sz = vch_get_voucher_F2_00_02(&data);
    return mp_obj_new_bytes(data, sz);
}
STATIC MP_DEFINE_CONST_FUN_OBJ_0(mod_get_voucher_F2_00_02_obj, mod_get_voucher_F2_00_02);

STATIC mp_obj_t mod_get_masa_pem_F2_00_02(void) {
    uint8_t *data;
    size_t sz;

    sz = vch_get_masa_pem_F2_00_02(&data);
    return mp_obj_new_bytes(data, sz);
}
STATIC MP_DEFINE_CONST_FUN_OBJ_0(mod_get_masa_pem_F2_00_02_obj, mod_get_masa_pem_F2_00_02);

//STATIC mp_obj_t mod_validate(/* TODO */) {
//    // !!!!
//
//
//    return mp_obj_new_bool(false);
//}
//STATIC MP_DEFINE_CONST_FUN_OBJ_0!!!!!!!!(mod_validate_obj, mod_validate);

STATIC const mp_rom_map_elem_t mp_module_voucher_globals_table[] = {
    { MP_ROM_QSTR(MP_QSTR___name__), MP_ROM_QSTR(MP_QSTR_voucher) },
    { MP_ROM_QSTR(MP_QSTR_demo), MP_ROM_PTR(&mod_demo_obj) },
    { MP_ROM_QSTR(MP_QSTR_test_ffi), MP_ROM_PTR(&mod_test_ffi_obj) },
    { MP_ROM_QSTR(MP_QSTR_get_voucher_jada), MP_ROM_PTR(&mod_get_voucher_jada_obj) },
    { MP_ROM_QSTR(MP_QSTR_get_voucher_F2_00_02), MP_ROM_PTR(&mod_get_voucher_F2_00_02_obj) },
    { MP_ROM_QSTR(MP_QSTR_get_masa_pem_F2_00_02), MP_ROM_PTR(&mod_get_masa_pem_F2_00_02_obj) },
//    { MP_ROM_QSTR(MP_QSTR_validate), MP_ROM_PTR(&mod_validate_obj) },
};

STATIC MP_DEFINE_CONST_DICT(mp_module_voucher_globals, mp_module_voucher_globals_table);

const mp_obj_module_t mp_module_voucher = {
    .base = { &mp_type_module },
    .globals = (mp_obj_dict_t*)&mp_module_voucher_globals,
};

#endif // MICROPY_PY_VOUCHER
