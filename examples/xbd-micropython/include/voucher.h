int vch_square(int);

size_t vch_get_voucher_jada(uint8_t **prt);
size_t vch_get_voucher_F2_00_02(uint8_t **prt);
size_t vch_get_masa_pem_F2_00_02(uint8_t **prt);
void vch_debug(const uint8_t *prt, size_t sz);
bool vch_validate(const uint8_t *prt, size_t sz);
bool vch_validate_with_pem(const uint8_t *prt, size_t sz, const uint8_t *prt_pem, size_t sz_pem);
