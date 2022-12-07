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

size_t vi_sign(const uint8_t *ptr_raw, size_t sz_raw, const uint8_t *ptr_key, size_t sz_key,
               uint8_t **pp, uint8_t alg);
bool vi_validate(const uint8_t *ptr, size_t sz);
bool vi_validate_with_pem(const uint8_t *ptr, size_t sz, const uint8_t *ptr_pem, size_t sz_pem);
void vi_dump(const uint8_t *ptr, size_t sz);

//

typedef void vi_provider_t;

void vi_provider_allocate(vi_provider_t **pp, bool is_vrq);
bool vi_provider_allocate_from_cbor(vi_provider_t **pp, const uint8_t *buf, size_t sz);
void vi_provider_free(vi_provider_t **pp);

bool vi_provider_has_attr_int(vi_provider_t *p, uint8_t attr_key);
bool vi_provider_has_attr_bool(vi_provider_t *p, uint8_t attr_key);
bool vi_provider_has_attr_bytes(vi_provider_t *p, uint8_t attr_key);

uint64_t vi_provider_get_attr_int_or_panic(vi_provider_t *p, uint8_t attr_key);
bool vi_provider_get_attr_bool_or_panic(vi_provider_t *p, uint8_t attr_key);
size_t vi_provider_get_attr_bytes_or_panic(vi_provider_t *p, uint8_t attr_key, uint8_t **buf);

bool vi_provider_set_attr_int(vi_provider_t *p, uint8_t attr_key, uint64_t attr_val);
bool vi_provider_set_attr_bool(vi_provider_t *p, uint8_t attr_key, bool attr_val);
bool vi_provider_set_attr_bytes(vi_provider_t *p, uint8_t attr_key, const uint8_t *buf, size_t sz);

bool vi_provider_remove_attr(vi_provider_t *p, uint8_t attr_key);
uint8_t vi_provider_attr_key_at(vi_provider_t *p, size_t n);

bool vi_provider_is_vrq(vi_provider_t *p);
size_t vi_provider_to_cbor(vi_provider_t *p, uint8_t **buf);
void vi_provider_dump(vi_provider_t *p);
size_t vi_provider_len(vi_provider_t *p);

bool vi_provider_sign(vi_provider_t *p, const uint8_t *ptr_key, size_t sz_key, uint8_t alg);
bool vi_provider_validate(vi_provider_t *p);
bool vi_provider_validate_with_pem(vi_provider_t *p, const uint8_t *ptr_pem, size_t sz_pem);

size_t vi_provider_get_signer_cert(vi_provider_t *p, uint8_t **buf);
void vi_provider_set_signer_cert(vi_provider_t *p, const uint8_t *buf, size_t sz);
size_t vi_provider_get_content(vi_provider_t *p, uint8_t **buf);
size_t vi_provider_get_signature_bytes(vi_provider_t *p, uint8_t **buf);
uint8_t vi_provider_get_signature_alg(vi_provider_t *p);

// https://github.com/AnimaGUS-minerva/voucher/blob/master/src/attr.rs
#define ATTR_ASSERTION                          (0x00)
#define ATTR_CREATED_ON                         (0x01)
#define ATTR_DOMAIN_CERT_REVOCATION_CHECKS      (0x02)
#define ATTR_EXPIRES_ON                         (0x03)
#define ATTR_IDEVID_ISSUER                      (0x04)
#define ATTR_LAST_RENEWAL_DATE                  (0x05)
#define ATTR_NONCE                              (0x06)
#define ATTR_PINNED_DOMAIN_CERT                 (0x07)
#define ATTR_PINNED_DOMAIN_PUBK                 (0x20) // vch only
#define ATTR_PINNED_DOMAIN_PUBK_SHA256          (0x21) // vch only
#define ATTR_PRIOR_SIGNED_VOUCHER_REQUEST       (0x40) // vrq only
#define ATTR_PROXIMITY_REGISTRAR_CERT           (0x41) // vrq only
#define ATTR_PROXIMITY_REGISTRAR_PUBK           (0x42) // vrq only
#define ATTR_PROXIMITY_REGISTRAR_PUBK_SHA256    (0x43) // vrq only
#define ATTR_SERIAL_NUMBER                      (0x08)

const char * attr_key_to_str(uint8_t attr_key) {
    switch (attr_key) {
        case ATTR_ASSERTION:                       return "ATTR_ASSERTION";
        case ATTR_CREATED_ON:                      return "ATTR_CREATED_ON";
        case ATTR_DOMAIN_CERT_REVOCATION_CHECKS:   return "ATTR_DOMAIN_CERT_REVOCATION_CHECKS";
        case ATTR_EXPIRES_ON:                      return "ATTR_EXPIRES_ON";
        case ATTR_IDEVID_ISSUER:                   return "ATTR_IDEVID_ISSUER";
        case ATTR_LAST_RENEWAL_DATE:               return "ATTR_LAST_RENEWAL_DATE";
        case ATTR_NONCE:                           return "ATTR_NONCE";
        case ATTR_PINNED_DOMAIN_CERT:              return "ATTR_PINNED_DOMAIN_CERT";
        case ATTR_PINNED_DOMAIN_PUBK:              return "ATTR_PINNED_DOMAIN_PUBK";
        case ATTR_PINNED_DOMAIN_PUBK_SHA256:       return "ATTR_PINNED_DOMAIN_PUBK_SHA256";
        case ATTR_PRIOR_SIGNED_VOUCHER_REQUEST:    return "ATTR_PRIOR_SIGNED_VOUCHER_REQUEST";
        case ATTR_PROXIMITY_REGISTRAR_CERT:        return "ATTR_PROXIMITY_REGISTRAR_CERT";
        case ATTR_PROXIMITY_REGISTRAR_PUBK:        return "ATTR_PROXIMITY_REGISTRAR_PUBK";
        case ATTR_PROXIMITY_REGISTRAR_PUBK_SHA256: return "ATTR_PROXIMITY_REGISTRAR_PUBK_SHA256";
        case ATTR_SERIAL_NUMBER:                   return "ATTR_SERIAL_NUMBER";
    }
    return "unknown";
}

#define ASSERTION_VERIFIED                      (0)
#define ASSERTION_LOGGED                        (1)
#define ASSERTION_PROXIMITY                     (2)

// https://animagus-minerva.github.io/voucher/doc/minerva_voucher/enum.SignatureAlgorithm.html
#define SA_ES256                                (0)
#define SA_ES384                                (1)
#define SA_ES512                                (2)
#define SA_PS256                                (3)

#endif // VOUCHER_IF_H

#ifdef __cplusplus
}
#endif