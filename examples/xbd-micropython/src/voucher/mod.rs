mod cose_data;

#[cfg(feature = "std")]
mod validate_std;
#[cfg(not(feature = "std"))]
mod validate;

use cose_data::{CoseData, CoseSignature};
pub use cose_data::SignatureAlgorithm;

pub static VOUCHER_JADA: &[u8] = core::include_bytes!(
    concat!(env!("CARGO_MANIFEST_DIR"), "/files/voucher_jada123456789.vch"));
pub static VOUCHER_F2_00_02: &[u8] = core::include_bytes!(
    concat!(env!("CARGO_MANIFEST_DIR"), "/files/voucher_00-D0-E5-F2-00-02.vch"));
pub static MASA_PEM_F2_00_02: &[u8] = core::include_bytes!(
    concat!(env!("CARGO_MANIFEST_DIR"), "/files/masa_00-D0-E5-F2-00-02.crt"));

pub struct Voucher(CoseSignature);

impl Voucher {
    pub fn from(raw_voucher: &[u8]) -> Self {
        if let Ok(cose_data) = CoseData::decode(raw_voucher) {
            match cose_data {
                CoseData::CoseSignOne(cose_signature) => return Self(cose_signature),
                CoseData::CoseSign(_) => unimplemented!("Only `CoseSign1` vouchers are supported"),
            }
        } else {
            panic!("Failed to decode raw voucher");
        };
    }

    pub fn validate(&self, masa_pem: Option<&[u8]>) -> bool {
        if 0 == 1 { self.dump(); } // debug

        #[cfg(feature = "std")]
        {
            validate_std::validate(masa_pem, self.to_validate()).is_ok()
        }
        #[cfg(not(feature = "std"))]
        {
            validate::validate(masa_pem, self.to_validate())
        }
    }

    fn to_validate(&self) -> (Option<&[u8]>, &[u8], &SignatureAlgorithm, &[u8]) {
        let (signature, alg) = self.get_signature();

        (self.get_signer_cert(), signature, alg, &self.0.to_verify)
    }

    pub fn get_signature(&self) -> (&[u8], &SignatureAlgorithm) {
        (&self.0.signature, &self.0.signature_type)
    }

    pub fn get_signer_cert(&self) -> Option<&[u8]> {
        let signer_cert = &self.0.signer_cert;

        if signer_cert.len() > 0 { Some(signer_cert) } else { None }
    }

    fn dump(&self) {
        CoseData::dump(&self.0);
    }
}
