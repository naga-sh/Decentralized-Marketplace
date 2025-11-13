#![allow(non_snake_case)]
#![no_std]
use soroban_sdk::{contract, contracttype, contractimpl, log, Env, Symbol, String, Address, symbol_short};

// Structure to store listing details
#[contracttype]
#[derive(Clone)]
pub struct Listing {
    pub listing_id: u64,
    pub seller: Address,
    pub title: String,
    pub description: String,
    pub price: i128,        // Price in stroops (1 XLM = 10,000,000 stroops)
    pub is_active: bool,
    pub created_at: u64,
}

// Structure to track marketplace statistics
#[contracttype]
#[derive(Clone)]
pub struct MarketplaceStats {
    pub total_listings: u64,
    pub active_listings: u64,
    pub completed_sales: u64,
}

// Key for marketplace statistics
const MARKETPLACE_STATS: Symbol = symbol_short!("MKT_STATS");

// Counter for generating unique listing IDs
const LISTING_COUNT: Symbol = symbol_short!("LIST_CNT");

// Enum for mapping listing ID to listing data
#[contracttype]
pub enum ListingBook {
    Listing(u64)
}

#[contract]
pub struct MarketplaceContract;

#[contractimpl]
impl MarketplaceContract {
    
    /// Creates a new listing in the marketplace
    /// Returns the unique listing ID
    pub fn create_listing(
        env: Env,
        seller: Address,
        title: String,
        description: String,
        price: i128
    ) -> u64 {
        // Verify the seller is the caller
        seller.require_auth();
        
        // Validate price is positive
        if price <= 0 {
            log!(&env, "Price must be greater than zero");
            panic!("Invalid price");
        }
        
        // Get and increment listing counter
        let mut listing_count: u64 = env.storage().instance().get(&LISTING_COUNT).unwrap_or(0);
        listing_count += 1;
        
        // Get current timestamp
        let timestamp = env.ledger().timestamp();
        
        // Create new listing
        let new_listing = Listing {
            listing_id: listing_count,
            seller: seller.clone(),
            title,
            description,
            price,
            is_active: true,
            created_at: timestamp,
        };
        
        // Update marketplace stats
        let mut stats = Self::get_marketplace_stats(env.clone());
        stats.total_listings += 1;
        stats.active_listings += 1;
        
        // Store the listing
        env.storage().instance().set(&ListingBook::Listing(listing_count), &new_listing);
        env.storage().instance().set(&LISTING_COUNT, &listing_count);
        env.storage().instance().set(&MARKETPLACE_STATS, &stats);
        
        env.storage().instance().extend_ttl(5000, 5000);
        
        log!(&env, "Listing created with ID: {}", listing_count);
        listing_count
    }
    
    /// Purchases an item from the marketplace
    /// Marks the listing as inactive after successful purchase
    pub fn purchase_item(
        env: Env,
        buyer: Address,
        listing_id: u64
    ) {
        // Verify the buyer is the caller
        buyer.require_auth();
        
        // Get the listing
        let mut listing = Self::get_listing(env.clone(), listing_id);
        
        // Validate listing exists and is active
        if listing.listing_id == 0 {
            log!(&env, "Listing not found");
            panic!("Listing does not exist");
        }
        
        if !listing.is_active {
            log!(&env, "Listing is no longer active");
            panic!("Listing not active");
        }
        
        // Prevent seller from buying their own item
        if buyer == listing.seller {
            log!(&env, "Seller cannot purchase their own item");
            panic!("Invalid purchase");
        }
        
        // Mark listing as inactive (sold)
        listing.is_active = false;
        
        // Update marketplace stats
        let mut stats = Self::get_marketplace_stats(env.clone());
        stats.active_listings -= 1;
        stats.completed_sales += 1;
        
        // Store updated data
        env.storage().instance().set(&ListingBook::Listing(listing_id), &listing);
        env.storage().instance().set(&MARKETPLACE_STATS, &stats);
        
        env.storage().instance().extend_ttl(5000, 5000);
        
        log!(&env, "Item purchased - Listing ID: {}", listing_id);
    }
    
    /// Retrieves details of a specific listing
    pub fn get_listing(env: Env, listing_id: u64) -> Listing {
        let key = ListingBook::Listing(listing_id);
        
        env.storage().instance().get(&key).unwrap_or(Listing {
            listing_id: 0,
            seller: Address::from_string(&String::from_str(&env, "GAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAWHF")),
            title: String::from_str(&env, "Not Found"),
            description: String::from_str(&env, "Not Found"),
            price: 0,
            is_active: false,
            created_at: 0,
        })
    }
    
    /// Retrieves marketplace statistics
    pub fn get_marketplace_stats(env: Env) -> MarketplaceStats {
        env.storage().instance().get(&MARKETPLACE_STATS).unwrap_or(MarketplaceStats {
            total_listings: 0,
            active_listings: 0,
            completed_sales: 0,
        })
    }
}
