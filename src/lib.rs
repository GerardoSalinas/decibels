//!
//! Stylus Hello World
//!
//! The following contract implements the Counter example from Foundry.
//!
//! ```solidity
//! contract Counter {
//!     uint256 public number;
//!     function setNumber(uint256 newNumber) public {
//!         number = newNumber;
//!     }
//!     function increment() public {
//!         number++;
//!     }
//! }
//! ```
//!
//! The program is ABI-equivalent with Solidity, which means you can call it from both Solidity and Rust.
//! To do this, run `cargo stylus export-abi`.
//!
//! Note: this code is a template-only and has not been audited.
//!
// Allow `cargo stylus export-abi` to generate a main function.
#![cfg_attr(not(any(test, feature = "export-abi")), no_main)]
#![cfg_attr(not(any(test, feature = "export-abi")), no_std)]

#[macro_use]
extern crate alloc;

// use alloc::vec::Vec;

/// Import items from the SDK. The prelude contains common traits and macros.
// use stylus_sdk::{alloy_primitives::U256, prelude::*};
use stylus_sdk::{
    alloy_primitives::{address, U256, Address}, 
    prelude::*, 
    msg,
    call::transfer_eth,
};
use alloc::vec::Vec;

// Define some persistent storage using the Solidity ABI.
// `Counter` will be the entrypoint.

sol_storage! {
    pub struct Song {
        address artist;          
        uint256 song_id;         
        uint256 price_per_play;  
        uint256 plays;           
    }
}

sol_storage! {
    pub struct User {
        address user_address;    
        uint256 balance;         
    }
}

sol_storage! {
    #[entrypoint]
    pub struct MusicStreamingPlatform {
        User listener;
        mapping(uint256 => Song) songs;
        uint256 total_songs;   
        bool initialized;
        mapping(address => bool) artists; 
    }
}

#[public]
impl MusicStreamingPlatform {
    
    /// Inicializar la plataforma
    pub fn initialize(&mut self) -> Result<(), Vec<u8>> {
        // let mut new_song1 = Song {
        //     artist: Address::from_str("0xcadb505909332A4190aa82b12F09Ff3572aABb55"),
        //     song_id: U256::from(10),
        //     price_per_play: U256::from(2300000000000000u64), 
        //     plays: U256::from(1542),
        // };

        let mut song1 = self.songs.setter(U256::from(0));
        song1.artist.set(address!("0xcadb505909332A4190aa82b12F09Ff3572aABb55"));
        song1.song_id.set(U256::from(10));
        song1.price_per_play.set(U256::from(2300000000000000u64)); 
        song1.plays.set(U256::from(1542));
        

        let mut song2 = self.songs.setter(U256::from(1));
        song2.artist.set(address!("0x1080b094cFa7f8e0326530e99391A8A8da0336a1"));
        song2.song_id.set(U256::from(11));
        song2.price_per_play.set(U256::from(5600000000000000u64)); 
        song2.plays.set(U256::from(12));
        
        let mut song3 = self.songs.setter(U256::from(2));
        song3.artist.set(address!("0xcadb505909332A4190aa82b12F09Ff3572aABb55"));
        song3.song_id.set(U256::from(12));
        song3.price_per_play.set(U256::from(9000000000000000u64)); 
        song3.plays.set(U256::from(154));
        
        self.total_songs.set(U256::from(3));
        self.initialized.set(true);
        
        Ok(())
    }

    
    pub fn get_config(&self) -> U256 {
        self.total_songs.get()
    }

    
    pub fn register_artist(&mut self) {
        let sender = msg::sender();
        self.artists.setter(sender).set(true);
    }
    
    // pub fn upload_song(&mut self, price_per_play: U256) -> Result<U256, Vec<u8>> {
    //     let sender = msg::sender();
        
    //     // if !self.is_artist(sender) {
    //     //     return Err(b"Solo artistas registrados pueden subir canciones".to_vec());
    //     // }
    //     if self.initialized.get() == false {
    //         self.initialize();
    //     }
        
    //     let current_total = self.total_songs.get();
    //     let new_song_id = current_total + U256::from(1);
        
    //     // Crear nueva canción
    //     let mut new_song = self.songs.setter(new_song_id);
    //     new_song.artist.set(sender);
    //     new_song.song_id.set(new_song_id);
    //     new_song.price_per_play.set(price_per_play);
    //     new_song.plays.set(U256::ZERO);
        
    //     // Actualizar contador total
    //     self.total_songs.set(new_song_id);
        
    //     Ok(new_song_id)
    // }
    
    /// Obtener información de canción
    pub fn get_song(&self, song_id: U256) -> (Address, U256, U256, U256) {
        let song = self.songs.get(song_id);
        (
            song.artist.get(),
            song.song_id.get(),
            song.price_per_play.get(),
            song.plays.get()
        )
    }
    
    /// Obtener número total de canciones
    pub fn get_total_songs(&self) -> U256 {
        self.total_songs.get()
    }

    
    /// Reproducir canción con micropago en ETH
    #[payable]
    pub fn play_song(&mut self, song_id: U256) -> Result<(), Vec<u8>> {
        self.initialize();
        // Verificar que la canción existe
        if song_id == U256::ZERO || song_id > self.total_songs.get() {
            return Err(b"Cancion no existe".to_vec());
        }
        
        // Obtener información de la canción y guardar valores necesarios
        let (artist_address, required_price, current_plays) = {
            let song = self.songs.get(song_id);
            (
                song.artist.get(),
                song.price_per_play.get(),
                song.plays.get()
            )
        };
        
        // Verificar que se envió el monto correcto
        // if msg::value() != required_price {
        //     return Err(b"Monto incorrecto para reproducir".to_vec());
        // }
        
        // El ETH ya fue descontado de la wallet del usuario cuando envió la transacción
        transfer_eth(artist_address, msg::value())?;
        
        // Incrementar contador de reproducciones (ahora podemos usar mutable borrow)
        // let mut song_mut = self.songs.setter(song_id);
        // song_mut.plays.set(current_plays + U256::from(1));
        
        Ok(())
    }
    
    /// Obtener reproducciones de una canción
    pub fn get_song_plays(&self, song_id: U256) -> U256 {
        if song_id == U256::ZERO || song_id > self.total_songs.get() {
            return U256::ZERO;
        }
        let song = self.songs.get(song_id);
        song.plays.get()
    }

    /// Transfer funds genérico (para propinas, etc.)
    #[payable]
    pub fn transfer_funds(&mut self, artist_address: Address) -> Result<(), Vec<u8>> {
        // Verificar que se envió ETH
        if msg::value() == U256::ZERO {
            return Err(b"Debe enviar ETH para transferir".to_vec());
        }
        
        // Transferir todo el ETH enviado
        transfer_eth(artist_address, msg::value())?;
        
        Ok(())
    }

    /// Obtener dirección del usuario actual
    pub fn get_user_address(&self) -> Address {
        msg::sender()
    }

    /// Obtener precio por reproducción
    pub fn get_price_per_play(&self, song_id: U256) -> U256 {
        if song_id == U256::ZERO || song_id > self.total_songs.get() {
            return U256::ZERO;
        }
        let song = self.songs.get(song_id);
        song.price_per_play.get()
    }

    /// Verificar si una canción existe
    pub fn song_exists(&self, song_id: U256) -> bool {
        song_id != U256::ZERO && song_id <= self.total_songs.get()
    }
}


