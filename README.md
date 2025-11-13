# Decentralized Marketplace

## Project Title
**Decentralized Marketplace on Stellar Blockchain**

## Project Description
A peer-to-peer marketplace smart contract built on the Stellar blockchain using Soroban SDK. This decentralized platform enables users to create listings for goods and services, and facilitates secure transactions using XLM or custom Stellar tokens. The contract eliminates intermediaries, reduces transaction costs, and provides transparency through blockchain technology.

The marketplace operates entirely on-chain, ensuring that all listings and transactions are immutable, transparent, and verifiable. Sellers can list their items with custom prices, while buyers can browse and purchase items directly through the smart contract.

## Project Vision
Our vision is to revolutionize e-commerce by creating a trustless, decentralized marketplace that empowers individuals worldwide to trade goods and services without relying on centralized platforms. By leveraging the Stellar blockchain's speed and low transaction costs, we aim to:

- **Democratize Commerce**: Enable anyone with internet access to participate in global trade
- **Eliminate Middlemen**: Remove unnecessary intermediaries and their associated fees
- **Ensure Transparency**: Provide complete visibility into all marketplace transactions
- **Build Trust**: Use blockchain technology to create a tamper-proof record of all listings and sales
- **Foster Financial Inclusion**: Allow users in underbanked regions to participate in digital commerce using Stellar's accessible infrastructure

## Key Features

### 1. **Create Listings**
- Sellers can create product/service listings with title, description, and price
- Each listing receives a unique identifier for easy tracking
- Listings are timestamped for chronological tracking
- Prices are denominated in stroops (smallest XLM unit) for precise pricing

### 2. **Purchase Items**
- Buyers can securely purchase listed items through the smart contract
- Authentication ensures only authorized users can make purchases
- Automatic validation prevents sellers from purchasing their own items
- Listings are automatically marked as inactive after successful purchase

### 3. **View Listing Details**
- Anyone can query and view detailed information about any listing
- Retrieve seller information, pricing, descriptions, and availability status
- Check whether items are still available for purchase

### 4. **Marketplace Statistics**
- Real-time tracking of total listings created
- Monitor active (available) listings
- Track completed sales transactions
- Provides transparency and insights into marketplace activity

### 5. **Security Features**
- Authentication required for all state-changing operations
- Seller verification to prevent unauthorized listing modifications
- Buyer verification to ensure legitimate purchases
- Input validation to prevent invalid data entries

## Future Scope

### Short-term Enhancements
1. **Escrow Functionality**: Implement an escrow mechanism to hold funds until buyers confirm receipt of goods
2. **Cancel Listing**: Allow sellers to deactivate their listings before sale
3. **Update Listing**: Enable sellers to modify price and descriptions of active listings
4. **Search and Filter**: Add functionality to search listings by title, price range, or seller

### Medium-term Development
5. **Rating System**: Implement buyer and seller rating mechanisms for reputation building
6. **Multi-token Support**: Enable payments using custom Stellar tokens beyond XLM
7. **Dispute Resolution**: Create a decentralized dispute resolution mechanism
8. **Category Management**: Organize listings into categories for better discoverability
9. **Image Storage Integration**: Integrate with decentralized storage (IPFS) for product images

### Long-term Vision
10. **Auction Mechanism**: Add support for auction-style listings with bidding
11. **Shipping Integration**: Partner with logistics providers for automated shipping
12. **Multi-chain Support**: Expand to other blockchain networks through bridges
13. **DAO Governance**: Implement decentralized governance for marketplace rules and fees
14. **NFT Marketplace**: Extend functionality to support NFT trading
15. **Automated Market Making**: Integration with Stellar DEX for automatic token swaps
16. **Social Features**: Add messaging between buyers and sellers
17. **Analytics Dashboard**: Provide comprehensive analytics for users and marketplace trends

---

## Technical Details

### Prerequisites
- Rust programming language
- Soroban SDK
- Stellar account for deployment
- Soroban CLI tools

### Contract Functions

#### `create_listing(seller, title, description, price) -> u64`
Creates a new marketplace listing and returns its unique ID.

#### `purchase_item(buyer, listing_id)`
Allows a buyer to purchase an item from an active listing.

#### `get_listing(listing_id) -> Listing`
Retrieves detailed information about a specific listing.

#### `get_marketplace_stats() -> MarketplaceStats`
Returns overall marketplace statistics including total listings, active listings, and completed sales.

### Data Structures

**Listing**
- `listing_id`: Unique identifier
- `seller`: Seller's Stellar address
- `title`: Product/service title
- `description`: Detailed description
- `price`: Price in stroops
- `is_active`: Availability status
- `created_at`: Timestamp of creation

**MarketplaceStats**
- `total_listings`: Total number of listings created
- `active_listings`: Currently available listings
- `completed_sales`: Number of successful transactions

---

## Contributing
We welcome contributions from the community! Please feel free to submit issues, fork the repository, and create pull requests for any improvements.

## License
This project is open-source and available under the MIT License.

## Contact
For questions, suggestions, or collaborations, please reach out through our community channels.

---


## Contract Details
CAEVAEUPRXYRZB5LMGUL6SSU36HAC2WQVEQL3RC7L6AKQCLOHQF4Z5SA