Find your star in the galaxy (or more prosaically your node in the XOR space) by using vault_coordinates utility and passing the id of your vault to it.

- git clone https://github.com/Thierry61/vault_coordinates.git
- cd vault_coordinates
- cargo build
- ./target/debug/vault_coordinates *your-id*

This returns the (x, y) coordinates of your vault. For example:

./target/debug/vault_coordinates 9ecfa3..

returns (x, y) = (107, 187)

The vault id is visible in the routing table size lines of the vault log. For example,
in **Node(9ecfa3..)  PeerId(08da..) - Routing Table size: 15** the vault id is **9ecfa3**
