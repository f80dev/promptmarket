#![no_std]
use multiversx_sc::derive_imports::*;
#[allow(unused_imports)]
use multiversx_sc::imports::*;

//modele https://github.com/multiversx/mx-sdk-rs/blob/master/contracts/examples/crowdfunding-esdt/src/crowdfunding_esdt.rs

//On utilise des références plutôt que les données directement afin de limité la quantité de mémoire

//voir la structure https://github.com/multiversx/mx-sdk-rs/blob/master/contracts/examples/lottery-esdt/src/lottery_info.rs

#[derive(TypeAbi, TopEncode, TopDecode,NestedEncode,NestedDecode)]
pub struct Prompt<M: ManagedTypeApi> {
    pub text:ManagedBuffer<M>,          //contenu du prompt ou address pointant sur un prompt
    pub owner:usize,                    //Propriétaire
    pub start:u32,                       //Date de lancement
    pub server:usize                    //Server demandé
}

#[derive(TypeAbi, TopEncode, TopDecode,NestedEncode,NestedDecode)]
pub struct Server<M: ManagedTypeApi> {
    pub title:ManagedBuffer<M>,             //Promotion du moteur
    pub price:BigUint<M>,               //Montant proposé
    pub token:EgldOrEsdtTokenIdentifier<M>,
    pub model:u16,                       //Modele demandé
    pub inference:u16,                  //Inférence
    pub scale:u16,                      //résolution
    pub owner:usize,                    //Propriétaire
}

#[derive(TypeAbi, TopEncode, TopDecode,ManagedVecItem,NestedEncode,NestedDecode)]
pub struct Render<M: ManagedTypeApi> {
    pub prompt_id:usize,
    pub url:ManagedBuffer<M>,
    pub creator:usize,
}


/// An empty contract. To be used as a template when starting a new contract from scratch.
#[multiversx_sc::contract]
pub trait PromptMarket {
    // Storage **************************************************************************************************************************************

    //voir https://docs.multiversx.com/developers/developer-reference/storage-mappers#vecmapper-vs-linkedlistmapper
    //voir https://docs.rs/elrond-wasm/latest/elrond_wasm/storage/mappers/struct.LinkedListMapper.html

    // #[view(getMarge)]
    // #[storage_mapper("royalties")]
    // fn sc_royalties(&self) -> SingleValueMapper<u64>;        //Contient une référence d'adresse (cf_users) et l'id d'open_airdrop
    // //Représente l'historiques des bénéficiaires des airdrop ouverts pour ne pas distribuer deux fois un airdrop
    //voir exemple dans https://github.com/multiversx/mx-sdk-rs/blob/f2845bb197c3fa07e80fdf7681189d59b722a6d0/contracts/examples/rewards-distribution/src/rewards_distribution.rs#L367

    #[view(users)]
    #[storage_mapper("users")]
    fn users(&self) -> UnorderedSetMapper<ManagedAddress>;

    //voir https://github.com/multiversx/mx-sdk-rs/blob/a9f22d00e3b8dca5c310a87421f75a1da064c05c/contracts/examples/nft-minter/src/nft_module.rs
    // #[view(getNftTokenId)]
    // #[storage_mapper("tokens")]
    // fn tokens(&self) -> NonFungibleTokenMapper;

    //
    #[view(prompts)]
    #[storage_mapper("prompts")]
    fn prompts(&self) -> UnorderedSetMapper<Prompt<Self::Api>>;

    #[view(servers)]
    #[storage_mapper("servers")]
    fn servers(&self) -> UnorderedSetMapper<Server<Self::Api>>;

    #[view(renders)]
    #[storage_mapper("renders")]
    fn renders(&self) -> UnorderedSetMapper<Render<Self::Api>>;


    #[view(dt_start_market)]
    #[storage_mapper("dt_start_market")]
    fn dt_start_market(&self) -> SingleValueMapper<u64>;

    #[view(closed_prompt)]
    #[storage_mapper("closed_prompt")]
    fn closed_prompt(&self) -> UnorderedSetMapper<usize>;



    #[view(fee)]
    #[storage_mapper("fee")]
    fn fee(&self) -> SingleValueMapper<u64>;


    #[init]
    fn init(&self,fee:u64) {
        self.fee().set(fee);
        self.dt_start_market().set(self.blockchain().get_block_timestamp());
    }



    fn add_address(&self, addr:&ManagedAddress) -> usize {
        //Ajoute une address de user (airdropper ou claimer) sans doublon et retourne sa position
        let pos=self.users().get_index(&addr);
        if pos>0 {return pos;} //On retourne la position sans ajouter
        self.users().insert(addr.clone());
        return self.users().len();
    }



