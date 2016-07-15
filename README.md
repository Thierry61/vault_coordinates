Find your star in the galaxy (or more prosaically your node in the XOR space) by using vault_coordinates utility and passing the id of your vault to it.

- git clone https://github.com/Thierry61/vault_coordinates.git
- cd vault_coordinates
- cargo build
- ./target/debug/vault_coordinates *your-id*

This returns the (x, y) coordinates of your vault. For example:

./target/debug/vault_coordinates.exe 9ecfa3..

returns (x, y) = (107, 187)

