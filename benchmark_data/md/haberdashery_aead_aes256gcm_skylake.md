[//]: # (@generated)

| operation     | cycles              |
|---------------|---------------------|
| init          |   163.6779 ± 0.4181 |
| aad empty     |    44.2941 ± 0.3210 |
| decrypt empty |    45.8537 ± 0.5978 |
| encrypt empty |    44.2491 ± 0.2710 |

| bytes   | aad                 | decrypt             | encrypt             |
|---------|---------------------|---------------------|---------------------|
| 16b     |     3.6597 ± 0.0330 |     6.0470 ± 0.0312 |     7.7910 ± 0.0515 |
| 32b     |     2.5457 ± 0.0122 |     3.7336 ± 0.0209 |     4.8449 ± 0.0198 |
| 48b     |     2.1323 ± 0.0102 |     3.0643 ± 0.0158 |     3.8557 ± 0.0247 |
| 64b     |     1.9095 ± 0.0147 |     2.6919 ± 0.0153 |     3.3711 ± 0.0298 |
| 80b     |     1.8385 ± 0.0102 |     2.4894 ± 0.0161 |     3.1286 ± 0.0193 |
| 96b     |     0.8686 ± 0.0061 |     1.7464 ± 0.0110 |     2.1707 ± 0.0114 |
| 112b    |     0.9374 ± 0.0057 |     1.7380 ± 0.0110 |     2.4154 ± 0.0075 |
| 128b    |     0.9980 ± 0.0071 |     1.7119 ± 0.0117 |     2.3757 ± 0.0112 |
| 144b    |     1.1093 ± 0.0067 |     1.7734 ± 0.0078 |     2.3117 ± 0.0090 |
| 160b    |     1.1893 ± 0.0047 |     1.7346 ± 0.0117 |     2.2761 ± 0.0099 |
| 176b    |     1.2512 ± 0.0036 |     1.7338 ± 0.0113 |     2.2430 ± 0.0097 |
| 192b    |     0.6720 ± 0.0178 |     1.3360 ± 0.0082 |     1.5637 ± 0.0082 |
| 208b    |     0.6857 ± 0.0020 |     1.3534 ± 0.0085 |     1.7590 ± 0.0084 |
| 224b    |     0.7745 ± 0.0033 |     1.3989 ± 0.0259 |     1.7751 ± 0.0109 |
| 240b    |     0.8498 ± 0.0038 |     1.4291 ± 0.0074 |     1.7903 ± 0.0105 |
| 256b    |     0.9084 ± 0.0024 |     1.4232 ± 0.0096 |     1.7925 ± 0.0082 |
| 384b    |     0.4904 ± 0.0020 |     1.1261 ± 0.0109 |     1.2324 ± 0.0099 |
| 512b    |     0.5523 ± 0.0016 |     1.1025 ± 0.0075 |     1.2779 ± 0.0049 |
| 640b    |     0.5962 ± 0.0029 |     1.1173 ± 0.0090 |     1.2522 ± 0.0059 |
| 768b    |     0.4369 ± 0.0026 |     1.0078 ± 0.0046 |     1.0630 ± 0.0057 |
| 896b    |     0.4782 ± 0.0020 |     1.0214 ± 0.0077 |     1.1146 ± 0.0045 |
| 1kb     |     0.5135 ± 0.0013 |     1.0357 ± 0.0088 |     1.1216 ± 0.0090 |
| 1.125kb |     0.4171 ± 0.0021 |     0.9771 ± 0.0089 |     1.0002 ± 0.0036 |
| 1.25kb  |     0.4492 ± 0.0015 |     0.9852 ± 0.0068 |     1.0442 ± 0.0036 |
| 1.375kb |     0.4766 ± 0.0015 |     0.9951 ± 0.0043 |     1.0588 ± 0.0042 |
| 1.5kb   |     0.4061 ± 0.0009 |     0.9565 ± 0.0055 |     0.9767 ± 0.0062 |
| 1.625kb |     0.4311 ± 0.0008 |     0.9663 ± 0.0070 |     1.0109 ± 0.0053 |
| 1.75kb  |     0.4545 ± 0.0008 |     0.9733 ± 0.0060 |     1.0199 ± 0.0042 |
| 1.875kb |     0.4015 ± 0.0013 |     0.9412 ± 0.0064 |     0.9588 ± 0.0060 |
| 2kb     |     0.4221 ± 0.0016 |     0.9478 ± 0.0048 |     0.9908 ± 0.0045 |
| 2.125kb |     0.4418 ± 0.0009 |     0.9608 ± 0.0048 |     0.9969 ± 0.0036 |
| 2.25kb  |     0.3975 ± 0.0012 |     0.9345 ± 0.0041 |     0.9466 ± 0.0035 |
| 2.375kb |     0.4144 ± 0.0011 |     0.9429 ± 0.0045 |     0.9728 ± 0.0054 |
| 2.5kb   |     0.4319 ± 0.0008 |     0.9531 ± 0.0046 |     0.9807 ± 0.0049 |
| 2.625kb |     0.3944 ± 0.0016 |     0.9312 ± 0.0035 |     0.9360 ± 0.0027 |
| 2.75kb  |     0.4090 ± 0.0009 |     0.9349 ± 0.0045 |     0.9600 ± 0.0042 |
| 2.875kb |     0.4246 ± 0.0009 |     0.9455 ± 0.0047 |     0.9711 ± 0.0051 |
| 3kb     |     0.3924 ± 0.0008 |     0.9254 ± 0.0066 |     0.9385 ± 0.0081 |
| 3.125kb |     0.4058 ± 0.0009 |     0.9285 ± 0.0033 |     0.9554 ± 0.0068 |
| 3.25kb  |     0.4191 ± 0.0008 |     0.9384 ± 0.0043 |     0.9627 ± 0.0049 |
| 3.375kb |     0.3915 ± 0.0010 |     0.9238 ± 0.0038 |     0.9264 ± 0.0036 |
| 3.5kb   |     0.4031 ± 0.0010 |     0.9319 ± 0.0044 |     0.9476 ± 0.0048 |
| 3.625kb |     0.4164 ± 0.0010 |     0.9402 ± 0.0047 |     0.9536 ± 0.0048 |
| 3.75kb  |     0.3891 ± 0.0007 |     0.9226 ± 0.0041 |     0.9222 ± 0.0027 |
| 3.875kb |     0.4002 ± 0.0009 |     0.9243 ± 0.0044 |     0.9378 ± 0.0033 |
| 4kb     |     0.4120 ± 0.0009 |     0.9339 ± 0.0046 |     0.9451 ± 0.0045 |
| 16kb    |     0.3871 ± 0.0009 |     0.9091 ± 0.0034 |     0.9079 ± 0.0061 |
| 32kb    |     0.3814 ± 0.0009 |     0.9066 ± 0.0070 |     0.8986 ± 0.0039 |
| 48kb    |     0.3805 ± 0.0011 |     0.9024 ± 0.0038 |     0.8950 ± 0.0043 |
| 64kb    |     0.3803 ± 0.0011 |     0.9021 ± 0.0038 |     0.8977 ± 0.0067 |
| 80kb    |     0.3796 ± 0.0008 |     0.9024 ± 0.0039 |     0.8949 ± 0.0066 |
| 96kb    |     0.3787 ± 0.0009 |     0.9030 ± 0.0045 |     0.8927 ± 0.0037 |
| 112kb   |     0.3793 ± 0.0008 |     0.9029 ± 0.0053 |     0.8931 ± 0.0042 |
| 128kb   |     0.3787 ± 0.0006 |     0.9019 ± 0.0044 |     0.8919 ± 0.0031 |
| 256kb   |     0.3789 ± 0.0008 |     0.8998 ± 0.0035 |     0.8963 ± 0.0045 |
| 384kb   |     0.3795 ± 0.0009 |     0.9021 ± 0.0048 |     0.8896 ± 0.0035 |
| 512kb   |     0.3790 ± 0.0006 |     0.8998 ± 0.0024 |     0.8950 ± 0.0036 |
| 640kb   |     0.3793 ± 0.0007 |     0.9072 ± 0.0061 |     0.8970 ± 0.0039 |
| 768kb   |     0.3798 ± 0.0007 |     0.9029 ± 0.0045 |     0.8947 ± 0.0032 |
| 896kb   |     0.3800 ± 0.0009 |     0.9039 ± 0.0043 |     0.8942 ± 0.0048 |
| 1mb     |     0.3805 ± 0.0011 |     0.9036 ± 0.0042 |     0.8979 ± 0.0049 |

| metadata  |                                                                  |
|-----------|------------------------------------------------------------------|
| algorithm | aes256gcm                                                        |
| cpu       | IntelSkylake (06_55H)                                            |
| library   | haberdashery                                                     |
| path      | haberdashery_aead_aes256gcm_skylake                              |
| primitive | aead                                                             |
| profile   | skylake                                                          |
| version   | 7e569131a557e16a39f360b55578d58e62d977807ff9a5963d70b2d38d39f8b8 |