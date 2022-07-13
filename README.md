# Peerage

Peerage (pronounced peer-aazh not peer-age!) is a P2P communication and file sharing tool  with monetization abilities. The app is made of "rooms" and these rooms are customizable through HTML, CSS and JavaScript. But the room resides on your PC, not a server, and anyone who connect to your room accepts some of the load. Peers discover each othr through a ledger and muliicast DNS.  Speakinh of ledgers you can make your room VIP and have people who pay the token registered in the ledger to acces it. The token is not a cryptocurrency, it's not mined, it's just, like, a cadaver for the fiat currency. You can do "other" things with this token too. The ledger is shared by all members, only one aspect of ledger is active (aka uncompressed) active at time. The other aspects are compressed.

It's highly WIP. Any financial support during the development of this application is appreciated. Also you can do pull requests. 

## What I have made so for

### Peerage-Utils

This project contains all the utilities. Right now it hosts traits and this MASSIVE binary tool I have written. 


### Peerage-Hash

The custom hash that I am going to use all across the software. It's kinda slow curently but I will work on it to make it faster.

### Peerage-Coll

A dynamic list, like any dynamic list, I am using Rust arrays to hold the dat and it can be turned into an iterator as well (nothing spectial, I just created an iterator wrapper and implemented IntoIter. I will implement from iter too but there's already a function for that). The dynamic list (which I call "collection") has a macro (`coll!`) too, just like vec.

### Peerage-RTree

I have created this datatype for holding the room data and the Merkle DAG required called 'Rapid Tree'. Keep in mind that I am currenctly working  on it so it's not completed yet.

### Peerage-Macros

The necessary proc macros.

### Peerage-Holder

This is a holder enum, a bit like Option, a bit like Box. I need to rework it a bit. It can be used to hold several types into one type.



The rest of the projects are empty, don't bother. But slowly, they will be filled.


### I wall help! What should I do?

You can help me by "buying me a coffee" (but I'm not going to buy coffee with it lol gonna buy tea).

![](coffee.png)

Send me any amount you can to this ERC20 Ether address. It's a Nano S Ledger address, I bought it the moment I started working in crypto. PLEASE buy a hardware wallet! You can use them through MetaMask!


`0x503A3B99c0c086fD81186a0d5ac815eBd15e5983`

## Final Words

This is a dream I have and as you can see I am going forward, changing things, finalizing things, it would be great if my needs are taken care of, I don't need much money --- I can live off with 300 USD a month. Yes I am e-begging but I don't care. I know Peerage is going ot be revolutionary, something great, something I can be proud of.


My Discord: Chubak#7400