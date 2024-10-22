# Tweet Escrow Solana Smart Contract

### A marketplace that shows a list of Listings (from Makers) for Takers to accept and earn SOL or another token, once the Taker does some work that is validated.

When a Taker accepts a Listing, the Maker and Taker enter into a Deal

For each Listing, a Maker:
* Sets a contract address for a Token on the Listing (optional)
* If the Listing includes a Contract Address, the Maker sets a price (in that Token) for the Listing, otherwise sets a price for the Offer in SOL. A Maker can make a non-zero Offer price that is either SOL, Token, or both SOL and Token
* Sets Text content (text max 280 chars, the text max is adjustable by us)
* Adds expiration date for their Listing (timestamp)
* Each Listing generates a new PDA so all deposits/withdrawals are tracked in one escrow address (the Deal)

Taker:
* Sets their own Withdrawal wallet address
* Can accept Listings made by Makers
* Once a Listing is accepted, a Maker and Taker enter into a Deal
* Can "Negotiate" Deal, in that they can suggest: Text edits from the Listing, Listing price (edits to price of either SOL or Token or both) - if a Deal is being Negotiated, a Maker must approve the new Deal terms suggested by the Taker

Once a Taker has accepted an Listing, the Maker must deposit the exact amount of Token, Sol or Both into the deposit contract

Once the Maker has deposited into the Deal deposit contract, the Taker must perform the work (validated in our app) by a certain timestamp (settable by us)

Once the work has been validated, the Taker can withdraw the deposited funds to their preconfigured Withdrawal address

If the work is not valid, the Maker will be refunded in full

Variables us as Admins need to control
* Deal validation expiration timestamp (12 hours by default, as in, Taker has 12 hours to do the work)
* If a deal is valid or not
* Percent cut of each deal (of either SOL, Token, or Both if both set)
* Addresses for which to send cut (cut will be divided amongst these addresses, could be multiple addresses)