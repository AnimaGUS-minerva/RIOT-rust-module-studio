int vch_square(int);

size_t vch_get_voucher_jada(uint8_t **ptr);
size_t vch_get_voucher_F2_00_02(uint8_t **ptr);
size_t vch_get_masa_pem_F2_00_02(uint8_t **ptr);
size_t vch_get_key_pem_02_00_2E(uint8_t **ptr);
size_t vch_get_device_crt_02_00_2E(uint8_t **ptr);
void vch_debug(const uint8_t *ptr, size_t sz);
bool vch_validate(const uint8_t *ptr, size_t sz);
bool vch_validate_with_pem(const uint8_t *ptr, size_t sz, const uint8_t *ptr_pem, size_t sz_pem);
