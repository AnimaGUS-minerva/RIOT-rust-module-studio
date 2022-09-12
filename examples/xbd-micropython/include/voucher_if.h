#ifndef VOUCHER_IF_H
#define VOUCHER_IF_H

#pragma once

#ifdef __cplusplus
extern "C" {
#endif

int vi_square(int);

void vi_init_psa_crypto(void);

size_t vi_get_voucher_jada(uint8_t **pp);
size_t vi_get_voucher_F2_00_02(uint8_t **pp);
size_t vi_get_masa_pem_F2_00_02(uint8_t **pp);
size_t vi_get_key_pem_F2_00_02(uint8_t **pp);
size_t vi_get_device_crt_F2_00_02(uint8_t **pp);

size_t vi_get_vrq_F2_00_02(uint8_t **pp);
size_t vi_create_vrq_F2_00_02(uint8_t **pp);

size_t vi_sign(const uint8_t *ptr_raw, size_t sz_raw, const uint8_t *ptr_key, size_t sz_key, uint8_t **pp);
bool vi_validate(const uint8_t *ptr, size_t sz);
bool vi_validate_with_pem(const uint8_t *ptr, size_t sz, const uint8_t *ptr_pem, size_t sz_pem);
void vi_dump(const uint8_t *ptr, size_t sz);

//

typedef void vi_provider_t;
void vi_provider_allocate(vi_provider_t **pp, bool is_vrq);
void vi_provider_free(vi_provider_t **pp);
void vi_provider_set(vi_provider_t *p, int val);

#endif // VOUCHER_IF_H

#ifdef __cplusplus
}
#endif