/* @generated */

#include "broadwell.h"

int main(void) {
  haberdashery_broadwell_siv_mac_t mac;
  unsigned char key[HABERDASHERY_BROADWELL_SIVMAC_KEY_LEN] = {0};
  unsigned char tag[HABERDASHERY_BROADWELL_SIVMAC_TAG_LEN] = {0};
  unsigned char message[1] = {0};

  if (!haberdashery_sivmac_broadwell_is_supported()) {
    return 0;
  }

  if (!haberdashery_sivmac_broadwell_init(&mac, key, sizeof(key))) {
    return 1;
  }
  if (!haberdashery_sivmac_broadwell_sign(&mac, message, sizeof(message), tag, sizeof(tag))) {
    return 2;
  }
  if (!haberdashery_sivmac_broadwell_verify(&mac, message, sizeof(message), tag, sizeof(tag))) {
    return 3;
  }
  return 0;
}