    #[endpoint]
    #[payable("*")]
    fn add_prompt(&self, text: ManagedBuffer,server_id:usize) -> usize {
        //Ajout d'un prompt sur le marché
        //voir https://github.com/multiversx/mx-sdk-rs/blob/a9f22d00e3b8dca5c310a87421f75a1da064c05c/contracts/examples/seed-nft-minter/src/nft_module.rs#L26
        //let issue_cost = self.call_value().egld_value();
        //voir https://docs.multiversx.com/tokens/nft-tokens#issuance-of-non-fungible-tokens
        //voir https://github.com/multiversx/mx-nft-collection-minter-sc/blob/c198141d2436f41b5b3afcea24c3ebbb23d3f13b/nft-minter/src/brand_creation.rs#L148

        let server=self.servers().get_by_index(server_id);


        let token_payment: EgldOrEsdtTokenPayment = self.call_value().egld_or_single_esdt();
        require!(token_payment.token_identifier==server.token,"Ne correspond pas à la monnaie du server");
        require!(token_payment.amount==server.price,"Ne correspond pas au prix du server");


        //TODO Ajouter le controle du nombre max de prompt par wallet

        require!(text.len() > 4,"La prompts doit faire au moins 5 caractères");

        let delay=self.blockchain().get_block_timestamp()-self.dt_start_market().get();

        let prompt = Prompt {
            text: text,
            owner:self.add_address(&self.blockchain().get_caller()),
            start:u32::try_from(delay).unwrap(),
            server: server_id,
        };
        self.prompts().insert(prompt);
        return self.prompts().len()
    }


    #[endpoint]
    fn cancel_prompt(&self,prompt_id:usize) -> bool {
        let caller=self.blockchain().get_caller();
        let prompt=self.prompts().get_by_index(prompt_id);
        require!(prompt.owner==self.add_address(&caller),"Vous n'êtes pas propriétaire de ce prompt");

        //Fermeture du prompt
        self.closed_prompt().insert(prompt_id);

        //Remboursement
        let server=self.servers().get_by_index(prompt.server);
        self.send().direct(
            &caller,
            &server.token, 0,
            &server.price
        );

        return true;
    }

        #[endpoint]
    fn add_server(&self,title:ManagedBuffer,model:u16,inference:u16,scale:u16,price:BigUint,token: EgldOrEsdtTokenIdentifier) -> usize {
        require!(model > 0u16,"Le modele n'est pas correct");
        require!(inference > 10,"Un minimum de 10 inférences est requis");
        require!(scale > 64,"la taille minimum est de 64 pixels");

        let server = Server {
            title:title,
            price: price,
            token: token,
            model: model,
            inference:inference,
            owner:self.add_address(&self.blockchain().get_caller()),
            scale:scale
        };
        self.servers().insert(server);
        return self.servers().len()
    }


    // #[endpoint]
    // #[payable("EGLD")]
    // fn get_render(&self,render_id:usize) {
    //     //Récupération d'un rendu par l'acheteur
    //     let payment = self.call_value().egld_value();
    //     require!(payment.to_u64().unwrap()>=self.fee().get(),"Paiement inssufisant pour les fees");
    //
    //     require!(render_id<=self.renders().len(),"Ce rendu n'existe pas");
    //     let render: Render<Self::Api> = self.renders().get_by_index(render_id);
    //
    //     require!(!self.closed_prompt().contains(&render.prompt_id),"Ce prompt est clôt");
    //     //voir https://docs.multiversx.com/developers/developer-reference/sc-api-functions/#egld_or_single_esdt
    //
    //     let prompt=self.prompts().get_by_index(render.prompt_id);
    //     let token=prompt.token;
    //     require!(prompt.owner==self.add_address(self.blockchain().get_caller()),"vous n'étes pas propriétaire");
    //
    //     //Paiement https://docs.multiversx.com/developers/developer-reference/sc-api-functions/#direct_esdt
    //     //Paiement du créateur
    //     self.send().direct_esdt(
    //         &self.users().get_by_index(render.creator),
    //         &token, 0,
    //         &render.price
    //     );
    //
    //     //On rembourse le prompteur de la différence de prix
    //     if render.price>prompt.offer {
    //         let solde=prompt.offer-render.price;
    //         self.send().direct_esdt(
    //             &self.users().get_by_index(prompt.owner),
    //             &token, 0 ,
    //             &solde
    //         );
    //     }
    //
    //     //Fermeture du prompts
    //     self.closed_prompt().insert(render.prompt_id);
    // }


    #[endpoint]
    fn add_render(&self, prompt_id:usize,url:ManagedBuffer) -> usize {
        /*
        Ajout d'un rendu avec son prix
        */
        require!(prompt_id<=self.prompts().len(),"Ce prompt n'existe pas");

        let caller=self.blockchain().get_caller();
        let creator=self.add_address(&caller);

        let prompt=self.prompts().get_by_index(prompt_id);
        let server=self.servers().get_by_index(prompt.server);
        require!(server.owner==creator,"Vous n'etes pas autoriser à repondre");

        //TODO implémenter le contrôle sur la limite au niveau du smartcontract

        let render:Render<Self::Api> = Render {
            url:url,
            prompt_id:prompt_id,
            creator:creator,
        };
        self.renders().insert(render);

        //Paiement https://docs.multiversx.com/developers/developer-reference/sc-api-functions/#direct_esdt
        //Paiement du créateur
        self.send().direct(
            &caller,
            &server.token, 0,
            &server.price
        );

        return self.renders().len()
    }


    #[only_owner]
    #[endpoint]
    fn update_fee(&self,fee:u64) {
        self.fee().set(fee);
    }

    #[only_owner]
    #[endpoint]
    fn get_fees(&self) {
        //Récupère les fees
        let caller=self.blockchain().get_caller();
        //voir https://docs.multiversx.com/developers/developer-reference/sc-api-functions/#direct_egld
        let solde=self.blockchain().get_balance(&self.blockchain().get_sc_address());
        self.send().direct_egld(&caller,&solde);
    }



}
