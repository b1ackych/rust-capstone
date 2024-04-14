# A secure, containerised password manager

User passwords are stored in containers, which protects the data using encryption. He’s using master-password for these purposes, which encrypts container (via public key) and is stored on a client-side. 

# How to use?

1. **Clone the repository:**
   ```bash
   git clone git@github.com:b1ackych/rust-capstone
   ```

2. **Run the server:**
   ```bash
    cargo run --bin server
   ```

3. **Run the client separetely and start using application:**
   ```bash
    cargo run --bin client
   ```


For more detailed information, visit this doc:

(  https://docs.google.com/document/d/18Wg171E8y-jBw40NwiinwK5XhC_gT5S71zazCBUMX4A/edit  )