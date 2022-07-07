# Peerage

Peerage (pronounced peer-aazh) is an experimental P2P network written in Rust with minimal use of libraries. 

## The Nature of Experiment

The experience itself is comprised of rooms. Each member of the room shares files for other peers in the room to download and the other members share these files as well. There's a room owner. The settings for the room resides on the owner's PC. The rooms are listed for people to choose from.

The files are saved among different PCs as Merkle trees.

## The Merkle Tree and the Peerage Hash

The Merkle tree nodes are hashed with a custom hash called Peerage Hash. PH is a semi-cryptographic hash function that hashes each 256 chunk of the file data (and pads if necessary).


The hash is a 32-bit little-endian number. Each 256 byte chunk has 32 rounds which I'll explain what a round does in a minute. At the end of each round of it's odd the bit of that round number will be 1 else 0. Then we put aside this 32-bit number for something that I will tell you later.

But what is the round? As I said Peerage Hash is semi-cryptographic and its main aim is to serve as am experiment. PH in no way should be used in production (duh!) and we only use PH in Peerage as Merkle tree hash because the nature of this project is to be experimental.

Anyways. As we said we have a 256 byte chunk of our file, and a 32-bit number S that we need to flip the bits based on the result of each round. 

To do each round first first we need to split each chunk into 32-bit words. Now we have 8 chunks, A, B, C, D, E, F, G, H.

The round has 8 steps.

Step 1:
A = B ^ C

Step 2:
B = A >> F

Step 3:
C = (E >> 16) | (E << (32 - 16))

Step 4:
D = (B | C) & D

Step 5:
C = (G >> 12) | (G << (32 - 12)) + (B >> 18) | (B << (32 - 18))

Step 6:
F = (A ^ B) | (H & G) & (A | F)

Step 7:
G = (A >> 12) | (B << 8)

Step 8
H = (A | B) ^ C

So now we add these numbers togther. Just a straightforward addition, nothing fancy. If It was odd, then we set the bit as 1. Else 0.

Then we repeat this 32 times, with the same chunk, but this time, we scooch over the variables by one. So A becomes B, B becomes C...

(A, B, C, D, E, F, G, H) = (B, C, D, E, F, G, H, A)

At the end of the 32 bits, we have made 4 cycles of the variables.

After all the bits in S are set, we save it, and we skip over to the next chunk. At the end we are left with howmuchever 32-bit S values. We XOR them from number 1 to number n and there, we have our hash value!




## Parts of the Application

### Binary Utils

Binary utils are used across the application for binary needs. It contains bit, byte, and word.