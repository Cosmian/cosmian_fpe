class FF1:
    """A struct for performing FF1 encryption and decryption operations
    using the default 10 Feistel rounds
    """

    def __init__(self, key: bytes, radix: int): ...
    def encrypt(self, plaintext: bytes) -> bytes:
        """Format preserving encryption of binary numeral string

        Args:
            plaintext (bytes): data to encrypt

        Returns:
            bytes: ciphertext
        """
    def decrypt(self, plaintext: bytes) -> bytes:
        """Format preserving decryption of binary numeral string

        Args:
            ciphertext (bytes): data to decrypt

        Returns:
            bytes: plaintext
        """
