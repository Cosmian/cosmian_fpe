# -*- coding: utf-8 -*-
import unittest
from fpe import FF1


class TestFF1(unittest.TestCase):
    def test_encryption_decryption(self) -> None:
        ff1 = FF1(bytes([0] * 32), 2)
        plaintext = b"TEST_fpe"
        ciphertext = ff1.encrypt(plaintext)
        self.assertEqual(list(ciphertext), [135, 17, 208, 28, 48, 240, 214, 165])
        self.assertEqual(ff1.decrypt(ciphertext), plaintext)


if __name__ == "__main__":
    unittest.main()
