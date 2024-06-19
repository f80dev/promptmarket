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
    pub offer:u64,                      //Montant proposé
    pub token:TokenIdentifier<M>,
    pub model:u8,                       //Modele demandé
    pub inference:u16,                  //Inférence
    pub scale:u16,                      //résolution
    pub owner:usize                     //Propriétaire
}

#[derive(TypeAbi, TopEncode, TopDecode,ManagedVecItem,NestedEncode,NestedDecode)]
pub struct Render<M: ManagedTypeApi> {
    pub prompt_id:usize,
    pub url:ManagedBuffer<M>,
    pub creator:usize,
    pub price:u64                   //Prix du rendering
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

    #[view(renders)]
    #[storage_mapper("renders")]
    fn renders(&self) -> UnorderedSetMapper<Render<Self::Api>>;


    #[view(fee)]
    #[storage_mapper("fee")]
    fn fee(&self) -> SingleValueMapper<u64>;

    #[view(maxprompt)]
    #[storage_mapper("maxprompt")]
    fn maxprompt(&self) -> SingleValueMapper<u8>;

    #[init]
    fn init(&self,fee:u64,maxprompt:u8) {
        self.fee().set(fee);
        self.maxprompt().set(maxprompt);
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



    fn add_user(&self,addr:ManagedAddress) -> usize {
        //Ajoute une address de user (airdropper ou claimer) sans doublon et retourne sa position
        let pos=self.users().get_index(&addr);
        if pos>0 {return pos;} //On retourne la position sans ajouter
        self.users().insert(addr);
        return self.users().len();
    }



    #[endpoint]
    #[payable("*")]
    fn add_prompt(&self, text: ManagedBuffer,model:u8,inference:u16,scale:u16) -> usize {
        //Ajout d'un prompt sur le marché
        //voir https://github.com/multiversx/mx-sdk-rs/blob/a9f22d00e3b8dca5c310a87421f75a1da064c05c/contracts/examples/seed-nft-minter/src/nft_module.rs#L26
        //let issue_cost = self.call_value().egld_value();
        //voir https://docs.multiversx.com/tokens/nft-tokens#issuance-of-non-fungible-tokens
        //voir https://github.com/multiversx/mx-nft-collection-minter-sc/blob/c198141d2436f41b5b3afcea24c3ebbb23d3f13b/nft-minter/src/brand_creation.rs#L148

        let token_payment: EgldOrEsdtTokenPayment = self.call_value().egld_or_single_esdt();
        //require!(token_payment.is_esdt(),"Vous ne pouvez pas proposer des egld");

        //TODO Ajouter le controle du nombre max de prompt

        require!(model > 0u8,"Le modele n'est pas correct");
        require!(inference > 10,"Un minimum de 10 inférences est requis");
        require!(scale > 64,"la taille minimum est de 64 pixels");
        require!(text.len() > 4,"La prompts doit faire au moins 5 caractères");

        let prompt = Prompt {
            text: text,
            offer:token_payment.amount.to_u64().unwrap(),
            token:token_payment.unwrap_esdt().token_identifier,
            model:model,
            inference:inference,
            scale:scale,
            owner:self.add_user(self.blockchain().get_caller())
        };
        self.prompts().insert(prompt);
        return self.prompts().len()
    }


    #[endpoint]
    #[payable("EGLD")]
    fn get_render(&self,render_id:usize) {
        //Récupération d'un rendu par l'acheteur
        let payment = self.call_value().egld_value();
        require!(payment.to_u64().unwrap()>=self.fee().get(),"Paiement inssufisant pour les fees");

        require!(render_id<=self.renders().len(),"Ce rendu n'existe pas");
        let render: Render<Self::Api> = self.renders().get_by_index(render_id);
        //voir https://docs.multiversx.com/developers/developer-reference/sc-api-functions/#egld_or_single_esdt

        let prompt=self.prompts().get_by_index(render.prompt_id);
        let token=prompt.token;
        require!(prompt.owner==self.add_user(self.blockchain().get_caller()),"vous n'étes pas propriétaire");

        //Paiement https://docs.multiversx.com/developers/developer-reference/sc-api-functions/#direct_esdt
        //Paiement du créateur
        self.send().direct_esdt(
            &self.users().get_by_index(render.creator),
            &token, 0,
            &BigUint::from(render.price)
        );

        //On rembourse le prompteur de la différence de prix
        if prompt.offer-render.price>0 {
            self.send().direct_esdt(
                &self.users().get_by_index(prompt.owner),
                &token, 0 ,
                &BigUint::from(prompt.offer-render.price)
            );
        }
    }


    #[endpoint]
    fn add_render(&self, prompt_id:usize,url:ManagedBuffer,price:u64) -> usize {
        /*
        Ajout d'un rendu avec son prix
        */
        require!(prompt_id<=self.prompts().len(),"Ce prompt n'existe pas");

        let prompt=self.prompts().get_by_index(prompt_id);
        require!(prompt.offer>=price,"Proposition au dela du budget du demandeur");

        let render:Render<Self::Api> = Render {
            url:url,
            prompt_id:prompt_id,
            creator:self.add_user(self.blockchain().get_caller()),
            price:price
        };
        self.renders().insert(render);
        return self.renders().len()
    }

}
