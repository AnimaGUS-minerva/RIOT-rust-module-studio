int vch_square(int);

size_t vch_get_voucher_jada(uint8_t **pp);
size_t vch_get_voucher_F2_00_02(uint8_t **pp);
size_t vch_get_masa_pem_F2_00_02(uint8_t **pp);
size_t vch_get_key_pem_F2_00_02(uint8_t **pp);
size_t vch_get_device_crt_F2_00_02(uint8_t **pp);

size_t vch_get_vrq_F2_00_02(uint8_t **pp);
size_t vch_create_vrq_F2_00_02(uint8_t **pp);

size_t vch_sign(const uint8_t *ptr_raw, size_t sz_raw, const uint8_t *ptr_key, size_t sz_key, uint8_t **pp);
bool vch_validate(const uint8_t *ptr, size_t sz);
bool vch_validate_with_pem(const uint8_t *ptr, size_t sz, const uint8_t *ptr_pem, size_t sz_pem);
void vch_debug(const uint8_t *ptr, size_t sz);
