/* @generated */

#include "broadwell.h"

int main(void) {
  haberdashery_broadwell_aes_256_gcm_t aead;
  unsigned char key[HABERDASHERY_BROADWELL_AES_256_GCM_KEY_LEN] = {0};
  unsigned char nonce[HABERDASHERY_BROADWELL_AES_256_GCM_NONCE_LEN] = {0};
  unsigned char tag[HABERDASHERY_BROADWELL_AES_256_GCM_TAG_LEN] = {0};
  unsigned char aad[1] = {0};
  unsigned char plaintext[1] = {0};
  unsigned char ciphertext[1] = {0};

  if (!haberdashery_aes256gcm_broadwell_is_supported()) {
    return 0;
  }

  if (!haberdashery_aes256gcm_broadwell_init(&aead, key, sizeof(key))) {
    return 1;
  }
  if (!haberdashery_aes256gcm_broadwell_encrypt(&aead, nonce, sizeof(nonce), aad, sizeof(aad), plaintext, sizeof(plaintext), ciphertext, sizeof(ciphertext), tag, sizeof(tag))) {
    return 2;
  }
  if (!haberdashery_aes256gcm_broadwell_decrypt(&aead, nonce, sizeof(nonce), aad, sizeof(aad), ciphertext, sizeof(ciphertext), tag, sizeof(tag), plaintext, sizeof(plaintext))) {
    return 3;
  }
  return 0;
}
